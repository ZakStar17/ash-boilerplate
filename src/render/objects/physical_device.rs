use ash::vk;
use log::{debug, info};

use crate::render::utility;

#[derive(Debug)]
pub struct QueueFamilyIndices {
  pub graphics: u32,
  pub compute: u32,
  pub transfer: Option<u32>,
}

pub unsafe fn select_physical_device(
  instance: &ash::Instance,
  surface: &vk::SurfaceKHR,
  surface_loader: &ash::extensions::khr::Surface,
  device_extensions: &[String],
  device_features: &vk::PhysicalDeviceFeatures,
) -> (vk::PhysicalDevice, QueueFamilyIndices) {
  let (physical_device, queue_family) = instance
    .enumerate_physical_devices()
    .expect("Failed to enumerate physical devices")
    .into_iter()
    .filter(|p| {
      let properties = instance.get_physical_device_properties(*p);
      info!(
        "
Found physical device \"{}\":
api_version: {},
driver_version: {},
vendor: {},
device_id: {},
device_type: {},
limits: {:?}
",
        utility::c_char_array_to_string(&properties.device_name),
        format!(
          "{}.{}.{}",
          vk::api_version_major(properties.api_version),
          vk::api_version_minor(properties.api_version),
          vk::api_version_patch(properties.api_version)
        ),
        properties.driver_version,
        match properties.vendor_id {
          0x1002 => "AMD".to_owned(),
          0x1010 => "ImgTec".to_owned(),
          0x10DE => "NVIDIA".to_owned(),
          0x13B5 => "ARM".to_owned(),
          0x5143 => "Qualcomm".to_owned(),
          0x8086 => "INTEL".to_owned(),
          other => format!("Unknown ({})", other),
        },
        properties.device_id,
        match properties.device_type {
          vk::PhysicalDeviceType::INTEGRATED_GPU => "Integrated GPU",
          vk::PhysicalDeviceType::DISCRETE_GPU => "Discrete GPU",
          vk::PhysicalDeviceType::VIRTUAL_GPU => "Virtual GPU",
          vk::PhysicalDeviceType::CPU => "CPU",
          _ => "Unknown",
        },
        properties.limits
      );

      is_superset(&instance.get_physical_device_features(*p), device_features)
        && check_extension_support(instance, p, device_extensions)
        && check_swapchain_support(p, surface, surface_loader)
    })
    .filter_map(|physical_device| {
      let mut graphics = None;
      let mut compute = None;
      let mut transfer = None;
      for (i, family) in instance
        .get_physical_device_queue_family_properties(physical_device)
        .iter()
        .enumerate()
      {
        let supports_surface = unsafe {
          surface_loader
            .get_physical_device_surface_support(physical_device, i as u32, *surface)
            .expect("Failed to query for physical device surface support")
        };
        if family.queue_flags.contains(vk::QueueFlags::GRAPHICS) && supports_surface {
          graphics = Some(i as u32);
        } else if family.queue_flags.contains(vk::QueueFlags::COMPUTE) {
          compute = Some(i as u32)
        } else if family.queue_flags.contains(vk::QueueFlags::TRANSFER) {
          transfer = Some(i as u32);
        }
      }

      if let Some(graphics) = graphics {
        if let Some(compute) = compute {
          Some((
            physical_device,
            QueueFamilyIndices {
              graphics,
              compute,
              transfer,
            },
          ))
        } else {
          panic!("No available compute queue family found")
        }
      } else {
        None
      }
    })
    .min_by_key(|(physical_device, _)| {
      let t = instance
        .get_physical_device_properties(*physical_device)
        .device_type;
      if t == vk::PhysicalDeviceType::DISCRETE_GPU {
        0
      } else if t == vk::PhysicalDeviceType::INTEGRATED_GPU {
        1
      } else if t == vk::PhysicalDeviceType::VIRTUAL_GPU {
        2
      } else if t == vk::PhysicalDeviceType::CPU {
        3
      } else if t == vk::PhysicalDeviceType::OTHER {
        4
      } else {
        panic!()
      }
    })
    .expect("no device available");

  if let Some(transfer) = queue_family.transfer {
    info!("Found an exclusive queue family for transfer operations with index {transfer}");
  } else {
    info!("An exclusive queue family for transfer operations has not been found");
  }
  print_debug_info(instance, physical_device);

  (physical_device, queue_family)
}

fn print_debug_info(instance: &ash::Instance, physical_device: vk::PhysicalDevice) {
  let mem_properties = unsafe { instance.get_physical_device_memory_properties(physical_device) };
  debug!("available memory heaps:");
  for i in 0..mem_properties.memory_heap_count {
    let heap = mem_properties.memory_heaps[i as usize];
    let flags = if heap.flags.is_empty() {
      String::from("no flags")
    } else {
      format!("flags {:?}", heap.flags)
    };
    debug!("{}: {}mb with {}", i, heap.size / 1000000, flags);
    let mem_type_flags: Vec<vk::MemoryPropertyFlags> = mem_properties.memory_types
      [0..(mem_properties.memory_type_count as usize)]
      .iter()
      .filter_map(|mem_type| {
        if mem_type.heap_index == i {
          Some(mem_type.property_flags)
        } else {
          None
        }
      })
      .collect();
    debug!("available memory type flags: {:?}", mem_type_flags)
  }
}

fn is_superset(_a: &vk::PhysicalDeviceFeatures, _b: &vk::PhysicalDeviceFeatures) -> bool {
  // todo
  true
}

fn check_extension_support(
  instance: &ash::Instance,
  device: &vk::PhysicalDevice,
  extensions: &[String],
) -> bool {
  let available_extensions = unsafe {
    instance
      .enumerate_device_extension_properties(*device)
      .expect("Failed to get device extension properties.")
  };

  let mut available_extensions: Vec<String> = available_extensions
    .into_iter()
    .map(|prop| utility::c_char_array_to_string(&prop.extension_name))
    .collect();

  match utility::contains_all(&mut available_extensions, extensions) {
    Ok(_) => true,
    Err(_) => false,
  }
}

unsafe fn check_swapchain_support(
  device: &vk::PhysicalDevice,
  surface: &vk::SurfaceKHR,
  surface_loader: &ash::extensions::khr::Surface,
) -> bool {
  let formats = surface_loader
    .get_physical_device_surface_formats(*device, *surface)
    .expect("Failed to query for surface formats.");
  let present_modes = surface_loader
    .get_physical_device_surface_present_modes(*device, *surface)
    .expect("Failed to query for surface present mode.");
  !formats.is_empty() && !present_modes.is_empty()
}
