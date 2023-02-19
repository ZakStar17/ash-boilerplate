use rand::Rng;
use winit::{
  event::{ElementState, VirtualKeyCode},
  event_loop::EventLoop,
};

use crate::render::{SquareInstance, SyncRender};

const MAX_SQUARE_AMMOUNT: usize = 8;

pub struct App {
  render: SyncRender,
  squares: Vec<SquareInstance>,
}

impl App {
  pub fn new(event_loop: &EventLoop<()>) -> Self {
    let squares = vec![SquareInstance::new([-0.5, -0.5], 0.2)];
    let render = SyncRender::initialize(event_loop, MAX_SQUARE_AMMOUNT as u64);

    Self { render, squares }
  }

  pub fn handle_key_event(&mut self, keycode: Option<VirtualKeyCode>, state: ElementState) -> bool {
    match (keycode, state) {
      (Some(VirtualKeyCode::Escape), ElementState::Pressed) => {
        return true;
      }
      (Some(VirtualKeyCode::Space), ElementState::Released) => {
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
    false
  }

  pub fn handle_window_resize(&mut self) {
    self.render.handle_window_resize()
  }

  pub fn request_redraw(&mut self) {
    self.render.request_redraw()
  }

  pub fn render_next_frame(&mut self) {
    self.render.render_next_frame(&self.squares);
  }
}
