use ash::vk;
use std::mem::MaybeUninit;

use crate::render::{
  objects::{QueueFamilyIndices, SquareInstance},
  sync::FRAMES_IN_FLIGHT,
  utility,
};

use super::{create_buffer, SizedBuffer, LOCAL_MEMORY_PROPERTY_FLAGS, VERTEX_STORAGE_DST_USAGE};

pub struct LocalMemory {
  memory: vk::DeviceMemory,
  instance: [SizedBuffer; 2],
}

impl LocalMemory {
  pub fn new(
    device: &ash::Device,
    memory_properties: vk::PhysicalDeviceMemoryProperties,
    queue_families: &QueueFamilyIndices,
    max_instances: u64,
  ) -> Self {
    let instance_size = std::mem::size_of::<SquareInstance>() as u64 * max_instances;

    let instance_usages: Vec<_> = std::iter::repeat((instance_size, VERTEX_STORAGE_DST_USAGE))
      .take(FRAMES_IN_FLIGHT)
      .collect();
    // write by compute, read by graphics
    let queue_indices = [queue_families.graphics, queue_families.compute];
    let vk_buffers = instance_usages
      .into_iter()
      .map(|(size, usage)| (size, create_buffer(device, size, usage, &queue_indices)))
      .collect();

    let (memory, _memory_size, buffers) = SizedBuffer::allocate_vk_buffers(
      device,
      vk_buffers,
      memory_properties,
      LOCAL_MEMORY_PROPERTY_FLAGS,
    );

    let mut buffers_iter = buffers.into_iter();
    let instance = utility::iter_into_array!(buffers_iter, FRAMES_IN_FLIGHT);

    Self { memory, instance }
  }

  pub fn instance(&self, i: usize) -> vk::Buffer {
    self.instance[i].inner()
  }

  pub unsafe fn destroy_self(&mut self, device: &ash::Device) {
    for buffer in self.instance.iter_mut() {
      buffer.destroy_self(device);
    }
    device.free_memory(self.memory, None);
  }
}
