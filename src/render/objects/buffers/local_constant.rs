use std::{mem::size_of, ptr};

use ash::vk;
use log::debug;

use crate::{
  render::{
    models::Models,
    objects::{
      command_buffer_pools::CopyBufferOperation, ColorVertex, CommandBufferPools, InstProperties,
      MatrixInstance, QueueFamilyIndices, Queues, TexVertex,
    },
  },
  static_scene::StaticScene,
  structures::Partition,
};

use super::{
  create_travel_buffers, HOST_MEMORY_PROPERTY_FLAGS, INDEX_DST_USAGE, LOCAL_MEMORY_PROPERTY_FLAGS,
  STORAGE_USAGE, VERTEX_DST_USAGE,
};

pub struct Vertex {
  pub buffer: vk::Buffer,
  pub mem_offset: u64,
  // internal data offsets
  pub color_offset: u64,
  pub tex_offset: u64,
}

pub struct Index {
  pub buffer: vk::Buffer,
  pub mem_offset: u64,
  // internal data offsets
  pub color_offset: u64,
  pub tex_offset: u64,
}

pub struct Inst {
  pub buffer: vk::Buffer,
  pub mem_offset: u64,
  pub size: u64,
  pub color_part: Partition<u64>,
  pub color_props: Vec<InstProperties>,
  pub tex_part: Partition<u64>,
  pub tex_props: Vec<InstProperties>,
}

impl Inst {
  pub fn count(&self) -> u64 {
    (self.color_props.len() + self.tex_props.len()) as u64
  }
}

// holds model information and static objects
// contains local memory, buffers and their offsets (in memory)
// ------------ Memory Layout ------------
// [  Vertex  ] [  Index   ] [   Instance   ]
// [Color][Tex] [Color][Tex] [Color ][ Tex  ]
//                           [Blocks][Blocks]
pub struct LocalConstantMemory {
  memory: vk::DeviceMemory,
  pub vertex: Vertex,
  pub index: Index,
  pub inst: Inst,
}

