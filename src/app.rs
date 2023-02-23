use std::time::Duration;

use rand::Rng;
use winit::{
  event::{ElementState, VirtualKeyCode},
  event_loop::EventLoop,
};

use crate::{
  keys::{Keys, Pressed},
  render::{SquareInstance, SyncRender, CameraPos},
};

const MAX_SQUARE_AMMOUNT: usize = 8;
const CAMERA_SPEED: f32 = 0.4;

pub struct App {
  render: SyncRender,
  squares: Vec<SquareInstance>,
  keys: Keys,
  camera_pos: CameraPos
}

impl App {
  pub fn new(event_loop: &EventLoop<()>) -> Self {
    let squares = vec![SquareInstance::new([-0.5, -0.5], 0.2)];
    let render = SyncRender::initialize(event_loop, MAX_SQUARE_AMMOUNT as u64);
    let camera_pos = CameraPos::new([0.0, 0.0]);

    Self {
      render,
      squares,
      keys: Keys::new(),
      camera_pos
    }
  }

  pub fn handle_key_event(&mut self, keycode: Option<VirtualKeyCode>, state: ElementState) -> bool {
    if let Some(code) = keycode {
      self.keys.update_from_event(code, state);
      match (code, state) {
        (VirtualKeyCode::Escape, ElementState::Pressed) => {
          return true;
        }
        (VirtualKeyCode::Q, ElementState::Released) => {
          if self.squares.len() < MAX_SQUARE_AMMOUNT {
            let mut rng = rand::thread_rng();
            self.squares.push(SquareInstance::new(
              [rng.gen::<f32>() - 0.5, rng.gen::<f32>() - 0.5],
              rng.gen::<f32>() * 0.5,
            ));
          } else {
            println!("Max square ammount reached");
          }
        }
        _ => {}
      };
    }
    false
  }

  pub fn handle_window_resize(&mut self) {
    self.render.handle_window_resize()
  }

  pub fn request_redraw(&mut self) {
    self.render.request_redraw()
  }

  pub fn render_next_frame(&mut self, duration_since_last_frame: Duration) {
    if self.keys.a ^ self.keys.d {
      if self.keys.a == Pressed {
        self.camera_pos.pos[0] -= CAMERA_SPEED * duration_since_last_frame.as_secs_f32();
      } else {
        self.camera_pos.pos[0] += CAMERA_SPEED * duration_since_last_frame.as_secs_f32();
      }
    }
    if self.keys.w ^ self.keys.s {
      if self.keys.w == Pressed {
        self.camera_pos.pos[1] -= CAMERA_SPEED * duration_since_last_frame.as_secs_f32();
      } else {
        self.camera_pos.pos[1] += CAMERA_SPEED * duration_since_last_frame.as_secs_f32();
      }
    }
    self
      .render
      .render_next_frame(&duration_since_last_frame, &self.squares, &self.camera_pos);
  }
}
