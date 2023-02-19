mod compute;
mod main;
mod transfer;

use std::ptr;

use ash::vk;

use self::{
  compute::ComputeCommandBufferPool, main::MainCommandBufferPool,
  transfer::TransferCommandBufferPool,
};

use super::QueueFamilyIndices;

pub use transfer::CopyBufferOperation;

pub struct CommandBufferPools {
  pub main: MainCommandBufferPool,
  pub transfer: TransferCommandBufferPool,
  pub compute: ComputeCommandBufferPool,
}

impl CommandBufferPools {
  pub fn create(device: &ash::Device, queue_families: &QueueFamilyIndices) -> Self {
    Self {
      main: MainCommandBufferPool::create(device, queue_families),
      transfer: TransferCommandBufferPool::create(device, queue_families),
      compute: ComputeCommandBufferPool::create(device, queue_families),
    }
  }

  pub unsafe fn destroy_self(&mut self, device: &ash::Device) {
    self.main.destroy_self(device);
    self.transfer.destroy_self(device);
    self.compute.destroy_self(device)
  }
}

pub fn create_command_pool(
  device: &ash::Device,
  flags: vk::CommandPoolCreateFlags,
  queue_family_index: u32,
) -> vk::CommandPool {
  let command_pool_create_info = vk::CommandPoolCreateInfo {
    s_type: vk::StructureType::COMMAND_POOL_CREATE_INFO,
    p_next: ptr::null(),
    flags,
    queue_family_index,
  };

  unsafe {
    device
      .create_command_pool(&command_pool_create_info, None)
      .expect("Failed to create Command Pool!")
  }
}

fn create_command_buffers(
  device: &ash::Device,
  command_pool: vk::CommandPool,
  command_buffer_count: u32,
) -> Vec<vk::CommandBuffer> {
  let allocate_info = vk::CommandBufferAllocateInfo {
    s_type: vk::StructureType::COMMAND_BUFFER_ALLOCATE_INFO,
    p_next: ptr::null(),
    command_buffer_count,
    command_pool,
    level: vk::CommandBufferLevel::PRIMARY,
  };

  unsafe {
    device
      .allocate_command_buffers(&allocate_info)
      .expect("Failed to allocate command buffers")
  }
}
