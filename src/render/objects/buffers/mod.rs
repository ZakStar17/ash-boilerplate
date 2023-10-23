mod host_writable;
mod local;
mod local_constant;

use std::ptr;

use ash::vk;
use log::debug;

use crate::render::Models;

use self::{
  host_writable::HostWritableMemory, local::LocalMemory, local_constant::LocalConstantMemory,
};

use super::{CommandBufferPools, MatrixInstance, QueueFamilyIndices, Queues};

macro_rules! const_flag_bitor {
    ($t:ty, $x:expr, $($y:expr),+) => {
      // ash flags don't implement const bitor
      <$t>::from_raw(
        $x.as_raw() $(| $y.as_raw())+,
      )
    };
}

macro_rules! buffer_usage {
    ( $x:expr, $($y:expr),+) => {
      const_flag_bitor!(vk::BufferUsageFlags, $x $(, $y)+)
    };
}

macro_rules! memory_property_flags {
    ( $x:expr, $($y:expr),+) => {
      const_flag_bitor!(vk::MemoryPropertyFlags, $x, $($y)*)
    };
}

pub const VERTEX_DST_USAGE: vk::BufferUsageFlags = buffer_usage!(
  vk::BufferUsageFlags::VERTEX_BUFFER,
  vk::BufferUsageFlags::TRANSFER_DST
);

pub const STORAGE_SRC_USAGE: vk::BufferUsageFlags = buffer_usage!(
  vk::BufferUsageFlags::TRANSFER_SRC,
  vk::BufferUsageFlags::STORAGE_BUFFER
);
pub const VERTEX_STORAGE_DST_USAGE: vk::BufferUsageFlags =
  buffer_usage!(VERTEX_DST_USAGE, vk::BufferUsageFlags::STORAGE_BUFFER);
pub const STORAGE_USAGE: vk::BufferUsageFlags = buffer_usage!(
  vk::BufferUsageFlags::TRANSFER_SRC,
  vk::BufferUsageFlags::TRANSFER_DST,
  vk::BufferUsageFlags::STORAGE_BUFFER
);

pub const INDEX_DST_USAGE: vk::BufferUsageFlags = buffer_usage!(
  vk::BufferUsageFlags::INDEX_BUFFER,
  vk::BufferUsageFlags::TRANSFER_DST
);

pub const HOST_MEMORY_PROPERTY_FLAGS: vk::MemoryPropertyFlags = memory_property_flags!(
  vk::MemoryPropertyFlags::HOST_VISIBLE,
  vk::MemoryPropertyFlags::HOST_COHERENT
);
// todo: add manual sync and remove host coherent
pub const LOCAL_MEMORY_PROPERTY_FLAGS: vk::MemoryPropertyFlags = memory_property_flags!(
  vk::MemoryPropertyFlags::DEVICE_LOCAL,
  vk::MemoryPropertyFlags::HOST_COHERENT
);

// returns memory, memory size and offset of each buffer
fn allocate_vk_buffers(
  device: &ash::Device,
  // (buffer data size, buffer)
  buffers: &Vec<(u64, vk::Buffer)>,
  mem_properties: vk::PhysicalDeviceMemoryProperties,
  required_mem_flags: vk::MemoryPropertyFlags,
) -> (vk::DeviceMemory, u64, Vec<u64>) {
  let mut buffer_requirements_bits = 0;
  let mut alignment = 0;
  let mut full_sizes = Vec::with_capacity(buffers.len());
  for (_, buffer) in buffers.iter() {
    let mem_requirements = unsafe { device.get_buffer_memory_requirements(*buffer) };
    buffer_requirements_bits |= mem_requirements.memory_type_bits;

    // the specification guarantees that the alignment is a power of 2
    #[cfg(debug_assertions)]
    assert!(
      mem_requirements.alignment > 0
        && (mem_requirements.alignment & (mem_requirements.alignment - 1)) == 0
    );
    alignment = alignment.max(mem_requirements.alignment);

    full_sizes.push(mem_requirements.size);
  }
  // align each internal buffer
  let mut total_size = 0;
  let mut offsets = Vec::with_capacity(buffers.len());
  for size in full_sizes.iter() {
    let size = size + alignment - (size % alignment);
    offsets.push(total_size);
    total_size += size;
  }

  let memory_type = find_memory_type(buffer_requirements_bits, required_mem_flags, mem_properties);

  let allocate_info = vk::MemoryAllocateInfo {
    s_type: vk::StructureType::MEMORY_ALLOCATE_INFO,
    p_next: ptr::null(),
    allocation_size: total_size,
    memory_type_index: memory_type,
  };

  debug!("Allocating buffer memory");
  let buffer_memory = unsafe {
    device
      .allocate_memory(&allocate_info, None)
      .expect("Failed to allocate vertex buffer memory")
  };
  for ((_size, buffer), offset) in buffers.iter().zip(offsets.iter()) {
    unsafe {
      device
        .bind_buffer_memory(*buffer, buffer_memory, *offset)
        .expect("Failed to bind buffer to its memory");
    }
  }

  (buffer_memory, total_size, offsets)
}

