mod app;
mod render;

use std::time::Duration;

use app::App;
use log::{debug, info};
use winit::{
  event::{Event, KeyboardInput, WindowEvent},
  event_loop::{ControlFlow, EventLoop},
};

pub const WINDOW_TITLE: &'static str = "Ash boilerplate";
pub const INITIAL_WINDOW_WIDTH: u32 = 800;
pub const INITIAL_WINDOW_HEIGHT: u32 = 600;

pub const ENABLE_VSYNC: bool = false;

pub const PRINT_FPS: bool = true;
pub const FPS_PRINT_INTERVAL: Duration = Duration::from_millis(2000);

pub const PRINT_GPU_WAIT: bool = true;
pub const GPU_PRINT_INTERVAL: Duration = Duration::from_millis(5000);

pub fn main_loop(event_loop: EventLoop<()>, mut app: App) {
  let mut application_paused = false;
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
      _ => {}
    },
    Event::MainEventsCleared => {
      app.request_redraw();
    }
    Event::RedrawRequested(_window_id) => {
      if !application_paused {
        app.render_next_frame();
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
