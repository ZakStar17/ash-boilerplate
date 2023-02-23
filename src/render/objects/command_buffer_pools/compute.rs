use std::{mem::MaybeUninit, ops::BitOr, ptr};

use ash::vk;

use crate::render::{
  objects::{CameraPos, DescriptorSets, Pipelines, QueueFamilyIndices},
  sync::FRAMES_IN_FLIGHT,
  utility,
};

pub struct ComputeCommandBufferPool {
  pool: vk::CommandPool,
  pub instance: [vk::CommandBuffer; FRAMES_IN_FLIGHT],
}

impl ComputeCommandBufferPool {
  pub fn create(device: &ash::Device, queue_families: &QueueFamilyIndices) -> Self {
    let flags =
      vk::CommandPoolCreateFlags::TRANSIENT.bitor(vk::CommandPoolCreateFlags::RESET_COMMAND_BUFFER);
    let pool = super::create_command_pool(device, flags, queue_families.compute);

    let mut buffer_iter =
      super::create_command_buffers(device, pool, FRAMES_IN_FLIGHT as u32).into_iter();
    let instance = utility::iter_into_array!(buffer_iter, FRAMES_IN_FLIGHT);

    Self { pool, instance }
  }

  pub unsafe fn record_instance(
    &mut self,
    i: usize,
    device: &ash::Device,
    pipelines: &Pipelines,
    descriptor_sets: &DescriptorSets,
    instance_count: u32,
    camera_pos: &CameraPos,
  ) {
    let cb = self.instance[i];
    device
      .reset_command_buffer(cb, vk::CommandBufferResetFlags::empty())
      .expect("Failed to reset instance compute command buffer");

    let command_buffer_begin_info = vk::CommandBufferBeginInfo {
      s_type: vk::StructureType::COMMAND_BUFFER_BEGIN_INFO,
      p_next: ptr::null(),
      p_inheritance_info: ptr::null(),
      flags: vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT,
    };

    device
      .begin_command_buffer(cb, &command_buffer_begin_info)
      .expect("Failed to start recording compute instance command buffer");

    let camera_pos_bytes = utility::any_as_u8_slice(camera_pos);
    device.cmd_push_constants(
      cb,
      pipelines.compute.layout,
      vk::ShaderStageFlags::COMPUTE,
      0,
      camera_pos_bytes,
    );

    let sets = [descriptor_sets.pool.instance_compute[i]];
    device.cmd_bind_descriptor_sets(
      cb,
      vk::PipelineBindPoint::COMPUTE,
      pipelines.compute.layout,
      0,
      &sets,
      &[],
    );
    device.cmd_bind_pipeline(
      cb,
      vk::PipelineBindPoint::COMPUTE,
      pipelines.compute.instance,
    );

    device.cmd_dispatch(cb, instance_count / 64 + 1, 1, 1);

    device
      .end_command_buffer(cb)
      .expect("Failed to finish recording command buffer")
  }

  pub unsafe fn destroy_self(&mut self, device: &ash::Device) {
    device.destroy_command_pool(self.pool, None);
  }
}
