#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct CameraPos {
  pub pos: [f32; 2],
}

impl CameraPos {
  pub fn new(pos: [f32; 2]) -> Self {
    Self { pos }
  }
}
