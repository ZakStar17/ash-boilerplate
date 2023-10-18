use std::{mem::size_of, ptr};

use ash::vk;

use crate::render::{objects::Buffers, sync::FRAMES_IN_FLIGHT, MatrixInstance};

use super::layouts::DescriptorSetLayouts;

pub struct DescriptorSetPool {
  pool: vk::DescriptorPool,
  pub inst_static: [vk::DescriptorSet; FRAMES_IN_FLIGHT],
  pub inst_dyn: [vk::DescriptorSet; FRAMES_IN_FLIGHT],
}

impl DescriptorSetPool {
  pub fn new(device: &ash::Device, layouts: &DescriptorSetLayouts) -> Self {
    // this all needs some sort of restructuring
    // 2 for each set array
    let layouts_arr = [layouts.inst.layout; FRAMES_IN_FLIGHT * 2];

    // 2 for each descriptor array
    let descriptor_count = (layouts.inst.descriptor_count * FRAMES_IN_FLIGHT * 2) as u32;
    let sizes = [vk::DescriptorPoolSize {
      ty: vk::DescriptorType::STORAGE_BUFFER,
      descriptor_count,
    }];
    let pool_create_info = vk::DescriptorPoolCreateInfo {
      s_type: vk::StructureType::DESCRIPTOR_POOL_CREATE_INFO,
      p_next: ptr::null(),
      pool_size_count: sizes.len() as u32,
      p_pool_sizes: sizes.as_ptr(),
      max_sets: layouts_arr.len() as u32,
      flags: vk::DescriptorPoolCreateFlags::empty(),
    };
    let pool = unsafe {
      device
        .create_descriptor_pool(&pool_create_info, None)
        .expect("Failed to create descriptor pool")
    };

    let allocate_info = vk::DescriptorSetAllocateInfo {
      s_type: vk::StructureType::DESCRIPTOR_SET_ALLOCATE_INFO,
      p_next: ptr::null(),
      descriptor_pool: pool,
      descriptor_set_count: layouts_arr.len() as u32,
      p_set_layouts: layouts_arr.as_ptr(),
    };
    let descriptor_sets = unsafe {
      device
        .allocate_descriptor_sets(&allocate_info)
        .expect("Failed to allocate descriptor sets")
    };

    let mut iter = descriptor_sets.into_iter();
    let inst_static = iter.next_chunk().unwrap();
    let inst_dyn = iter.next_chunk().unwrap();
    Self {
      pool,
      inst_static,
      inst_dyn,
    }
  }

  pub fn update_all_inst_static(&mut self, device: &ash::Device, buffers: &Buffers) {
    for i in 0..(self.inst_static.len()) {
      self.update_inst_static(i, device, buffers);
    }
  }

  pub fn update_inst_static(&mut self, i: usize, device: &ash::Device, buffers: &Buffers) {
    // [static] [dyn] -> [static dyn]
    let static_size = buffers.local_constant.inst.size;
    let buffer_info_source = vk::DescriptorBufferInfo {
      buffer: buffers.local_constant.inst.buffer,
      offset: 0,
      range: static_size,
    };
    let buffer_info_dest = vk::DescriptorBufferInfo {
      buffer: buffers.local.inst[i].0,
      offset: 0,
      range: static_size,
    };

    let source = vk::WriteDescriptorSet {
      s_type: vk::StructureType::WRITE_DESCRIPTOR_SET,
      p_next: ptr::null(),
      dst_set: self.inst_static[i],
      dst_binding: 0,
      dst_array_element: 0,
      descriptor_count: 1,
      descriptor_type: vk::DescriptorType::STORAGE_BUFFER,
      p_buffer_info: &buffer_info_source,
      p_image_info: ptr::null(),
      p_texel_buffer_view: ptr::null(),
    };
    let mut dest = source.clone();
    dest.dst_binding = 1;
    dest.p_buffer_info = &buffer_info_dest;

    let writes = [source, dest];
    let copies = [];
    unsafe {
      device.update_descriptor_sets(&writes, &copies);
    }
  }

  pub fn update_inst_dyn(
    &mut self,
    i: usize,
    device: &ash::Device,
    buffers: &Buffers,
    dyn_inst_count: u64,
  ) {
    // [static] [dyn] -> [static dyn]
    let static_size = buffers.local_constant.inst.size;
    let dyn_size = size_of::<MatrixInstance>() as u64 * dyn_inst_count;
    let buffer_info_source = vk::DescriptorBufferInfo {
      buffer: buffers.host_writable.inst[i].0,
      offset: 0,
      range: dyn_size,
    };
    let buffer_info_dest = vk::DescriptorBufferInfo {
      buffer: buffers.local.inst[i].0,
      offset: static_size,
      range: dyn_size,
    };

    let source = vk::WriteDescriptorSet {
      s_type: vk::StructureType::WRITE_DESCRIPTOR_SET,
      p_next: ptr::null(),
      dst_set: self.inst_dyn[i],
      dst_binding: 0,
      dst_array_element: 0,
      descriptor_count: 1,
      descriptor_type: vk::DescriptorType::STORAGE_BUFFER,
      p_buffer_info: &buffer_info_source,
      p_image_info: ptr::null(),
      p_texel_buffer_view: ptr::null(),
    };
    let mut dest = source.clone();
    dest.dst_binding = 1;
    dest.p_buffer_info = &buffer_info_dest;

    let writes = [source, dest];
    let copies = [];
    unsafe {
      device.update_descriptor_sets(&writes, &copies);
    }
  }

  pub unsafe fn destroy_self(&mut self, device: &ash::Device) {
    device.destroy_descriptor_pool(self.pool, None);
  }
}
