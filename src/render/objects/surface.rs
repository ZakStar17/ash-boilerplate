use ash::vk;
use log::info;
use winit::window::Window;

use super::surface_platforms;

pub fn create_surface(
  entry: &ash::Entry,
  instance: &ash::Instance,
  window: &Window,
) -> (vk::SurfaceKHR, ash::extensions::khr::Surface) {
  info!("Creating surface");
  let surface = unsafe {
    surface_platforms::create_surface(entry, instance, window).expect("Failed to create surface.")
  };
  let surface_loader = ash::extensions::khr::Surface::new(entry, instance);

  (surface, surface_loader)
}
