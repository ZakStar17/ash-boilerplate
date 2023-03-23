use super::{
  camera::RenderCamera,
  models::ModelProperties,
  objects::{
    self, Buffers, CommandBufferPools, DescriptorSets, InstProperties, Pipelines,
    QueueFamilyIndices, Queues, Swapchains,
  },
  MatrixInstance, Models, DEVICE_EXTENSIONS,
};

use crate::{INITIAL_WINDOW_HEIGHT, INITIAL_WINDOW_WIDTH, WINDOW_TITLE};
use ash::vk;
use winit::{event_loop::EventLoop, window::Window};

#[cfg(feature = "vulkan_vl")]
use super::{objects::DebugUtils, VALIDATION_LAYERS};
#[cfg(feature = "vulkan_vl")]
use log::info;
#[cfg(feature = "vulkan_vl")]
use std::ffi::{c_char, CStr};

pub struct Renderer {
  _entry: ash::Entry,
  pub window: Window,
  instance: ash::Instance,
  #[cfg(feature = "vulkan_vl")]
  debug_utils: DebugUtils,
  physical_device: vk::PhysicalDevice,
  _queue_family_indices: QueueFamilyIndices,
  pub device: ash::Device,
  pub queues: Queues,
  surface: vk::SurfaceKHR,
  surface_loader: ash::extensions::khr::Surface,
  pub swapchains: Swapchains,
  pub pipelines: Pipelines,
  descriptor_sets: DescriptorSets,
  render_pass: vk::RenderPass,
  framebuffers: Vec<vk::Framebuffer>,
  pub command_buffer_pools: CommandBufferPools,
  buffers: Buffers,
  model_props: Vec<ModelProperties>,
}

#[cfg(all(feature = "link_vulkan", feature = "load_vulkan"))]
compile_error!("Cannot load and link Vulkan at the same time");

#[allow(unreachable_code)]
unsafe fn get_entry() -> ash::Entry {
  #[cfg(feature = "link_vulkan")]
  return ash::Entry::linked();
  #[cfg(feature = "load_vulkan")]
  return ash::Entry::load().expect("Failed to load entry");
  panic!("No compile feature was included for accessing Vulkan");
}

#[cfg(feature = "vulkan_vl")]
fn check_validation_layers_support(entry: &ash::Entry) -> Result<(), &'static CStr> {
  let properties: Vec<vk::LayerProperties> = entry.enumerate_instance_layer_properties().unwrap();
  let mut available: Vec<&CStr> = properties
    .iter()
    .map(|p| {
      let i8slice: &[i8] = &p.layer_name;
      let slice: &[u8] =
        unsafe { std::slice::from_raw_parts(i8slice.as_ptr() as *const u8, i8slice.len()) };
      CStr::from_bytes_until_nul(slice).expect("Failed to read system available validation layer")
    })
    .collect();
  available.sort();

  info!("System available validation layers: {:?}", available);

  for name in VALIDATION_LAYERS {
    if let Err(_) = available.binary_search_by(|&av| av.cmp(name)) {
      return Err(name);
    }
  }
  Ok(())
}

