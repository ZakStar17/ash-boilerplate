use std::{ops::BitOr, ptr};

use ash::vk;

use crate::render::objects::QueueFamilyIndices;

pub struct CopyBufferOperation {
  pub source_buffer: vk::Buffer,
  pub dest_buffer: vk::Buffer,
  pub copy_regions: Vec<vk::BufferCopy>,
}

pub struct TransferCommandBufferPool {
  pool: vk::CommandPool,
  pub copy_buffer: vk::CommandBuffer,
}

impl TransferCommandBufferPool {
  pub fn create(device: &ash::Device, queue_families: &QueueFamilyIndices) -> Self {
    let flags =
      vk::CommandPoolCreateFlags::TRANSIENT.bitor(vk::CommandPoolCreateFlags::RESET_COMMAND_BUFFER);
    let pool = if let Some(transfer) = queue_families.transfer {
      super::create_command_pool(device, flags, transfer)
    } else {
      super::create_command_pool(device, flags, queue_families.graphics)
    };

    let mut buffer_iter = super::create_command_buffers(device, pool, 1).into_iter();
    let copy_buffer = buffer_iter.next().unwrap();

    Self { pool, copy_buffer }
  }

  pub unsafe fn record_copy_buffers(
    &mut self,
    device: &ash::Device,
    operations: &[CopyBufferOperation],
  ) {
    device
      .reset_command_buffer(self.copy_buffer, vk::CommandBufferResetFlags::empty())
      .expect("Failed to reset command buffer");

    let command_buffer_begin_info = vk::CommandBufferBeginInfo {
      s_type: vk::StructureType::COMMAND_BUFFER_BEGIN_INFO,
      p_next: ptr::null(),
      p_inheritance_info: ptr::null(),
      flags: vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT,
    };

    device
      .begin_command_buffer(self.copy_buffer, &command_buffer_begin_info)
      .expect("Failed to start recording command buffer");

    for op in operations {
      device.cmd_copy_buffer(
        self.copy_buffer,
        op.source_buffer,
        op.dest_buffer,
        &op.copy_regions,
      );
    }

    device
      .end_command_buffer(self.copy_buffer)
      .expect("Failed to finish recording command buffer")
  }

  pub unsafe fn destroy_self(&mut self, device: &ash::Device) {
    device.destroy_command_pool(self.pool, None);
  }
}
