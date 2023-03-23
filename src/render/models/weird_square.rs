use crate::render::objects::Vertex;

use super::Model;

pub struct WeirdSquare {}

impl WeirdSquare {
  const VERTICES: [Vertex; 4] = [
    Vertex {
      pos: [-0.5, -0.5, 0.0],
      color: [1.0, 0.0, 0.0],
    },
    Vertex {
      pos: [0.5, -0.5, 0.0],
      color: [0.0, 1.0, 0.0],
    },
    Vertex {
      pos: [0.5, 0.5, 0.0],
      color: [0.0, 0.0, 1.0],
    },
    Vertex {
      pos: [-1.0, 1.0, 0.0],
      color: [0.0, 0.0, 0.0],
    },
  ];

  const INDICES: [u16; 12] = [0, 1, 2, 2, 3, 0, 2, 1, 0, 0, 3, 2];
}

impl Model for WeirdSquare {
  fn load() -> (Vec<Vertex>, Vec<u16>) {
    (Self::VERTICES.to_vec(), Self::INDICES.to_vec())
  }
}