impl Renderer {
  pub fn new(event_loop: &EventLoop<()>, max_dyn_inst_count: u64) -> Self {
    let entry: ash::Entry = unsafe { get_entry() };

    #[cfg(feature = "vulkan_vl")]
    check_validation_layers_support(&entry)
      .unwrap_or_else(|name| panic!("The validation layer {:?} was not found", name));
    #[cfg(feature = "vulkan_vl")]
    let vl_pointers: Vec<*const c_char> =
      VALIDATION_LAYERS.iter().map(|name| name.as_ptr()).collect();
    #[cfg(feature = "vulkan_vl")]
    let debug_create_info = DebugUtils::get_debug_messenger_create_info();

    let window = Self::init_window(event_loop);

    #[cfg(feature = "vulkan_vl")]
    let instance = objects::create_instance(&entry, &window, &vl_pointers, &debug_create_info);
    #[cfg(not(feature = "vulkan_vl"))]
    let instance = objects::create_instance(&entry, &window);

    #[cfg(feature = "vulkan_vl")]
    let debug_utils = DebugUtils::setup(&entry, &instance, debug_create_info);

    let (surface, surface_loader) = objects::create_surface(&entry, &instance, &window);

    let device_features = vk::PhysicalDeviceFeatures::default();
    let device_extensions: Vec<String> = DEVICE_EXTENSIONS.iter().map(|x| x.to_string()).collect();
    let (physical_device, queue_family_indices) = unsafe {
      objects::select_physical_device(
        &instance,
        &surface,
        &surface_loader,
        &device_extensions,
        &device_features,
      )
    };

    #[cfg(feature = "vulkan_vl")]
    let (logical_device, queues) = objects::create_logical_device(
      &instance,
      &physical_device,
      &device_features,
      &device_extensions,
      &queue_family_indices,
      &vl_pointers,
    );
    #[cfg(not(feature = "vulkan_vl"))]
    let (logical_device, queues) = objects::create_logical_device(
      &instance,
      &physical_device,
      &device_features,
      &device_extensions,
      &queue_family_indices,
    );

    let swapchains = Swapchains::new(
      &instance,
      physical_device,
      &logical_device,
      surface,
      &surface_loader,
      &window.inner_size(),
    );

    let render_pass = objects::create_render_pass(&logical_device, swapchains.get_format());

    let mut descriptor_sets = DescriptorSets::new(&logical_device);

    let pipelines = Pipelines::new(
      &logical_device,
      swapchains.get_extent(),
      render_pass,
      &descriptor_sets,
    );

    let framebuffers = objects::create_framebuffers(
      &logical_device,
      render_pass,
      &swapchains.get_image_views(),
      &swapchains.get_extent(),
    );

    let mut command_buffer_pools =
      CommandBufferPools::create(&logical_device, &queue_family_indices);
    let models = Models::load();
    let buffers = Buffers::create(
      &instance,
      &logical_device,
      physical_device,
      &queue_family_indices,
      &queues,
      &mut command_buffer_pools,
      &models,
      max_dyn_inst_count,
    );

    descriptor_sets
      .pool
      .update_all_inst_static(&logical_device, &buffers);

    Self {
      _entry: entry,
      window,
      instance,
      #[cfg(feature = "vulkan_vl")]
      debug_utils,
      physical_device,
      _queue_family_indices: queue_family_indices,
      device: logical_device,
      queues,
      surface,
      surface_loader,
      swapchains,
      pipelines,
      render_pass,
      framebuffers,
      buffers,
      command_buffer_pools,
      descriptor_sets,
      model_props: models.into_properties(),
    }
  }

  fn init_window(event_loop: &EventLoop<()>) -> Window {
    winit::window::WindowBuilder::new()
      .with_title(WINDOW_TITLE)
      .with_inner_size(winit::dpi::LogicalSize::new(
        INITIAL_WINDOW_WIDTH,
        INITIAL_WINDOW_HEIGHT,
      ))
      .build(event_loop)
      .expect("Failed to create window.")
  }

  pub unsafe fn record_main_command_buffer(
    &mut self,
    i: usize,
    framebuffer_i: usize,
    dyn_inst_props: &Vec<InstProperties>,
  ) {
    self.command_buffer_pools.main.record(
      i,
      &self.device,
      self.render_pass,
      self.framebuffers[framebuffer_i],
      self.swapchains.get_extent(),
      &self.pipelines,
      &self.buffers,
      &self.model_props,
      dyn_inst_props,
    );
  }

  pub unsafe fn record_inst_static_comm_buffer(&mut self, i: usize, camera: &RenderCamera) {
    self.command_buffer_pools.compute.record_inst_static(
      i,
      &self.device,
      &self.pipelines,
      &self.buffers,
      &self.descriptor_sets,
      &camera.projection_view(),
    );
  }

