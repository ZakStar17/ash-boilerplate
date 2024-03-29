use ash::vk;
use std::mem::MaybeUninit;

use crate::render::{
  objects::{MatrixInstance, QueueFamilyIndices},
  sync::FRAMES_IN_FLIGHT,
  utility,
};

use super::{
  allocate_vk_buffers, create_buffer_with_sharing_exclusive, HOST_MEMORY_PROPERTY_FLAGS,
  STORAGE_SRC_USAGE,
};

pub struct HostWritableMemory {
  memory: vk::DeviceMemory,
  pub inst: [(vk::Buffer, u64); FRAMES_IN_FLIGHT],
}

impl HostWritableMemory {
  pub fn new(
    device: &ash::Device,
    memory_properties: vk::PhysicalDeviceMemoryProperties,
    queue_families: &QueueFamilyIndices,
    max_dyn_inst_count: u64,
  ) -> Self {
    let inst_size = std::mem::size_of::<MatrixInstance>() as u64 * max_dyn_inst_count;

    let instance_usages: Vec<_> = std::iter::repeat((inst_size, STORAGE_SRC_USAGE))
      .take(FRAMES_IN_FLIGHT)
      .collect();

    let buffers = instance_usages
      .into_iter()
      .map(|(size, usage)| {
        (
          size,
          create_buffer_with_sharing_exclusive(device, size, usage, queue_families.compute),
        )
      })
      .collect();

    let (memory, _memory_size, offsets) = allocate_vk_buffers(
      device,
      &buffers,
      memory_properties,
      HOST_MEMORY_PROPERTY_FLAGS,
    );

    let mut instance_iter = buffers
      .into_iter()
      .map(|(_, buffer)| buffer)
      .zip(offsets.into_iter());
    let inst = utility::iter_into_array!(instance_iter, FRAMES_IN_FLIGHT);

    Self { memory, inst }
  }

  pub unsafe fn write_instance(
    &mut self,
    i: usize,
    device: &ash::Device,
    data: &Vec<MatrixInstance>,
  ) {
    // writes instance data from the start of the buffer
    let data_ptr = device
      .map_memory(
        self.memory,
        self.inst[i].1,
        (std::mem::size_of::<MatrixInstance>() * data.len()) as u64,
        vk::MemoryMapFlags::empty(),
      )
      .expect("Failed to map memory") as *mut MatrixInstance;
    data_ptr.copy_from_nonoverlapping(data.as_ptr(), data.len());
    device.unmap_memory(self.memory);
  }

  pub unsafe fn destroy_self(&mut self, device: &ash::Device) {
    for (buffer, _) in self.inst.iter_mut() {
      device.destroy_buffer(*buffer, None);
    }
    device.free_memory(self.memory, None);
  }
}
