#![feature(iter_next_chunk)]
#![feature(pointer_byte_offsets)]

mod app;
mod keys;
mod objects;
mod render;
mod static_scene;
mod structures;

use std::time::{Duration, Instant};

use app::App;
use log::{debug, info};
use winit::{
  event::{Event, KeyboardInput, MouseScrollDelta, WindowEvent},
  event_loop::{ControlFlow, EventLoop},
};

pub const WINDOW_TITLE: &'static str = "Ash boilerplate";
pub const INITIAL_WINDOW_WIDTH: u32 = 800;
pub const INITIAL_WINDOW_HEIGHT: u32 = 600;

pub const ENABLE_VSYNC: bool = true;

pub const PRINT_FPS: bool = true;
pub const FPS_PRINT_INTERVAL: Duration = Duration::from_millis(2000);

pub const PRINT_GPU_WAIT: bool = true;
pub const GPU_PRINT_INTERVAL: Duration = Duration::from_millis(5000);

pub fn main_loop(event_loop: EventLoop<()>, mut app: App) {
  let mut application_paused = false;
  let mut last_frame_instant = Instant::now();
  event_loop.run(move |event, _, control_flow| match event {
    Event::Suspended => {
      debug!("Application suspended");
      application_paused = true;
    }
    Event::Resumed => {
      debug!("Application resumed");
      application_paused = false;
    }
    Event::WindowEvent { event, .. } => match event {
      WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
      WindowEvent::KeyboardInput { input, .. } => match input {
        KeyboardInput {
          virtual_keycode,
          state,
          ..
        } => {
          // returns true if program needs to terminate
          if app.handle_key_event(virtual_keycode, state) {
            *control_flow = ControlFlow::Exit
          }
        }
      },
      WindowEvent::Resized(dimensions) => {
        info!("Window resized");
        if dimensions.width == 0 && dimensions.height == 0 {
          // the window has been minimized
          application_paused = true;
        } else {
          application_paused = false;
          app.handle_window_resize();
        }
      }
      WindowEvent::CursorMoved { position, .. } => {
        app.handle_cursor_moved(position);
      }
      WindowEvent::MouseWheel { delta, .. } => {
        if let MouseScrollDelta::LineDelta(_, y) = delta {
          app.handle_mouse_wheel(y);
        }
      }
      WindowEvent::CursorLeft { .. } => {
        app.handle_cursor_left_window();
      }
      WindowEvent::CursorEntered { .. } => {
        app.handle_cursor_entered_window();
      }
      _ => {}
    },
    Event::MainEventsCleared => {
      app.request_redraw();
    }
    Event::RedrawRequested(_window_id) => {
      if !application_paused {
        let now = Instant::now();
        app.render_next_frame(now - last_frame_instant);
        last_frame_instant = now;
      }
    }
    _ => (),
  })
}

fn main() {
  env_logger::init();
  let event_loop = EventLoop::new();
  let app = App::new(&event_loop);
  main_loop(event_loop, app);
}