  pub unsafe fn record_inst_dyn_comm_buffer(
    &mut self,
    i: usize,
    camera: &RenderCamera,
    dyn_inst_count: u32,
  ) {
    self.command_buffer_pools.compute.record_inst_dyn(
      i,
      &self.device,
      &self.pipelines,
      &self.descriptor_sets,
      &camera.projection_view(),
      dyn_inst_count,
    )
  }

  pub fn get_aspect_ratio(&self) -> f32 {
    let window_size = self.window.inner_size();
    window_size.width as f32 / window_size.height as f32
  }

  pub unsafe fn update_instance_data(&mut self, i: usize, data: &Vec<MatrixInstance>) {
    self.buffers.update_instance_data(i, &self.device, data);
  }

  pub unsafe fn acquire_next_image(
    &mut self,
    semaphore: vk::Semaphore,
  ) -> Result<(u32, bool), vk::Result> {
    self.swapchains.acquire_next_image(semaphore)
  }

  pub unsafe fn queue_present(
    &mut self,
    image_index: u32,
    wait_semaphores: &[vk::Semaphore],
  ) -> Result<bool, vk::Result> {
    self
      .swapchains
      .queue_present(image_index, self.queues.graphics, wait_semaphores)
  }

  pub unsafe fn recreate_swapchain(&mut self, swapchain_render_finished: vk::Fence) {
    // TODO:
    // currenty the code waits for the old swapchain to finish rendering before recreating its dependencies
    // maybe there is a way to make it continue working while already preparing to acquire and present at the new swapchain
    // however, marking render_pass and pipeline as "old" and creating new ones seems quite bothersome and not right

    // old swapchain becomes retired
    let changes = self.swapchains.recreate_swapchain(
      self.physical_device,
      &self.device,
      self.surface,
      &self.surface_loader,
      &self.window.inner_size(),
    );

    unsafe {
      self
        .device
        .wait_for_fences(&[swapchain_render_finished], true, std::u64::MAX)
        .expect("Failed to wait for fence");
    }

    for &framebuffer in self.framebuffers.iter() {
      self.device.destroy_framebuffer(framebuffer, None);
    }

    if changes.extent {
      if changes.format {
        self.device.destroy_render_pass(self.render_pass, None);
        self.render_pass = objects::create_render_pass(&self.device, self.swapchains.get_format());
      }
      self
        .pipelines
        .recreate_main(&self.device, self.swapchains.get_extent(), self.render_pass);
    }
    // kill retired swapchain
    self.swapchains.destroy_old(&self.device);

    self.framebuffers = objects::create_framebuffers(
      &self.device,
      self.render_pass,
      &self.swapchains.get_image_views(),
      &self.swapchains.get_extent(),
    );
  }

  pub fn update_inst_dyn_descriptor_set(&mut self, i: usize, dyn_inst_count: u64) {
    self
      .descriptor_sets
      .pool
      .update_inst_dyn(i, &self.device, &self.buffers, dyn_inst_count);
  }
}

impl Drop for Renderer {
  fn drop(&mut self) {
    unsafe {
      self.command_buffer_pools.destroy_self(&self.device);
      self.buffers.destroy_self(&self.device);
      for &framebuffer in self.framebuffers.iter() {
        self.device.destroy_framebuffer(framebuffer, None);
      }
      self.pipelines.destroy_self(&self.device);
      self.descriptor_sets.destroy_self(&self.device);
      self.device.destroy_render_pass(self.render_pass, None);
      self.swapchains.destroy_self(&self.device);
      self.device.destroy_device(None);
      self.surface_loader.destroy_surface(self.surface, None);
      #[cfg(feature = "vulkan_vl")]
      self.debug_utils.destroy_self();
      self.instance.destroy_instance(None);
    }
  }
}
