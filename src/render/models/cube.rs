use crate::render::objects::ColorVertex;

use super::Model;

pub struct Cube {}

impl Cube {
  const VERTICES: [ColorVertex; 8] = [
    ColorVertex {
      pos: [-1.0, -1.0, -1.0],
      color: [1.0, 1.0, 1.0],
    },
    ColorVertex {
      pos: [1.0, -1.0, -1.0],
      color: [1.0, 1.0, 1.0],
    },
    ColorVertex {
      pos: [1.0, 1.0, -1.0],
      color: [1.0, 1.0, 1.0],
    },
    ColorVertex {
      pos: [-1.0, 1.0, -1.0],
      color: [1.0, 1.0, 1.0],
    },
    ColorVertex {
      pos: [-1.0, -1.0, 1.0],
      color: [1.0, 1.0, 1.0],
    },
    ColorVertex {
      pos: [1.0, -1.0, 1.0],
      color: [1.0, 1.0, 1.0],
    },
    ColorVertex {
      pos: [1.0, 1.0, 1.0],
      color: [1.0, 1.0, 1.0],
    },
    ColorVertex {
      pos: [-1.0, 1.0, 1.0],
      color: [1.0, 1.0, 1.0],
    },
  ];

  const INDICES: [u16; 36] = [
    0, 1, 3, 3, 1, 2, 1, 5, 2, 2, 5, 6, 5, 4, 6, 6, 4, 7, 4, 0, 7, 7, 0, 3, 3, 2, 7, 7, 2, 6, 4, 5,
    0, 0, 5, 1,
  ];
}

impl Model for Cube {
  fn load() -> (Vec<ColorVertex>, Vec<u16>) {
    (Self::VERTICES.to_vec(), Self::INDICES.to_vec())
  }
}
