mod camera;
mod cursor;
mod objects;
mod renderable_3d;
mod renderer;
mod shaders;
mod sync;
mod utility;

pub const ENABLE_VALIDATION_LAYERS: bool = true;
pub const VALIDATION_LAYERS: [&'static str; 1] = ["VK_LAYER_KHRONOS_validation"];

pub use camera::Camera;
pub use objects::SquareInstance;
pub use renderable_3d::Renderable3dObject;
pub use sync::SyncRender;
