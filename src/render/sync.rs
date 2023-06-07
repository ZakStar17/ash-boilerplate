use std::{
  ptr,
  time::{Duration, Instant},
};

use ash::vk;
use log::{info, warn};
use winit::{dpi::PhysicalPosition, event_loop::EventLoop, window::CursorGrabMode};

use crate::{
  objects::Niko,
  FPS_PRINT_INTERVAL, GPU_PRINT_INTERVAL, PRINT_FPS, PRINT_GPU_WAIT,
};

use super::{
  camera::{Camera, RenderCamera},
  cursor::Cursor,
  objects::InstProperties,
  renderer::Renderer,
  MatrixInstance, RenderableIn3d,
};

// only 2 will work
pub const FRAMES_IN_FLIGHT: usize = 2;

pub const INITIAL_CAMERA_FOV: f32 = 0.8;
pub const CAMERA_SENTIVITY: f32 = 0.003;

struct Frame {
  pub image_available: vk::Semaphore,
  pub render_finished: vk::Semaphore,
  pub finished: vk::Fence,
  pub instance_compute_finished: vk::Semaphore,
}

impl Frame {
  pub fn new(device: &ash::Device) -> Self {
    let semaphore_create_info = vk::SemaphoreCreateInfo {
      s_type: vk::StructureType::SEMAPHORE_CREATE_INFO,
      p_next: ptr::null(),
      flags: vk::SemaphoreCreateFlags::empty(),
    };

    let image_available = unsafe {
      device
        .create_semaphore(&semaphore_create_info, None)
        .expect("Failed to create Semaphore Object!")
    };
    let render_finished = unsafe {
      device
        .create_semaphore(&semaphore_create_info, None)
        .expect("Failed to create Semaphore Object!")
    };
    let instance_compute_finished = unsafe {
      device
        .create_semaphore(&semaphore_create_info, None)
        .expect("Failed to create Semaphore Object!")
    };

    let fence_create_info = vk::FenceCreateInfo {
      s_type: vk::StructureType::FENCE_CREATE_INFO,
      p_next: ptr::null(),
      flags: vk::FenceCreateFlags::SIGNALED,
    };

    let finished = unsafe {
      device
        .create_fence(&fence_create_info, None)
        .expect("Failed to create Fence Object!")
    };
    Self {
      image_available,
      render_finished,
      instance_compute_finished,
      finished,
    }
  }

  pub unsafe fn destroy(&mut self, device: &ash::Device) {
    device.device_wait_idle().expect("Failed to wait on device");
    device.destroy_semaphore(self.image_available, None);
    device.destroy_semaphore(self.render_finished, None);
    device.destroy_semaphore(self.instance_compute_finished, None);
    device.destroy_fence(self.finished, None);
  }
}

struct FPSCounter {
  last_print_elapsed_time: Duration,
}

impl FPSCounter {
  pub fn new() -> Self {
    Self {
      last_print_elapsed_time: Duration::from_millis(0),
    }
  }

  pub fn try_print(&mut self, time_passed: &Duration) {
    self.last_print_elapsed_time += *time_passed;
    if self.last_print_elapsed_time > FPS_PRINT_INTERVAL {
      println!(
        "Current fps: {}",
        1000000.0 / (time_passed.as_micros() as f64)
      );
      self.last_print_elapsed_time -= FPS_PRINT_INTERVAL;
    }
  }
}

struct GPULattency {
  prev_frame_time: Instant,
  last_print_elapsed_time: Duration,
  latencies: Vec<Duration>,
}

impl GPULattency {
  pub fn new() -> Self {
    Self {
      prev_frame_time: Instant::now(),
      last_print_elapsed_time: Duration::from_millis(0),
      latencies: Vec::new(),
    }
  }