impl LocalConstantMemory {
  pub fn new(
    device: &ash::Device,
    memory_properties: vk::PhysicalDeviceMemoryProperties,
    queue_families: &QueueFamilyIndices,
    queues: &Queues,
    command_pools: &mut CommandBufferPools,
    models: &Models,
  ) -> Self {
    let scene = StaticScene::load(); // information about static (constant location, etc.) objects

    // create vulkan buffers
    let color_vertex_size = (size_of::<ColorVertex>() * models.color.vertices.len()) as u64;
    let tex_vertex_size = (size_of::<TexVertex>() * models.tex.vertices.len()) as u64;
    let vertex_size = color_vertex_size + tex_vertex_size;

    let color_index_size = (size_of::<u16>() * models.color.indices.len()) as u64;
    let tex_index_size = (size_of::<u16>() * models.tex.indices.len()) as u64;
    let index_size = color_index_size + tex_index_size;

    let color_inst_size = (size_of::<MatrixInstance>() * scene.color_obj_count) as u64;
    let tex_inst_size = (size_of::<MatrixInstance>() * scene.tex_obj_count) as u64;
    let inst_size = color_inst_size + tex_inst_size;
    let total_min_allocation_size = vertex_size + index_size + inst_size;

    let vk_buffers = create_travel_buffers(
      device,
      queue_families,
      vec![
        (
          vertex_size,
          vk::BufferUsageFlags::TRANSFER_SRC,
          VERTEX_DST_USAGE,
        ),
        (
          index_size,
          vk::BufferUsageFlags::TRANSFER_SRC,
          INDEX_DST_USAGE,
        ),
        (inst_size, vk::BufferUsageFlags::TRANSFER_SRC, STORAGE_USAGE),
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

    debug!(
      "Buffer internal alignment wasted space: {} bytes",
      dst_size - total_min_allocation_size
    );

    let (vec, (inst_color_is, inst_tex_is)) = scene.objects();
    let (inst_objs, inst_parts) = vec.deconstruct();
    let inst_color_props = inst_color_is
      .into_iter()
      .zip(inst_parts[0..inst_color_is.len()].iter())
      .map(|(model_i, part)| InstProperties {
        model_i: model_i.0,
        inst_count: part.size as u32,
        inst_offset: part.offset as u32,
      })
      .collect();
    let inst_tex_props = inst_tex_is
      .into_iter()
      .zip(inst_parts[inst_color_is.len()..inst_tex_is.len()].iter())
      .map(|(model_i, part)| InstProperties {
        model_i: model_i.0,
        inst_count: part.size as u32,
        inst_offset: part.offset as u32,
      })
      .collect();

    let inst_data: Vec<MatrixInstance> = inst_objs
      .into_iter()
      .map(|obj| MatrixInstance::new(obj.ren().model().clone()))
      .collect();

    // Some fields are repeated but whatever
    let vertex_src = Vertex {
      buffer: src_buffers[0].1,
      mem_offset: src_offsets[0],
      color_offset: 0,
      tex_offset: color_vertex_size,
    };
    let vertex_dst = Vertex {
      buffer: dst_buffers[0].1,
      mem_offset: dst_offsets[0],
      color_offset: 0,
      tex_offset: color_vertex_size,
    };
    let index_src = Index {
      buffer: src_buffers[1].1,
      mem_offset: src_offsets[1],
      color_offset: 0,
      tex_offset: color_index_size,
    };
    let index_dst = Index {
      buffer: dst_buffers[1].1,
      mem_offset: dst_offsets[1],
      color_offset: 0,
      tex_offset: color_index_size,
    };
    let inst_src_offset = src_offsets[2];
    let inst_dst = Inst {
      buffer: dst_buffers[2].1,
      mem_offset: dst_offsets[2],
      size: inst_size,
      color_part: Partition {
        size: color_inst_size,
        offset: 0,
      },
      tex_part: Partition {
        size: tex_inst_size,
        offset: color_inst_size,
      },
      color_props: inst_color_props,
      tex_props: inst_tex_props,
    };

    // copy data into the source buffers (host memory)
    // because the source buffer is internally aligned and probably not continuous,
    //    data is copied separately
    println!("Starting buffer copy");
    unsafe {
      let mem_ptr = device
        .map_memory(src_memory, 0, src_size, vk::MemoryMapFlags::empty())
        .expect("Failed to map constant source memory") as *mut u8;

      ptr::copy_nonoverlapping(
        models.color.vertices.as_ptr() as *const u8,
        mem_ptr.byte_add((vertex_src.mem_offset + vertex_src.color_offset) as usize) as *mut u8,
        color_vertex_size as usize,
      );
      ptr::copy_nonoverlapping(
        models.tex.indices.as_ptr() as *const u8,
        mem_ptr.byte_add((vertex_src.mem_offset + vertex_src.tex_offset) as usize) as *mut u8,
        tex_vertex_size as usize,
      );

      ptr::copy_nonoverlapping(
        models.color.indices.as_ptr() as *const u8,
        mem_ptr.byte_add((index_src.mem_offset + index_src.color_offset) as usize) as *mut u8,
        color_index_size as usize,
      );
      ptr::copy_nonoverlapping(
        models.tex.indices.as_ptr() as *const u8,
        mem_ptr.byte_add((index_src.mem_offset + index_src.tex_offset) as usize) as *mut u8,
        tex_index_size as usize,
      );

      ptr::copy_nonoverlapping(
        inst_data.as_ptr() as *const u8,
        mem_ptr.byte_add(inst_src_offset as usize) as *mut u8,
        inst_size as usize,
      );

      device.unmap_memory(src_memory);
    }

    // copy data from source buffers into destination and deallocate the source memory
    // instance destination buffer will be used as source for computations
    unsafe {
      let operations: Vec<CopyBufferOperation> = src_buffers
        .iter()
        .zip(dst_buffers.iter())
        .map(|((src_size, src), (_dst_size, dst))| CopyBufferOperation {
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
      for (_, src) in src_buffers {
        device.destroy_buffer(src, None);
      }
      device.free_memory(src_memory, None);
    }

    Self {
      memory: dst_memory,
      vertex: vertex_dst,
      index: index_dst,
      inst: inst_dst,
    }
  }

  pub unsafe fn destroy_self(&mut self, device: &ash::Device) {
    device.destroy_buffer(self.vertex.buffer, None);
    device.destroy_buffer(self.index.buffer, None);
    device.destroy_buffer(self.inst.buffer, None);
    device.free_memory(self.memory, None);
  }
}
