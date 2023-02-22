use std::ops::BitXor;

use winit::event::{ElementState, VirtualKeyCode};

/// State of each key
#[derive(PartialEq, Clone, Copy)]
pub enum KeyState {
  Pressed,
  Released,
}
pub use KeyState::{Pressed, Released};

impl Default for KeyState {
  fn default() -> Self {
    KeyState::Released
  }
}

impl BitXor for KeyState {
  type Output = bool;

  fn bitxor(self, rhs: Self) -> Self::Output {
    Into::<bool>::into(self) ^ Into::<bool>::into(rhs)
  }
}

impl From<ElementState> for KeyState {
  fn from(state: ElementState) -> Self {
    match state {
      ElementState::Pressed => Self::Pressed,
      ElementState::Released => Self::Released,
    }
  }
}

impl Into<bool> for KeyState {
  fn into(self) -> bool {
    match self {
      Self::Pressed => true,
      Self::Released => false,
    }
  }
}

/// struct that contains information about each pressed / not pressed key
#[derive(Default)]
pub struct Keys {
  pub a: KeyState,
  pub w: KeyState,
  pub s: KeyState,
  pub d: KeyState,
  pub space: KeyState,
  pub l_shift: KeyState,
  pub l_ctrl: KeyState,
  pub up_key: KeyState,
  pub down_key: KeyState,
  pub left_key: KeyState,
  pub right_key: KeyState,
}

impl Keys {
  pub fn new() -> Self {
    Self::default()
  }

  pub fn update_from_event(&mut self, code: VirtualKeyCode, state: ElementState) {
    let s = KeyState::from(state);
    match code {
      VirtualKeyCode::A => self.a = s,
      VirtualKeyCode::W => self.w = s,
      VirtualKeyCode::S => self.s = s,
      VirtualKeyCode::D => self.d = s,
      VirtualKeyCode::Space => self.space = s,
      VirtualKeyCode::LShift => self.l_shift = s,
      VirtualKeyCode::LControl => self.l_ctrl = s,
      VirtualKeyCode::Up => self.up_key = s,
      VirtualKeyCode::Down => self.down_key = s,
      VirtualKeyCode::Left => self.left_key = s,
      VirtualKeyCode::Right => self.right_key = s,
      _ => {}
    }
  }
}
