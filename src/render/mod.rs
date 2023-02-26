mod camera;
mod cursor;
mod objects;
mod renderable_3d;
mod renderer;
mod shaders;
mod sync;
mod utility;

#[cfg(feature = "vulkan_vl")]
use std::ffi::CStr;

#[cfg(feature = "vulkan_vl")]
macro_rules! cstr {
  ( $s:literal ) => {{
    unsafe { std::mem::transmute::<_, &CStr>(concat!($s, "\0")) }
  }};
}

// validation layers should be valid cstrings (for example, not contain null bytes)
#[cfg(feature = "vulkan_vl")]
pub const VALIDATION_LAYERS: [&'static CStr; 1] = [cstr!("VK_LAYER_KHRONOS_validation")];

pub const DEVICE_EXTENSIONS: [&'static str; 1] = ["VK_KHR_swapchain"];

pub use camera::Camera;
pub use objects::SquareInstance;
pub use renderable_3d::Renderable3dObject;
pub use sync::SyncRender;
