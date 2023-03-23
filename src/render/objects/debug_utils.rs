use ash::vk::{self, DebugUtilsMessengerCreateInfoEXT};
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

pub struct DebugUtils {
  loader: ash::extensions::ext::DebugUtils,
  messenger: vk::DebugUtilsMessengerEXT,
}

impl DebugUtils {
  pub fn setup(
    entry: &ash::Entry,
    instance: &ash::Instance,
    create_info: DebugUtilsMessengerCreateInfoEXT,
  ) -> Self {
    let loader = ash::extensions::ext::DebugUtils::new(entry, instance);

    info!("Creating debug utils messenger");
    let messenger = unsafe {
      loader
        .create_debug_utils_messenger(&create_info, None)
        .expect("Failed to create debug utils")
    };

    Self { loader, messenger }
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

  pub unsafe fn destroy_self(&mut self) {
    self
      .loader
      .destroy_debug_utils_messenger(self.messenger, None);
  }
}
