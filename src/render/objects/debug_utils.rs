use ash::{extensions::ext::DebugUtils, vk};
use log::{debug, error, info, warn};
use std::{ffi::CStr, os::raw::c_void, ptr};

unsafe extern "system" fn vulkan_debug_utils_callback(
  message_severity: vk::DebugUtilsMessageSeverityFlagsEXT,
  message_type: vk::DebugUtilsMessageTypeFlagsEXT,
  p_callback_data: *const vk::DebugUtilsMessengerCallbackDataEXT,
  _p_user_data: *mut c_void,
) -> vk::Bool32 {
  let types = match message_type {
    vk::DebugUtilsMessageTypeFlagsEXT::GENERAL => "[General]",
    vk::DebugUtilsMessageTypeFlagsEXT::PERFORMANCE => "[Performance]",
    vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION => "[Validation]",
    _ => "[Unknown]",
  };
  let message = CStr::from_ptr((*p_callback_data).p_message);
  let message = format!("{} {}", types, message.to_str().unwrap());
  match message_severity {
    vk::DebugUtilsMessageSeverityFlagsEXT::VERBOSE => debug!("{message}"),
    vk::DebugUtilsMessageSeverityFlagsEXT::WARNING => warn!("{message}"),
    vk::DebugUtilsMessageSeverityFlagsEXT::ERROR => error!("{message}"),
    vk::DebugUtilsMessageSeverityFlagsEXT::INFO => info!("{message}"),
    _ => warn!("Unknown: {message}"),
  }

  vk::FALSE
}

pub fn setup_debug_utils(
  entry: &ash::Entry,
  instance: &ash::Instance,
) -> (DebugUtils, vk::DebugUtilsMessengerEXT) {
  let debug_utils_loader = DebugUtils::new(entry, instance);

  let create_info = get_debug_messenger_create_info();

  info!("Creating debug utils messenger");
  let utils_messenger = unsafe {
    debug_utils_loader
      .create_debug_utils_messenger(&create_info, None)
      .expect("Failed to create debug utils")
  };

  (debug_utils_loader, utils_messenger)
}

pub fn get_debug_messenger_create_info() -> vk::DebugUtilsMessengerCreateInfoEXT {
  vk::DebugUtilsMessengerCreateInfoEXT {
    s_type: vk::StructureType::DEBUG_UTILS_MESSENGER_CREATE_INFO_EXT,
    p_next: ptr::null(),
    flags: vk::DebugUtilsMessengerCreateFlagsEXT::empty(),
    message_severity: vk::DebugUtilsMessageSeverityFlagsEXT::WARNING
      | vk::DebugUtilsMessageSeverityFlagsEXT::VERBOSE
      | vk::DebugUtilsMessageSeverityFlagsEXT::INFO
      | vk::DebugUtilsMessageSeverityFlagsEXT::ERROR,
    message_type: vk::DebugUtilsMessageTypeFlagsEXT::GENERAL
      | vk::DebugUtilsMessageTypeFlagsEXT::PERFORMANCE
      | vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION,
    pfn_user_callback: Some(vulkan_debug_utils_callback),
    p_user_data: ptr::null_mut(),
  }
}
