use ash::vk;
use log::debug;
use std::mem::MaybeUninit;

use crate::render::{
  objects::{MatrixInstance, QueueFamilyIndices},
  sync::FRAMES_IN_FLIGHT,
  utility,
};

use super::{
  allocate_vk_buffers, create_buffer, LOCAL_MEMORY_PROPERTY_FLAGS, VERTEX_STORAGE_DST_USAGE,
};

pub struct LocalMemory {
  memory: vk::DeviceMemory,
  pub inst: [(vk::Buffer, u64); FRAMES_IN_FLIGHT],
}

impl LocalMemory {
  pub fn new(
    device: &ash::Device,
    memory_properties: vk::PhysicalDeviceMemoryProperties,
    queue_families: &QueueFamilyIndices,
    static_inst_count: u64,
    max_dyn_inst_count: u64,
  ) -> Self {
    debug!(
      "Creating local instance buffer with {} static and {} dynamic count",
      static_inst_count, max_dyn_inst_count
    );
    let inst_size =
      std::mem::size_of::<MatrixInstance>() as u64 * (static_inst_count + max_dyn_inst_count);

    let inst_usages: Vec<_> = std::iter::repeat((inst_size, VERTEX_STORAGE_DST_USAGE))
      .take(FRAMES_IN_FLIGHT)
      .collect();
    // write by compute, read by graphics
    let queue_indices = [queue_families.graphics, queue_families.compute];
    let buffers = inst_usages
      .into_iter()
      .map(|(size, usage)| (size, create_buffer(device, size, usage, &queue_indices)))
      .collect();

    let (memory, _memory_size, offsets) = allocate_vk_buffers(
      device,
      &buffers,
      memory_properties,
      LOCAL_MEMORY_PROPERTY_FLAGS,
    );

    let mut buffers_iter = buffers
      .into_iter()
      .map(|(_, buffer)| buffer)
      .zip(offsets.into_iter());
    let inst = utility::iter_into_array!(buffers_iter, FRAMES_IN_FLIGHT);

    Self { memory, inst }
  }

  pub unsafe fn destroy_self(&mut self, device: &ash::Device) {
    for (buffer, _) in self.inst.iter_mut() {
      device.destroy_buffer(*buffer, None);
    }
    device.free_memory(self.memory, None);
  }
}
