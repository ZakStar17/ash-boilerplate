use std::ffi::CString;

use super::{
  objects::{
    self, Buffers, CommandBufferPools, DebugUtils, DescriptorSets, Pipelines, QueueFamilyIndices,
    Queues, SquareInstance, Swapchains, Vertex,
  },
  ENABLE_VALIDATION_LAYERS, VALIDATION_LAYERS,
};
use crate::{INITIAL_WINDOW_HEIGHT, INITIAL_WINDOW_WIDTH, WINDOW_TITLE};
use ash::vk;
use winit::{event_loop::EventLoop, window::Window};

const DEVICE_EXTENSIONS: [&'static str; 1] = ["VK_KHR_swapchain"];

const VERTICES_DATA: [Vertex; 4] = [
  Vertex {
    pos: [-0.5, -0.5],
    color: [1.0, 0.0, 0.0],
  },
  Vertex {
    pos: [0.5, -0.5],
    color: [0.0, 1.0, 0.0],
  },
  Vertex {
    pos: [0.5, 0.5],
    color: [0.0, 0.0, 1.0],
  },
  Vertex {
    pos: [-1.0, 1.0],
    color: [0.0, 0.0, 0.0],
  },
];

const INDICES_DATA: [u16; 6] = [0, 1, 2, 2, 3, 0];

pub struct Renderer {
  _entry: ash::Entry,
  pub window: Window,
  instance: ash::Instance,
  debug_utils: Option<DebugUtils>,
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
}

impl Renderer {
  pub fn new(event_loop: &EventLoop<()>, max_instance_amount: u64) -> Self {
    // init vulkan stuff
    let entry = unsafe { ash::Entry::load().unwrap() };

    let validation_layers: Option<Vec<std::ffi::CString>> = if ENABLE_VALIDATION_LAYERS {
      check_validation_layers_support(&entry)
        .unwrap_or_else(|name| panic!("The validation layer \"{name}\" was not found"));
      Some(
        VALIDATION_LAYERS
          .iter()
          .map(|name| CString::new(*name).unwrap())
          .collect(),
      )
    } else {
      None
    };

    let window = Self::init_window(event_loop);
    let instance = objects::create_instance(&entry, &window, validation_layers.as_ref());
    let debug_utils = if validation_layers != None {
      Some(DebugUtils::setup(&entry, &instance))
    } else {
      None
    };

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
    let (logical_device, queues) = objects::create_logical_device(
      &instance,
      &physical_device,
      &device_features,
      &device_extensions,
      &queue_family_indices,
      validation_layers.as_ref(),
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

    let descriptor_sets = DescriptorSets::new(&logical_device);

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

    let vertices = Vec::from(VERTICES_DATA);
    let indices = Vec::from(INDICES_DATA);
    let mut command_buffer_pools =
      CommandBufferPools::create(&logical_device, &queue_family_indices);
    let buffers = Buffers::create(
      &instance,
      &logical_device,
      physical_device,
      &queue_family_indices,
      &queues,
      &mut command_buffer_pools,
      &vertices,
      &indices,
      max_instance_amount,
    );

    Self {
      _entry: entry,
      window,
      instance,
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
    instances_len: u32,
  ) {
    self.command_buffer_pools.main.record(
      i,
      &self.device,
      self.render_pass,
      self.framebuffers[framebuffer_i],
      self.swapchains.get_extent(),
      &self.pipelines,
      &self.buffers,
      INDICES_DATA.len() as u32,
      instances_len,
    );
  }

  pub unsafe fn record_instance_compute_command_buffer(&mut self, i: usize, instance_count: u32) {
    self.command_buffer_pools.compute.record_instance(
      i,
      &self.device,
      &self.pipelines,
      &self.descriptor_sets,
      instance_count,
    )
  }

  pub unsafe fn update_instance_data(&mut self, i: usize, data: &Vec<SquareInstance>) {
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
    // also, kinda half baked

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

  pub fn update_instance_compute_descriptor_set(&mut self, i: usize, instance_count: u64) {
    self.descriptor_sets.pool.update_instance_compute(
      i,
      &self.device,
      &self.buffers,
      instance_count,
    );
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
      if let Some(utils) = &mut self.debug_utils {
        utils.destroy_self();
      }
      self.instance.destroy_instance(None);
    }
  }
}

fn check_validation_layers_support(entry: &ash::Entry) -> Result<(), &'static str> {
  let mut available: Vec<&str> = entry
    .enumerate_instance_layer_properties()
    .unwrap()
    .iter()
    .map(|x| {
      let rust_id = unsafe { std::ffi::CStr::from_ptr(x.layer_name.as_ptr()) };
      // println!("{:?}", rust_id);  // print installed validation layer names
      rust_id.to_str().unwrap()
    })
    .collect();
  available.sort();

  for name in VALIDATION_LAYERS {
    if let Err(_) = available.binary_search(&name) {
      return Err(name);
    }
  }
  Ok(())
}
