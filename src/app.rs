use std::time::Duration;

use cgmath::{Euler, Point3, Rad};
use rand::Rng;
use winit::{
  dpi::PhysicalPosition,
  event::{ElementState, VirtualKeyCode},
  event_loop::EventLoop,
};

use crate::{
  keys::{Keys, Pressed},
  objects::Niko,
  render::{Camera, RenderableIn3d, SyncRender},
};

const MAX_DYN_OBJ_AMOUNT: usize = 16;

const CAMERA_NORMAL_SPEED: f32 = 2.0;
const CAMERA_FAST_SPEED: f32 = 10.0;

pub struct App {
  render: SyncRender,
  objects: Vec<Niko>,
  keys: Keys,
}

impl App {
  pub fn new(event_loop: &EventLoop<()>) -> Self {
    let objects = vec![Niko::from_full(
      Point3::new(-0.5, 0.0, 0.5),
      Euler::new(Rad(0.0), Rad(0.0), Rad(0.0)),
      0.1,
    )];
    let camera = Camera::new(CAMERA_NORMAL_SPEED);
    let render = SyncRender::initialize(event_loop, camera, MAX_DYN_OBJ_AMOUNT as u64);

    Self {
      render,
      objects,
      keys: Keys::new(),
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
          // all just a placeholder for now
          // TODO: implement dynamic allocation buffers
          if self.objects.len() < MAX_DYN_OBJ_AMOUNT {
            let mut rng = rand::thread_rng();
            let size = rng.gen::<f32>() * 0.2 + 0.05;
            let pos =
              self.render.camera.position() + (self.render.camera.front() * (2.0 + (size * 5.0)));
            self.objects.push(Niko::from_full(
              pos,
              Euler::new(Rad(0.0), Rad(0.0), Rad(0.0)),
              size,
            ));
          } else {
            println!("Max dynamic object amount reached");
          }
        }
        (VirtualKeyCode::C, ElementState::Pressed) => {
          self.render.toggle_cursor_grab();
        }
        (VirtualKeyCode::LControl, ElementState::Pressed) => {
          let speed = self.render.camera.speed_mut();
          *speed = CAMERA_FAST_SPEED;
        }
        (VirtualKeyCode::LControl, ElementState::Released) => {
          let speed = self.render.camera.speed_mut();
          *speed = CAMERA_NORMAL_SPEED;
        }
        _ => {}
      };
    }
    false
  }

  pub fn handle_cursor_moved(&mut self, position: PhysicalPosition<f64>) {
    self.render.handle_cursor_moved(position)
  }

  pub fn handle_mouse_wheel(&mut self, delta: f32) {
    self.render.handle_mouse_wheel(delta)
  }

  pub fn handle_cursor_left_window(&mut self) {
    self.render.handle_cursor_left_window()
  }

  pub fn handle_cursor_entered_window(&mut self) {
    self.render.handle_cursor_entered_window()
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
        self.render.camera.move_left(&duration_since_last_frame)
      } else {
        self.render.camera.move_right(&duration_since_last_frame)
      }
    }
    if self.keys.w ^ self.keys.s {
      if self.keys.w == Pressed {
        self.render.camera.move_forward(&duration_since_last_frame)
      } else {
        self
          .render
          .camera
          .move_backwards(&duration_since_last_frame)
      }
    }
    if self.keys.space ^ self.keys.l_shift {
      if self.keys.space == Pressed {
        self.render.camera.move_up(&duration_since_last_frame)
      } else {
        self.render.camera.move_down(&duration_since_last_frame)
      }
    }
    for obj in self.objects.iter_mut() {
      let ren = obj.ren_mut();
      let rot = ren.rotation();
      let mut rng = rand::thread_rng();
      ren.rotate(Euler {
        x: rot.x + Rad(rng.gen::<f32>() / 100.0),
        y: rot.y + Rad(rng.gen::<f32>() / 100.0),
        z: rot.z + Rad(rng.gen::<f32>() / 100.0),
      });
    }

    self
      .render
      .render_next_frame(&duration_since_last_frame, &self.objects);
  }
}
