use std::{
  ptr,
  time::{Duration, Instant},
};

use ash::vk;
use log::{info, warn};
use winit::event_loop::EventLoop;

use crate::{FPS_PRINT_INTERVAL, GPU_PRINT_INTERVAL, PRINT_FPS, PRINT_GPU_WAIT};

use super::{objects::CameraPos, renderer::Renderer, SquareInstance};

// only 2 will work
pub const FRAMES_IN_FLIGHT: usize = 2;

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
      info!(
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
}

impl SyncRender {
  pub fn initialize(event_loop: &EventLoop<()>, max_square_amount: u64) -> Self {
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

    Self {
      renderer,
      frames,
      last_in_use_i: 0,
      fps_counter,
      gpu_latency_counter,
      recreate_swapchain_next_frame: false,
    }
  }

  pub fn handle_window_resize(&mut self) {
    self.recreate_swapchain_next_frame = true;
  }

  pub fn render_next_frame(
    &mut self,
    time_since_last_frame: &Duration,
    squares: &Vec<SquareInstance>,
    camera_pos: &CameraPos
  ) {
    // cpu "intensive" operations
    // std::thread::sleep(std::time::Duration::from_millis(100));

    let cur_frame_i = (self.last_in_use_i + 1) % FRAMES_IN_FLIGHT;
    let cur_frame = &self.frames[cur_frame_i];
    let last_frame = &self.frames[self.last_in_use_i];

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

    let wait_start = Instant::now();
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
      self.renderer.record_main_command_buffer(
        cur_frame_i,
        image_index as usize,
        squares.len() as u32,
      );

      self
        .renderer
        .update_instance_compute_descriptor_set(cur_frame_i, squares.len() as u64);
      self.renderer.update_instance_data(cur_frame_i, squares);
      self.renderer.record_instance_compute_command_buffer(
        cur_frame_i,
        squares.len() as u32,
        camera_pos
      );
    }

    // compute queue submit
    let wait_semaphores = [];
    let wait_stages = [];
    let signal_semaphores = [cur_frame.instance_compute_finished];
    let submit_infos = [vk::SubmitInfo {
      s_type: vk::StructureType::SUBMIT_INFO,
      p_next: ptr::null(),
      wait_semaphore_count: wait_semaphores.len() as u32,
      p_wait_semaphores: wait_semaphores.as_ptr(),
      p_wait_dst_stage_mask: wait_stages.as_ptr(),
      command_buffer_count: 1,
      p_command_buffers: &self.renderer.command_buffer_pools.compute.instance[cur_frame_i],
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
        // however, it is very inconsistent
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