  pub fn try_print(&mut self, cur_frame_latency: Duration) {
    self.latencies.push(cur_frame_latency);

    let current = Instant::now();
    let elapsed = current - self.prev_frame_time;
    self.last_print_elapsed_time += elapsed;

    if self.last_print_elapsed_time > GPU_PRINT_INTERVAL {
      let average_wait =
        self.latencies.iter().map(|d| d.as_micros()).sum::<u128>() / (self.latencies.len() as u128);
      info!("Average gpu wait: {average_wait}μs",);
      self.latencies.clear();
      self.last_print_elapsed_time -= GPU_PRINT_INTERVAL;
    }

    self.prev_frame_time = current;
  }
}

pub struct SyncRender {
  renderer: Renderer,
  frames: Vec<Frame>,
  last_in_use_i: usize,
  fps_counter: Option<FPSCounter>,
  gpu_latency_counter: Option<GPULattency>,
  recreate_swapchain_next_frame: bool,
  cursor: Cursor,
  middle_screen: PhysicalPosition<f64>,
  pub camera: RenderCamera,
  updated_aspect_ratio: bool,
  delta_zoom: f32,
}

impl SyncRender {
  pub fn initialize(event_loop: &EventLoop<()>, camera: Camera, max_square_amount: u64) -> Self {
    let renderer = Renderer::new(event_loop, max_square_amount);
    let frames = vec![Frame::new(&renderer.device), Frame::new(&renderer.device)];

    let fps_counter = if PRINT_FPS {
      Some(FPSCounter::new())
    } else {
      None
    };

    let gpu_latency_counter = if PRINT_FPS {
      Some(GPULattency::new())
    } else {
      None
    };

    let window_dimensions = renderer.window.inner_size();
    let aspect_ratio = window_dimensions.width as f32 / window_dimensions.height as f32;
    let middle_screen = PhysicalPosition {
      x: window_dimensions.width as f64 / 2.0,
      y: window_dimensions.height as f64 / 2.0,
    };

    let camera = RenderCamera::new(camera, INITIAL_CAMERA_FOV, aspect_ratio, CAMERA_SENTIVITY);

    Self {
      renderer,
      frames,
      last_in_use_i: 0,
      fps_counter,
      gpu_latency_counter,
      recreate_swapchain_next_frame: false,
      cursor: Cursor::new(),
      middle_screen,
      camera,
      updated_aspect_ratio: false,
      delta_zoom: 0.0,
    }
  }

  pub fn handle_window_resize(&mut self) {
    // calculating the aspect ratio involves recalculating camera's projection matrix
    self.updated_aspect_ratio = true;
    let window_dimensions = self.renderer.window.inner_size();
    self.middle_screen = PhysicalPosition {
      x: window_dimensions.width as f64 / 2.0,
      y: window_dimensions.height as f64 / 2.0,
    };
    self.cursor.delta_x = 0.0;
    self.cursor.delta_y = 0.0;
    self.recreate_swapchain_next_frame = true;
  }

  pub fn handle_cursor_moved(&mut self, position: PhysicalPosition<f64>) {
    if self.cursor.getting_grabbed {
      if self.cursor.in_window {
        self.cursor.delta_x += position.x - self.middle_screen.x;
        self.cursor.delta_y += position.y - self.middle_screen.y;
      }
      self
        .renderer
        .window
        .set_cursor_position(self.middle_screen)
        .unwrap();
    }
  }

  pub fn handle_mouse_wheel(&mut self, delta: f32) {
    // no need to recalculate zoom every event
    self.delta_zoom += delta;
  }

  fn grab_cursor(&mut self) {
    // try to grab in some way or another
    // the program resets the mouse each frame anyway, however its nice when you can't
    // accidentaly leave the application from a big mouse jump
    self
      .renderer
      .window
      .set_cursor_grab(CursorGrabMode::Locked)
      .unwrap_or_else(|_| {
        self
          .renderer
          .window
          .set_cursor_grab(CursorGrabMode::Confined)
          .unwrap_or(());
      });

    self
      .renderer
      .window
      .set_cursor_position(self.middle_screen)
      .unwrap();
    // self.renderer.window.set_cursor_visible(false);
    self.cursor.delta_x = 0.0;
    self.cursor.delta_y = 0.0;
  }

  fn ungrab_cursor(&mut self) {
    self
      .renderer
      .window
      .set_cursor_grab(CursorGrabMode::None)
      .unwrap();
    self.renderer.window.set_cursor_visible(true);
  }

