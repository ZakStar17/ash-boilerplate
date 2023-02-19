use ash::vk;
use std::mem::MaybeUninit;

use crate::render::{
  objects::{QueueFamilyIndices, SquareInstance},
  sync::FRAMES_IN_FLIGHT,
  utility,
};

use super::{
  create_buffer_with_sharing_exclusive, SizedBuffer, HOST_MEMORY_PROPERTY_FLAGS, STORAGE_SRC_USAGE,
};

pub struct HostWritableMemory {
  memory: vk::DeviceMemory,
  instance: [SizedBuffer; FRAMES_IN_FLIGHT],
}

impl HostWritableMemory {
  pub fn new(
    device: &ash::Device,
    memory_properties: vk::PhysicalDeviceMemoryProperties,
    queue_families: &QueueFamilyIndices,
    max_instances: u64,
  ) -> Self {
    let instance_size = std::mem::size_of::<SquareInstance>() as u64 * max_instances;

    let instance_usages: Vec<_> = std::iter::repeat((instance_size, STORAGE_SRC_USAGE))
      .take(FRAMES_IN_FLIGHT)
      .collect();

    let vk_buffers = instance_usages
      .into_iter()
      .map(|(size, usage)| {
        (
          size,
          create_buffer_with_sharing_exclusive(device, size, usage, queue_families.compute),
        )
      })
      .collect();

    let (memory, _memory_size, buffers) = SizedBuffer::allocate_vk_buffers(
      device,
      vk_buffers,
      memory_properties,
      HOST_MEMORY_PROPERTY_FLAGS,
    );

    let mut buffers_iter = buffers.into_iter();
    let instance = utility::iter_into_array!(buffers_iter, FRAMES_IN_FLIGHT);

    Self { memory, instance }
  }

  pub unsafe fn write_instance(
    &mut self,
    i: usize,
    device: &ash::Device,
    data: &Vec<SquareInstance>,
  ) {
    // writes instance data from the start of the buffer
    let data_ptr = device
      .map_memory(
        self.memory,
        self.instance[i].offset,
        (std::mem::size_of::<SquareInstance>() * data.len()) as u64,
        vk::MemoryMapFlags::empty(),
      )
      .expect("Failed to map memory") as *mut SquareInstance;
    data_ptr.copy_from_nonoverlapping(data.as_ptr(), data.len());
    device.unmap_memory(self.memory);
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
