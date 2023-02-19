mod objects;
mod renderer;
mod shaders;
mod sync;
mod utility;

pub const VALIDATION_LAYERS: [&'static str; 1] = ["VK_LAYER_KHRONOS_validation"];

pub use objects::SquareInstance;
pub use sync::SyncRender;
