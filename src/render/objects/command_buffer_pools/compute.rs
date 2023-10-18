use std::{mem::MaybeUninit, ops::BitOr, ptr};

use ash::vk;
use cgmath::Matrix4;

use crate::render::{
  objects::{Buffers, DescriptorSets, Pipelines, QueueFamilyIndices},
  sync::FRAMES_IN_FLIGHT,
  utility,
};

pub struct ComputeCommandBufferPool {
  pool: vk::CommandPool,
  pub inst_static: [vk::CommandBuffer; FRAMES_IN_FLIGHT],
  pub inst_dyn: [vk::CommandBuffer; FRAMES_IN_FLIGHT],
}

impl ComputeCommandBufferPool {
  pub fn create(device: &ash::Device, queue_families: &QueueFamilyIndices) -> Self {
    let flags =
      vk::CommandPoolCreateFlags::TRANSIENT.bitor(vk::CommandPoolCreateFlags::RESET_COMMAND_BUFFER);
    let pool = super::create_command_pool(device, flags, queue_families.compute);

    let mut inst_static_iter =
      super::create_command_buffers(device, pool, FRAMES_IN_FLIGHT as u32).into_iter();
    let inst_static = utility::iter_into_array!(inst_static_iter, FRAMES_IN_FLIGHT);

    let mut inst_static_dyn =
      super::create_command_buffers(device, pool, FRAMES_IN_FLIGHT as u32).into_iter();
    let inst_dyn = utility::iter_into_array!(inst_static_dyn, FRAMES_IN_FLIGHT);

    Self {
      pool,
      inst_static,
      inst_dyn,
    }
  }

  pub unsafe fn record_inst_static(
    &mut self,
    i: usize,
    device: &ash::Device,
    pipelines: &Pipelines,
    buffers: &Buffers,
    descriptor_sets: &DescriptorSets,
    projection_view: &Matrix4<f32>,
  ) {
    let cb = self.inst_static[i];
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

    let projection_view_bytes = utility::any_as_u8_slice(projection_view);
    device.cmd_push_constants(
      cb,
      pipelines.compute.layout,
      vk::ShaderStageFlags::COMPUTE,
      0,
      projection_view_bytes,
    );

    let sets = [descriptor_sets.pool.inst_static[i]];
    device.cmd_bind_descriptor_sets(
      cb,
      vk::PipelineBindPoint::COMPUTE,
      pipelines.compute.layout,
      0,
      &sets,
      &[],
    );
    device.cmd_bind_pipeline(cb, vk::PipelineBindPoint::COMPUTE, pipelines.compute.inst);

    device.cmd_dispatch(cb, buffers.local_constant.inst.count / 64 + 1, 1, 1);

    device
      .end_command_buffer(cb)
      .expect("Failed to finish recording command buffer")
  }

  pub unsafe fn record_inst_dyn(
    &mut self,
    i: usize,
    device: &ash::Device,
    pipelines: &Pipelines,
    descriptor_sets: &DescriptorSets,
    projection_view: &Matrix4<f32>,
    dyn_inst_count: u32,
  ) {
    let cb = self.inst_dyn[i];
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

    let projection_view_bytes = utility::any_as_u8_slice(projection_view);
    device.cmd_push_constants(
      cb,
      pipelines.compute.layout,
      vk::ShaderStageFlags::COMPUTE,
      0,
      projection_view_bytes,
    );

    let sets = [descriptor_sets.pool.inst_dyn[i]];
    device.cmd_bind_descriptor_sets(
      cb,
      vk::PipelineBindPoint::COMPUTE,
      pipelines.compute.layout,
      0,
      &sets,
      &[],
    );
    device.cmd_bind_pipeline(cb, vk::PipelineBindPoint::COMPUTE, pipelines.compute.inst);

    device.cmd_dispatch(cb, dyn_inst_count / 64 + 1, 1, 1);

    device
      .end_command_buffer(cb)
      .expect("Failed to finish recording command buffer")
  }

  pub unsafe fn destroy_self(&mut self, device: &ash::Device) {
    device.destroy_command_pool(self.pool, None);
  }
}
