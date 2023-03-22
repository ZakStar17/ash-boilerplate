use std::{ops::BitOr, ptr};

use ash::vk;

use crate::render::{
  models::ModelProperties,
  objects::{Buffers, InstProperties, Pipelines, QueueFamilyIndices},
};

pub struct MainCommandBufferPool {
  pool: vk::CommandPool,
  command_buffers: Vec<vk::CommandBuffer>,
}

impl MainCommandBufferPool {
  pub fn create(device: &ash::Device, queue_families: &QueueFamilyIndices) -> Self {
    let flags =
      vk::CommandPoolCreateFlags::TRANSIENT.bitor(vk::CommandPoolCreateFlags::RESET_COMMAND_BUFFER);
    let pool = super::create_command_pool(device, flags, queue_families.graphics);
    let buffers = super::create_command_buffers(device, pool, 2);

    Self {
      pool,
      command_buffers: buffers,
    }
  }

  pub unsafe fn record(
    &mut self,
    i: usize,
    device: &ash::Device,
    render_pass: vk::RenderPass,
    framebuffer: vk::Framebuffer,
    surface_extent: vk::Extent2D,
    pipelines: &Pipelines,
    buffers: &Buffers,
    model_props: &Vec<ModelProperties>,
    dyn_inst_props: &Vec<InstProperties>,
  ) {
    let command_buffer = self.command_buffers[i];

    device
      .reset_command_buffer(command_buffer, vk::CommandBufferResetFlags::empty())
      .expect("Failed to reset command buffer");

    let command_buffer_begin_info = vk::CommandBufferBeginInfo {
      s_type: vk::StructureType::COMMAND_BUFFER_BEGIN_INFO,
      p_next: ptr::null(),
      p_inheritance_info: ptr::null(),
      flags: vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT,
    };

    device
      .begin_command_buffer(command_buffer, &command_buffer_begin_info)
      .expect("Failed to start recording command buffer");

    let clear_values = [vk::ClearValue {
      color: vk::ClearColorValue {
        float32: [0.0, 0.0, 0.0, 1.0],
      },
    }];

    let render_pass_begin_info = vk::RenderPassBeginInfo {
      s_type: vk::StructureType::RENDER_PASS_BEGIN_INFO,
      p_next: ptr::null(),
      render_pass,
      framebuffer,
      render_area: vk::Rect2D {
        offset: vk::Offset2D { x: 0, y: 0 },
        extent: surface_extent,
      },
      clear_value_count: clear_values.len() as u32,
      p_clear_values: clear_values.as_ptr(),
    };

    device.cmd_begin_render_pass(
      command_buffer,
      &render_pass_begin_info,
      vk::SubpassContents::INLINE,
    );
    device.cmd_bind_pipeline(
      command_buffer,
      vk::PipelineBindPoint::GRAPHICS,
      pipelines.graphics.main,
    );
    let vertex_buffers = [buffers.local_constant.vertex, buffers.local.inst[i].0];
    let offsets = [0_u64, 0];

    device.cmd_bind_vertex_buffers(command_buffer, 0, &vertex_buffers, &offsets);
    device.cmd_bind_index_buffer(
      command_buffer,
      buffers.local_constant.index,
      0,
      vk::IndexType::UINT16,
    );

    println!("{:?}", model_props);
    // draw static objects
    for inst_p in buffers.local_constant.inst_props.iter() {
      let model_p = &model_props[inst_p.model_i];
      device.cmd_draw_indexed(
        command_buffer,
        model_p.index_count,
        inst_p.inst_count,
        model_p.index_offset,
        model_p.vertex_offset,
        inst_p.inst_offset,
      )
    }

    // draw dynamic objects (same thing but with offset)
    // I guess this will be better when I find out how to use indirect buffers
    let static_inst_offset = buffers.local_constant.inst_count;
    for inst_p in dyn_inst_props.iter() {
      let model_p = &model_props[inst_p.model_i];
      device.cmd_draw_indexed(
        command_buffer,
        model_p.index_count,
        inst_p.inst_count,
        model_p.index_offset,
        model_p.vertex_offset,
        static_inst_offset + inst_p.inst_offset,
      )
    }

    device.cmd_end_render_pass(command_buffer);

    device
      .end_command_buffer(command_buffer)
      .expect("Failed to finish recording command buffer")
  }

  pub unsafe fn destroy_self(&mut self, device: &ash::Device) {
    device.destroy_command_pool(self.pool, None);
  }

  pub fn command_buffer(&self, i: usize) -> vk::CommandBuffer {
    self.command_buffers[i]
  }
}
