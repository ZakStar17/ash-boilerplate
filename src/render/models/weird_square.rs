use crate::render::objects::ColorVertex;

use super::ColorModel;

pub struct WeirdSquare {}

impl WeirdSquare {
  const VERTICES: [ColorVertex; 4] = [
    ColorVertex {
      pos: [-0.5, -0.5, 0.0],
      color: [1.0, 0.0, 0.0],
    },
    ColorVertex {
      pos: [0.5, -0.5, 0.0],
      color: [0.0, 1.0, 0.0],
    },
    ColorVertex {
      pos: [0.5, 0.5, 0.0],
      color: [0.0, 0.0, 1.0],
    },
    ColorVertex {
      pos: [-1.0, 1.0, 0.0],
      color: [0.0, 0.0, 0.0],
    },
  ];

  const INDICES: [u16; 12] = [0, 1, 2, 2, 3, 0, 2, 1, 0, 0, 3, 2];
}

impl ColorModel for WeirdSquare {
  fn load() -> (Vec<ColorVertex>, Vec<u16>) {
    (Self::VERTICES.to_vec(), Self::INDICES.to_vec())
  }
}
