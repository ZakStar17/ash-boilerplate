use std::ptr;

use ash::vk;
use log::warn;

use crate::render::objects::{
  command_buffer_pools::CopyBufferOperation, CommandBufferPools, QueueFamilyIndices, Queues, Vertex,
};

use super::{
  create_travel_buffers, SizedBuffer, HOST_MEMORY_PROPERTY_FLAGS, INDEX_DST_USAGE, INDEX_SRC_USAGE,
  LOCAL_MEMORY_PROPERTY_FLAGS, VERTEX_DST_USAGE, VERTEX_SRC_USAGE,
};

pub struct LocalConstantMemory {
  memory: vk::DeviceMemory,
  vertex: SizedBuffer,
  index: SizedBuffer,
}

impl LocalConstantMemory {
  pub fn new(
    device: &ash::Device,
    memory_properties: vk::PhysicalDeviceMemoryProperties,
    queue_families: &QueueFamilyIndices,
    queues: &Queues,
    command_pools: &mut CommandBufferPools,
    vertices: &Vec<Vertex>,
    indices: &Vec<u16>,
  ) -> Self {
    let vertex_size = (std::mem::size_of::<Vertex>() * vertices.len()) as u64;
    let index_size = (std::mem::size_of::<u16>() * indices.len()) as u64;

    let vk_buffers = create_travel_buffers(
      device,
      queue_families,
      vec![
        (vertex_size, VERTEX_SRC_USAGE, VERTEX_DST_USAGE),
        (index_size, INDEX_SRC_USAGE, INDEX_DST_USAGE),
      ],
    );

    let source_vk_buffers = vk_buffers
      .iter()
      .map(|&(size, source, _)| (size, source))
      .collect();
    let dst_vk_buffers = vk_buffers
      .into_iter()
      .map(|(size, _, dst)| (size, dst))
      .collect();
    let (source_memory, source_size, sources) = SizedBuffer::allocate_vk_buffers(
      device,
      source_vk_buffers,
      memory_properties,
      HOST_MEMORY_PROPERTY_FLAGS,
    );

    let (dest_memory, dest_size, dests) = SizedBuffer::allocate_vk_buffers(
      device,
      dst_vk_buffers,
      memory_properties,
      LOCAL_MEMORY_PROPERTY_FLAGS,
    );
    if source_size != dest_size {
      warn!("Costant buffer creation: source size is not equal to destination size");
    }
    for (source, dest) in sources.iter().zip(dests.iter()) {
      assert_eq!(source.size, dest.size);
    }

    // TODO: copy everything at once
    unsafe {
      let data_ptr = device
        .map_memory(
          source_memory,
          sources[0].offset,
          vertex_size,
          vk::MemoryMapFlags::empty(),
        )
        .expect("Failed to map memory") as *mut Vertex;
      data_ptr.copy_from_nonoverlapping(vertices.as_ptr(), vertices.len());
      device.unmap_memory(source_memory);

      let data_ptr = device
        .map_memory(
          source_memory,
          sources[1].offset,
          index_size,
          vk::MemoryMapFlags::empty(),
        )
        .expect("Failed to map memory") as *mut u16;
      data_ptr.copy_from_nonoverlapping(indices.as_ptr(), indices.len());
      device.unmap_memory(source_memory);
    }

    unsafe {
      let operations: Vec<CopyBufferOperation> = sources
        .iter()
        .zip(dests.iter())
        .map(|(source, dest)| CopyBufferOperation {
          source_buffer: source.inner(),
          dest_buffer: dest.inner(),
          copy_regions: vec![vk::BufferCopy {
            src_offset: 0,
            dst_offset: 0,
            size: source.size,
          }],
        })
        .collect();
      command_pools
        .transfer
        .record_copy_buffers(device, &operations);
    }

    let finished = {
      let create_info = vk::FenceCreateInfo {
        s_type: vk::StructureType::FENCE_CREATE_INFO,
        p_next: ptr::null(),
        flags: vk::FenceCreateFlags::empty(),
      };
      unsafe {
        device
          .create_fence(&create_info, None)
          .expect("Failed to create fence")
      }
    };

    let command_buffer = command_pools.transfer.copy_buffer;
    let submit_infos = [vk::SubmitInfo {
      s_type: vk::StructureType::SUBMIT_INFO,
      p_next: ptr::null(),
      wait_semaphore_count: 0,
      p_wait_semaphores: ptr::null(),
      p_wait_dst_stage_mask: ptr::null(),
      command_buffer_count: 1,
      p_command_buffers: &command_buffer,
      signal_semaphore_count: 0,
      p_signal_semaphores: ptr::null(),
    }];

    unsafe {
      device
        .queue_submit(
          if let Some(transfer) = queues.transfer {
            transfer
          } else {
            queues.graphics
          },
          &submit_infos,
          finished,
        )
        .expect("Failed submit queue for copying initial buffers");
      device.wait_for_fences(&[finished], true, u64::MAX).unwrap();

      device.destroy_fence(finished, None);
      for mut source in sources.into_iter() {
        source.destroy_self(device);
      }
      device.free_memory(source_memory, None);
    }

    let mut dests_iter = dests.into_iter();
    let vertex = dests_iter.next().unwrap();
    let index = dests_iter.next().unwrap();

    Self {
      memory: dest_memory,
      vertex,
      index,
    }
  }

  pub fn vertex(&self) -> vk::Buffer {
    self.vertex.inner()
  }

  pub fn index(&self) -> vk::Buffer {
    self.index.inner()
  }

  pub unsafe fn destroy_self(&mut self, device: &ash::Device) {
    self.vertex.destroy_self(device);
    self.index.destroy_self(device);
    device.free_memory(self.memory, None);
  }
}
