use std::{mem::MaybeUninit, ptr};

use ash::vk;

use crate::render::{objects::Buffers, sync::FRAMES_IN_FLIGHT, utility, SquareInstance};

use super::layouts::DescriptorSetLayouts;

pub struct DescriptorSetPool {
  pool: vk::DescriptorPool,
  pub instance_compute: [vk::DescriptorSet; FRAMES_IN_FLIGHT],
}

impl DescriptorSetPool {
  pub fn new(device: &ash::Device, layouts: &DescriptorSetLayouts) -> Self {
    let layout_instance_compute = utility::iter_into_array!(
      std::iter::repeat(layouts.instance_compute.layout),
      FRAMES_IN_FLIGHT
    );

    let descriptor_count = (layouts.instance_compute.descriptor_count * FRAMES_IN_FLIGHT) as u32;
    let sizes = [vk::DescriptorPoolSize {
      ty: vk::DescriptorType::STORAGE_BUFFER,
      descriptor_count,
    }];
    let pool_create_info = vk::DescriptorPoolCreateInfo {
      s_type: vk::StructureType::DESCRIPTOR_POOL_CREATE_INFO,
      p_next: ptr::null(),
      pool_size_count: sizes.len() as u32,
      p_pool_sizes: sizes.as_ptr(),
      max_sets: 2, // instance_compute
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
      descriptor_set_count: layout_instance_compute.len() as u32,
      p_set_layouts: layout_instance_compute.as_ptr(),
    };
    let descriptor_sets = unsafe {
      device
        .allocate_descriptor_sets(&allocate_info)
        .expect("Failed to allocate descriptor sets")
    };

    let instance_compute = utility::vec_to_array!(descriptor_sets, FRAMES_IN_FLIGHT);
    Self {
      pool,
      instance_compute,
    }
  }

  pub fn update_instance_compute(
    &mut self,
    i: usize,
    device: &ash::Device,
    buffers: &Buffers,
    instance_count: u64,
  ) {
    let buffer_info_source = vk::DescriptorBufferInfo {
      buffer: buffers.instance_source(i),
      offset: 0,
      range: std::mem::size_of::<SquareInstance>() as u64 * instance_count,
    };
    let buffer_info_dest = vk::DescriptorBufferInfo {
      buffer: buffers.instance_dest(i),
      offset: 0,
      range: std::mem::size_of::<SquareInstance>() as u64 * instance_count,
    };

    let source = vk::WriteDescriptorSet {
      s_type: vk::StructureType::WRITE_DESCRIPTOR_SET,
      p_next: ptr::null(),
      dst_set: self.instance_compute[i],
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
