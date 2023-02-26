use ash::vk;
use log::{debug, info, warn};
use raw_window_handle::HasRawDisplayHandle;
use std::{ffi::CString, ptr};
use winit::window::Window;

use crate::{render::utility, WINDOW_TITLE};

#[cfg(feature = "vulkan_vl")]
use std::os::raw::{c_char, c_void};

pub fn create_instance(
  entry: &ash::Entry,
  window: &Window,
  #[cfg(feature = "vulkan_vl")] vl_pointers: &Vec<*const c_char>,
  #[cfg(feature = "vulkan_vl")] debug_create_info: &vk::DebugUtilsMessengerCreateInfoEXT,
) -> ash::Instance {
  let app_name = CString::new(WINDOW_TITLE).unwrap();
  let engine_name = CString::new("no engine").unwrap();
  let app_info = vk::ApplicationInfo {
    s_type: vk::StructureType::APPLICATION_INFO,
    api_version: vk::API_VERSION_1_3,
    p_application_name: app_name.as_ptr(),
    application_version: vk::make_api_version(0, 1, 0, 0),
    p_engine_name: engine_name.as_ptr(),
    engine_version: vk::make_api_version(0, 1, 0, 0),
    p_next: ptr::null(),
  };

  #[allow(unused_mut)]
  let mut required_extensions =
    ash_window::enumerate_required_extensions(window.raw_display_handle())
      .expect("Failed to enumerate window extensions")
      .to_vec();
  #[cfg(feature = "vulkan_vl")]
  required_extensions.push(ash::extensions::ext::DebugUtils::name().as_ptr());
  test_instance_extension_suport(entry, &required_extensions)
    .unwrap_or_else(|ext| panic!("Required instance extension is not available: {ext}"));

  #[allow(unused_mut)]
  let mut create_info = vk::InstanceCreateInfo {
    s_type: vk::StructureType::INSTANCE_CREATE_INFO,
    p_next: ptr::null(),
    p_application_info: &app_info,
    pp_enabled_layer_names: ptr::null(),
    enabled_layer_count: 0,
    pp_enabled_extension_names: required_extensions.as_ptr(),
    enabled_extension_count: required_extensions.len() as u32,
    flags: vk::InstanceCreateFlags::empty(),
  };

  #[cfg(feature = "vulkan_vl")]
  {
    create_info.p_next =
      debug_create_info as *const vk::DebugUtilsMessengerCreateInfoEXT as *const c_void;
    create_info.pp_enabled_layer_names = vl_pointers.as_ptr();
    create_info.enabled_layer_count = vl_pointers.len() as u32;
  }

  debug!("Creating instance");
  let instance: ash::Instance = unsafe {
    entry
      .create_instance(&create_info, None)
      .expect("Failed to create instance")
  };

  instance
}

fn test_instance_extension_suport(
  entry: &ash::Entry,
  extensions: &Vec<*const i8>,
) -> Result<(), String> {
  let required_extensions: Vec<&str> = extensions
    .iter()
    .map(|x| {
      let rust_id = unsafe { std::ffi::CStr::from_ptr(*x) };
      rust_id.to_str().unwrap()
    })
    .collect();
  info!("Instance required extensions: {:?}", required_extensions);

  let available_extensions: Vec<String> = entry
    .enumerate_instance_extension_properties(None)
    .unwrap()
    .iter()
    .filter_map(|x| match utility::i8_array_to_string(&x.extension_name) {
      Ok(s) => Some(s),
      Err(_) => {
        warn!("Found extension with invalid name");
        None
      }
    })
    .collect();
  let mut available_extensions: Vec<&str> =
    available_extensions.iter().map(|x| x.as_str()).collect();

  info!("Instance available extensions: {:?}", available_extensions);

  match utility::contains_all(&mut available_extensions, &required_extensions) {
    Ok(_) => Ok(()),
    Err(s) => Err(s.to_string()),
  }
}