pub struct Buffers {
  pub local_constant: LocalConstantMemory,
  pub host_writable: HostWritableMemory,
  pub local: LocalMemory,
}

impl Buffers {
  pub fn create(
    instance: &ash::Instance,
    device: &ash::Device,
    physical_device: vk::PhysicalDevice,
    queue_families: &QueueFamilyIndices,
    queues: &Queues,
    command_pools: &mut CommandBufferPools,
    models: &Models,
    max_dyn_inst_count: u64,
  ) -> Self {
    let memory_properties =
      unsafe { instance.get_physical_device_memory_properties(physical_device) };
    debug!("Allocating constant memory");
    let local_constant = LocalConstantMemory::new(
      device,
      memory_properties,
      queue_families,
      queues,
      command_pools,
      models,
    );
    debug!("Allocating host memory");
    let host_writable = HostWritableMemory::new(
      device,
      memory_properties,
      queue_families,
      max_dyn_inst_count,
    );
    debug!("Allocating local memory");
    let local = LocalMemory::new(
      device,
      memory_properties,
      queue_families,
      local_constant.inst.count(),
      max_dyn_inst_count,
    );

    Self {
      local_constant,
      host_writable,
      local,
    }
  }

  pub unsafe fn update_instance_data(
    &mut self,
    i: usize,
    device: &ash::Device,
    data: &Vec<MatrixInstance>,
  ) {
    self.host_writable.write_instance(i, device, data);
  }

  pub unsafe fn destroy_self(&mut self, device: &ash::Device) {
    self.local_constant.destroy_self(device);
    self.host_writable.destroy_self(device);
    self.local.destroy_self(device);
  }
}

fn create_buffer_with_sharing_exclusive(
  device: &ash::Device,
  size: u64,
  usage: vk::BufferUsageFlags,
  queue_family: u32,
) -> vk::Buffer {
  let vertex_buffer_create_info = vk::BufferCreateInfo {
    s_type: vk::StructureType::BUFFER_CREATE_INFO,
    p_next: ptr::null(),
    flags: vk::BufferCreateFlags::empty(),
    size,
    usage,
    sharing_mode: vk::SharingMode::EXCLUSIVE,
    queue_family_index_count: queue_family,
    p_queue_family_indices: ptr::null(),
  };

  unsafe {
    device
      .create_buffer(&vertex_buffer_create_info, None)
      .expect("failed to create buffer")
  }
}

fn create_buffer(
  device: &ash::Device,
  size: u64,
  usage: vk::BufferUsageFlags,
  queue_family_indices: &[u32],
) -> vk::Buffer {
  assert!(size > 0);
  let vertex_buffer_create_info = vk::BufferCreateInfo {
    s_type: vk::StructureType::BUFFER_CREATE_INFO,
    p_next: ptr::null(),
    flags: vk::BufferCreateFlags::empty(),
    size,
    usage,
    sharing_mode: vk::SharingMode::CONCURRENT,
    queue_family_index_count: queue_family_indices.len() as u32,
    p_queue_family_indices: queue_family_indices.as_ptr(),
  };

  unsafe {
    device
      .create_buffer(&vertex_buffer_create_info, None)
      .expect("failed to create buffer")
  }
}

// creates source and destination buffers from size and buffer usage
fn create_travel_buffers(
  device: &ash::Device,
  queue_families: &QueueFamilyIndices,
  data: Vec<(u64, vk::BufferUsageFlags, vk::BufferUsageFlags)>,
) -> Vec<(u64, vk::Buffer, vk::Buffer)> {
  if let Some(transfer) = queue_families.transfer {
    let queue_indices = [queue_families.graphics, transfer];
    data
      .into_iter()
      .map(|(size, source_usage, dst_usage)| {
        (
          size,
          create_buffer(device, size, source_usage, &queue_indices),
          create_buffer(device, size, dst_usage, &queue_indices),
        )
      })
      .collect()
  } else {
    data
      .into_iter()
      .map(|(size, source_usage, dst_usage)| {
        (
          size,
          create_buffer_with_sharing_exclusive(device, size, source_usage, queue_families.graphics),
          create_buffer_with_sharing_exclusive(device, size, dst_usage, queue_families.graphics),
        )
      })
      .collect()
  }
}

fn find_memory_type(
  type_filter: u32,
  required_properties: vk::MemoryPropertyFlags,
  mem_properties: vk::PhysicalDeviceMemoryProperties,
) -> u32 {
  for (i, memory_type) in mem_properties.memory_types.iter().enumerate() {
    // check each bit in type_filter for support
    if (type_filter & (1 << i)) > 0 && memory_type.property_flags.contains(required_properties) {
      return i as u32;
    }
  }

  panic!("failed to find suitable memory type");
}
