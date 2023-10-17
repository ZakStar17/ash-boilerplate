mod buffers;
mod command_buffer_pools;
#[cfg(feature = "vulkan_vl")]
mod debug_utils;
mod descriptor_sets;
mod framebuffers;
mod instance;
mod logical_device;
mod physical_device;
mod pipelines;
mod render_pass;
mod surface;
mod surface_platforms;
mod swapchain;
mod vert_instance;
mod vertex;

pub struct InstProperties {
  pub inst_count: u32,
  pub inst_offset: u32,
  pub model_i: usize,
}

pub use buffers::Buffers;
pub use command_buffer_pools::CommandBufferPools;
#[cfg(feature = "vulkan_vl")]
pub use debug_utils::DebugUtils;
pub use descriptor_sets::DescriptorSets;
pub use framebuffers::create_framebuffers;
pub use instance::create_instance;
pub use logical_device::{create_logical_device, Queues};
pub use physical_device::{select_physical_device, QueueFamilyIndices};
pub use pipelines::Pipelines;
pub use render_pass::create_render_pass;
pub use surface::create_surface;
pub use swapchain::Swapchains;
pub use vert_instance::MatrixInstance;
pub use vertex::{ColorVertex, TexVertex};
