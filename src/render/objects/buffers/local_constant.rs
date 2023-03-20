use std::ptr;

use ash::vk;

use crate::{
  render::{
    models::Models,
    objects::{
      command_buffer_pools::CopyBufferOperation, CommandBufferPools, MatrixInstance, 
      QueueFamilyIndices, Queues, Vertex,
    },
  },
  static_scene::StaticScene,
};

use super::{
  create_travel_buffers, HOST_MEMORY_PROPERTY_FLAGS, INDEX_DST_USAGE, INDEX_SRC_USAGE,
  LOCAL_MEMORY_PROPERTY_FLAGS, STORAGE_SRC_USAGE, STORAGE_USAGE, VERTEX_DST_USAGE,
  VERTEX_SRC_USAGE,
};

pub struct DrawProperties {
  index_count: u32,
  instance_count: u32,
  first_index: u32,
  vertex_offset: i32,
  first_instance: u32,
}

// holds model information and static objects
// contains local memory, buffers and their offsets (in memory)
pub struct LocalConstantMemory {
  memory: vk::DeviceMemory,
  vertex: vk::Buffer,
  vertex_offset: u64,
  index: vk::Buffer,
  index_offset: u64,
  instance: vk::Buffer,
  instance_offset: u64,
  draw_props: Vec<DrawProperties>,
}

macro_rules! copy_into_mem {
  ($device:expr, $mem:expr, $mem_offset:expr, $data:expr, $t:ty) => {
    let l = $data.len();
    let data_ptr = $device
      .map_memory(
        $mem,
        $mem_offset,
        (std::mem::size_of::<$t>() * l) as u64,
        vk::MemoryMapFlags::empty(),
      )
      .expect("Failed to map memory") as *mut $t;
    data_ptr.copy_from_nonoverlapping($data.as_ptr(), l);
    $device.unmap_memory($mem);
  };
}

impl LocalConstantMemory {
  pub fn new(
    device: &ash::Device,
    memory_properties: vk::PhysicalDeviceMemoryProperties,
    queue_families: &QueueFamilyIndices,
    queues: &Queues,
    command_pools: &mut CommandBufferPools,
  ) -> Self {
    let models = Models::load(); // vertices and indices
    let scene = StaticScene::load(); // information about static (constant location, etc.) objects

    // create vulkan buffers
    let vertex_size = (std::mem::size_of::<Vertex>() * models.vertices.len()) as u64;
    let index_size = (std::mem::size_of::<u16>() * models.indices.len()) as u64;
    let instance_size = (std::mem::size_of::<MatrixInstance>() * scene.total_obj_count) as u64;
    let vk_buffers = create_travel_buffers(
      device,
      queue_families,
      vec![
        (vertex_size, VERTEX_SRC_USAGE, VERTEX_DST_USAGE),
        (index_size, INDEX_SRC_USAGE, INDEX_DST_USAGE),
        (instance_size, STORAGE_SRC_USAGE, STORAGE_USAGE),
      ],
    );

    // allocate vulkan buffers
    let src_buffers = vk_buffers
      .iter()
      .map(|&(size, source, _)| (size, source))
      .collect();
    let dst_buffers = vk_buffers
      .into_iter()
      .map(|(size, _, dst)| (size, dst))
      .collect();
    let (src_memory, src_size, src_offsets) = super::allocate_vk_buffers(
      device,
      &src_buffers,
      memory_properties,
      HOST_MEMORY_PROPERTY_FLAGS,
    );
    let (dst_memory, dst_size, dst_offsets) = super::allocate_vk_buffers(
      device,
      &dst_buffers,
      memory_properties,
      LOCAL_MEMORY_PROPERTY_FLAGS,
    );
    if src_size != dst_size {
      // I don't know if this could happen or not
      panic!("Costant buffer creation: source size is not equal to destination size");
    }

    let (vec, inst_model_indices) = scene.objects();
    let (inst_objs, inst_parts) = vec.deconstruct();
    let draw_props = inst_model_indices
      .into_iter()
      .zip(inst_parts.iter())
      .map(|(model_i, part)| {
        let model_inst = (&models)[model_i];
        let (inst_size, inst_offset) = part.deconstruct();
        DrawProperties {
          index_count: model_inst.indices.len() as u32,
          first_index: model_inst.index_offset as u32,
          vertex_offset: model_inst.vertex_offset as i32,
          instance_count: inst_size as u32,
          first_instance: inst_offset as u32,
        }
      })
      .collect();

    let instance_data: Vec<MatrixInstance> = inst_objs
      .into_iter()
      .map(|ren| MatrixInstance::new(ren.obj().model().clone()))
      .collect();

    // copy data into the source buffers (host memory)
    // because the source buffer may be not continuous, data is copied buffer by buffer
    // may need optimizations
    unsafe {
      copy_into_mem!(device, src_memory, src_offsets[0], models.vertices, Vertex);
      copy_into_mem!(device, src_memory, src_offsets[1], models.indices, u16);
      copy_into_mem!(
        device,
        src_memory,
        src_offsets[2],
        instance_data,
        MatrixInstance
      );
    }

    // copy data from source buffers into destination and deallocate the source memory
    // instance destination buffer will be used as source for computations
    unsafe {
      let operations: Vec<CopyBufferOperation> = src_buffers
        .iter()
        .zip(dst_buffers.iter())
        .map(|((src_size, src), (dst_size, dst))| CopyBufferOperation {
          source_buffer: *src,
          dest_buffer: *dst,
          copy_regions: vec![vk::BufferCopy {
            src_offset: 0,
            dst_offset: 0,
            size: *src_size,
          }],
        })
        .collect();
      command_pools
        .transfer
        .record_copy_buffers(device, &operations);
    }

    // fence copying has finished
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
      for (_, mut src) in src_buffers {
        device.destroy_buffer(src, None);
      }
      device.free_memory(src_memory, None);
    }

    let (_, vertex_parts) = models.vertices.deconstruct();
    let (_, index_parts) = models.indices.deconstruct();
    let mut dsts_iter = dst_buffers.into_iter();
    let vertex = dsts_iter.next().unwrap().1;
    let index = dsts_iter.next().unwrap().1;
    let instance = dsts_iter.next().unwrap().1;

    Self {
      memory: dst_memory,
      vertex,
      vertex_offset: dst_offsets[0],
      index,
      index_offset: dst_offsets[1],
      instance,
      instance_offset: dst_offsets[2],
      draw_props,
    }
  }

  pub fn vertex(&self) -> vk::Buffer {
    self.vertex
  }

  pub fn index(&self) -> vk::Buffer {
    self.index
  }

  pub unsafe fn destroy_self(&mut self, device: &ash::Device) {
    device.destroy_buffer(self.vertex, None);
    device.destroy_buffer(self.index, None);
    device.destroy_buffer(self.instance, None);
    device.free_memory(self.memory, None);
  }
}