  pub fn handle_cursor_entered_window(&mut self) {
    if self.cursor.getting_grabbed {
      self.grab_cursor();
    }
    self.cursor.in_window = true;
  }

  pub fn handle_cursor_left_window(&mut self) {
    if self.cursor.getting_grabbed {
      self.ungrab_cursor();
    }
    self.cursor.in_window = false;
  }

  pub fn toggle_cursor_grab(&mut self) {
    if self.cursor.getting_grabbed {
      self.cursor.getting_grabbed = false;
      self.ungrab_cursor();
    } else {
      self.cursor.getting_grabbed = true;
      self.grab_cursor();
    }
  }

  pub fn render_next_frame(&mut self, time_since_last_frame: &Duration, dyn_objects: &Vec<Niko>) {
    let s = Duration::from_secs_f32(1.0 / 60.0);
    if time_since_last_frame < &s {
      std::thread::sleep(s - *time_since_last_frame);
    }
    if self.updated_aspect_ratio {
      self
        .camera
        .set_aspect_ratio(self.renderer.get_aspect_ratio());
    }
    if self.delta_zoom > 0.0 {
      self.camera.zoom_relative(self.delta_zoom);
      self.delta_zoom = 0.0;
    }
    self
      .camera
      .rotate(self.cursor.delta_x as f32, self.cursor.delta_y as f32);
    self.cursor.delta_x = 0.0;
    self.cursor.delta_y = 0.0;

    // todo: needs refinement / optimizations
    let square_instances: Vec<MatrixInstance> = dyn_objects
      .iter()
      .map(|sq| MatrixInstance::new(*sq.ren().model()))
      .collect();
    let dyn_inst_props = vec![InstProperties {
      inst_count: dyn_objects.len() as u32,
      inst_offset: 0,
      model_i: dyn_objects[0].model_i(),
    }];

    //

    let cur_frame_i = (self.last_in_use_i + 1) % FRAMES_IN_FLIGHT;
    let cur_frame = &self.frames[cur_frame_i];
    let last_frame = &self.frames[self.last_in_use_i];

    let wait_start = Instant::now();
    if self.recreate_swapchain_next_frame {
      // recreate swapchain - this function currently waits for last frame
      // TODO: implement old swapchain functionality so that rendering doesn't
      // stop during swapchain recreation
      unsafe {
        self.renderer.recreate_swapchain(last_frame.finished);
      }
      self.recreate_swapchain_next_frame = false;

      // current frame fence should already have been signaled (waited upon)
    } else {
      unsafe {
        self
          .renderer
          .device
          .wait_for_fences(&[cur_frame.finished], true, u64::MAX)
          .unwrap();
      }
    }

    // the fence should be signaled afterwards or it will deadlock
    unsafe {
      self
        .renderer
        .device
        .reset_fences(&[cur_frame.finished])
        .expect("failed to reset fence");
    }

    let wait_elapsed = wait_start.elapsed();
    if PRINT_GPU_WAIT {
      if let Some(counter) = &mut self.gpu_latency_counter {
        counter.try_print(wait_elapsed);
      }
    }

    if PRINT_FPS {
      // unecessary if
      if let Some(counter) = &mut self.fps_counter {
        counter.try_print(time_since_last_frame);
      }
    }

    let image_index = match unsafe { self.renderer.acquire_next_image(cur_frame.image_available) } {
      Ok((image_index, suboptimal)) => {
        if suboptimal {
          // recreate swapchain at the next best opportunity
          self.recreate_swapchain_next_frame = true;
        }
        image_index
      }
      Err(_) => unsafe {
        self
          .renderer
          .recreate_swapchain(self.frames[self.last_in_use_i].finished);

        let (image_index, new_suboptimal) = self
          .renderer
          .acquire_next_image(cur_frame.image_available)
          .expect("Invalid swapchain upon recreation");
        if new_suboptimal {
          panic!("Suboptimal swapchain upon recreation");
        }
        image_index
      },
    };

    // image not in use = safe to record current command buffer
    unsafe {
      self
        .renderer
        .record_main_command_buffer(cur_frame_i, image_index as usize, &dyn_inst_props);

      self
        .renderer
        .record_inst_static_comm_buffer(cur_frame_i, &self.camera);

      self
        .renderer
        .update_inst_dyn_descriptor_set(cur_frame_i, dyn_objects.len() as u64);
      self
        .renderer
        .update_instance_data(cur_frame_i, &square_instances);
      self.renderer.record_inst_dyn_comm_buffer(
        cur_frame_i,
        &self.camera,
        dyn_objects.len() as u32,
      );
    }

    // compute queue submit
    let wait_semaphores = [];
    let wait_stages = [];
    let signal_semaphores = [cur_frame.instance_compute_finished];
    // in theory these can be changed to execute on different queues (concurrently)
    let command_buffers = [
      self.renderer.command_buffer_pools.compute.inst_static[cur_frame_i],
      self.renderer.command_buffer_pools.compute.inst_dyn[cur_frame_i],
    ];
    let submit_infos = [vk::SubmitInfo {
      s_type: vk::StructureType::SUBMIT_INFO,
      p_next: ptr::null(),
      wait_semaphore_count: wait_semaphores.len() as u32,
      p_wait_semaphores: wait_semaphores.as_ptr(),
      p_wait_dst_stage_mask: wait_stages.as_ptr(),
      command_buffer_count: command_buffers.len() as u32,
      p_command_buffers: command_buffers.as_ptr(),
      signal_semaphore_count: signal_semaphores.len() as u32,
      p_signal_semaphores: signal_semaphores.as_ptr(),
    }];
    unsafe {
      self
        .renderer
        .device
        .queue_submit(
          self.renderer.queues.compute,
          &submit_infos,
          vk::Fence::null(),
        )
        .expect("failed to execute queue submit");
    }

    // std::thread::sleep(std::time::Duration::from_millis(5000));
    // unsafe {
    //   self.renderer.device.device_wait_idle().unwrap();
    // }

    // graphics queue submit
    let wait_semaphores = [
      cur_frame.image_available,
      cur_frame.instance_compute_finished,
    ];
    let wait_stages = [
      vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT,
      vk::PipelineStageFlags::VERTEX_INPUT,
    ];
    let signal_semaphores = [cur_frame.render_finished];
    let submit_infos = [vk::SubmitInfo {
      s_type: vk::StructureType::SUBMIT_INFO,
      p_next: ptr::null(),
      wait_semaphore_count: wait_semaphores.len() as u32,
      p_wait_semaphores: wait_semaphores.as_ptr(),
      p_wait_dst_stage_mask: wait_stages.as_ptr(),
      command_buffer_count: 1,
      p_command_buffers: &self
        .renderer
        .command_buffer_pools
        .main
        .command_buffer(cur_frame_i),
      signal_semaphore_count: signal_semaphores.len() as u32,
      p_signal_semaphores: signal_semaphores.as_ptr(),
    }];
    unsafe {
      self
        .renderer
        .device
        .queue_submit(
          self.renderer.queues.graphics,
          &submit_infos,
          cur_frame.finished,
        )
        .expect("failed to execute queue submit");
    }

    // unsafe {
    //   self.renderer.device.device_wait_idle().unwrap();
    // }

    unsafe {
      if let Err(_) = self.renderer.queue_present(image_index, &signal_semaphores) {
        // NOTE: It seems that sometimes the window can be resized while the image is being presented
        // this occurs because winit only takes notice at the start of the next frame
        // however, it is very inconsistent, so I don't know how to fix this
        warn!("failed to present to swapchain");
        self.recreate_swapchain_next_frame = true;
      }
    }

    self.last_in_use_i = cur_frame_i;
  }

  pub fn request_redraw(&mut self) {
    self.renderer.window.request_redraw();
  }
}

impl Drop for SyncRender {
  fn drop(&mut self) {
    unsafe {
      for frame in self.frames.iter_mut() {
        frame.destroy(&self.renderer.device);
      }
    }
  }
}
