mod camera;
mod cursor;
mod models;
mod objects;
mod renderable_3d;
mod renderer;
mod shaders;
mod sync;
mod textures;
pub mod utility;

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
pub use models::{
  ColorModelIndex, ColorModeled, ColorModels, Models, TexModelIndex, TexModeled, TexModels,
};
pub use objects::MatrixInstance;
pub use renderable_3d::{Renderable3dObject, RenderableIn3d};
pub use sync::SyncRender;
pub use textures::Textures;
