mod camera;
mod cursor;
mod objects;
mod renderable_3d;
mod renderer;
mod shaders;
mod sync;
mod utility;

use std::ffi::CStr;

macro_rules! cstr {
  ( $s:literal ) => {{
    unsafe { std::mem::transmute::<_, &CStr>(concat!($s, "\0")) }
  }};
}

pub const ENABLE_VALIDATION_LAYERS: bool = true;
// validation layers should be valid cstrings (for example, not contain null bytes)
pub const VALIDATION_LAYERS: [&'static CStr; 1] = [cstr!("VK_LAYER_KHRONOS_validation")];

pub use camera::Camera;
pub use objects::SquareInstance;
pub use renderable_3d::Renderable3dObject;
pub use sync::SyncRender;
