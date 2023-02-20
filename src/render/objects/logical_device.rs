use ash::vk;
use log::info;
use std::{ffi::CString, os::raw::c_char, ptr};

use super::QueueFamilyIndices;

fn get_queue_create_info(family_index: u32) -> vk::DeviceQueueCreateInfo {
  let queue_priorities = [1.0_f32];
  vk::DeviceQueueCreateInfo {
    s_type: vk::StructureType::DEVICE_QUEUE_CREATE_INFO,
    queue_family_index: family_index,
    queue_count: 1,
    p_queue_priorities: queue_priorities.as_ptr(),
    p_next: ptr::null(),
    flags: vk::DeviceQueueCreateFlags::empty(),
  }
}

pub struct Queues {
  pub graphics: vk::Queue,
  pub compute: vk::Queue,
  pub transfer: Option<vk::Queue>,
}

pub fn create_logical_device(
  instance: &ash::Instance,
  physical_device: &vk::PhysicalDevice,
  device_features: &vk::PhysicalDeviceFeatures,
  device_extensions: &[String],
  family_indices: &QueueFamilyIndices,
  validation_layers: Option<&Vec<CString>>,
) -> (ash::Device, Queues) {
  let mut unique_queue_families = vec![family_indices.graphics, family_indices.compute];
  if let Some(transfer) = family_indices.transfer {
    unique_queue_families.push(transfer);
  }
  let queues_create_info: Vec<vk::DeviceQueueCreateInfo> = unique_queue_families
    .into_iter()
    .map(|i| get_queue_create_info(i))
    .collect();

  let device_extensions_c: Vec<CString> = device_extensions
    .iter()
    .map(|s| CString::new(s.as_bytes()).expect("Invalid device extension"))
    .collect();
  let device_extensions_pointers: Vec<*const c_char> =
    device_extensions_c.iter().map(|s| s.as_ptr()).collect();

  // I wonder how could have I make this work without invalidating the pointer at the end
  // let pointer = if let Some(pointers) = layer_pointers {
  //   pointers.as_ptr()
  // } else {
  //   ptr::null()
  // };

  let mut create_info = vk::DeviceCreateInfo {
    s_type: vk::StructureType::DEVICE_CREATE_INFO,
    p_queue_create_infos: queues_create_info.as_ptr(),
    queue_create_info_count: queues_create_info.len() as u32,
    p_enabled_features: &*device_features,
    p_next: ptr::null(),
    pp_enabled_layer_names: ptr::null(),
    enabled_layer_count: 0,
    pp_enabled_extension_names: device_extensions_pointers.as_ptr(),
    enabled_extension_count: device_extensions_pointers.len() as u32,
    flags: vk::DeviceCreateFlags::empty(),
  };

  // should be valid until after device creation
  let _layer_pointers = if let Some(layers) = validation_layers {
    let layer_pointers: Vec<*const c_char> = layers.iter().map(|name| name.as_ptr()).collect();
    create_info.pp_enabled_layer_names = layer_pointers.as_ptr();
    create_info.enabled_layer_count = layer_pointers.len() as u32;
    Some(layer_pointers)
  } else {
    None
  };

  info!("Creating logical device");
  let device: ash::Device = unsafe {
    instance
      .create_device(*physical_device, &create_info, None)
      .expect("Failed to create logical device")
  };

  let queues = unsafe {
    Queues {
      graphics: device.get_device_queue(family_indices.graphics, 0),
      compute: device.get_device_queue(family_indices.compute, 0),
      transfer: if let Some(transfer) = family_indices.transfer {
        Some(device.get_device_queue(transfer, 0))
      } else {
        None
      },
    }
  };

  (device, queues)
}
