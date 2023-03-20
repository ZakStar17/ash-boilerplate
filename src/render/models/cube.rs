use crate::render::objects::Vertex;

use super::Model;

pub struct Cube {}

impl Cube {
  const VERTICES: [Vertex; 8] = [
    Vertex {
      pos: [-1.0, -1.0, -1.0],
      color: [1.0, 1.0, 1.0],
    },
    Vertex {
      pos: [1.0, -1.0, -1.0],
      color: [1.0, 1.0, 1.0],
    },
    Vertex {
      pos: [1.0, 1.0, -1.0],
      color: [1.0, 1.0, 1.0],
    },
    Vertex {
      pos: [-1.0, 1.0, -1.0],
      color: [1.0, 1.0, 1.0],
    },
    Vertex {
      pos: [-1.0, -1.0, 1.0],
      color: [1.0, 1.0, 1.0],
    },
    Vertex {
      pos: [1.0, -1.0, 1.0],
      color: [1.0, 1.0, 1.0],
    },
    Vertex {
      pos: [1.0, 1.0, 1.0],
      color: [1.0, 1.0, 1.0],
    },
    Vertex {
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
  fn get_vertices() -> Vec<Vertex> {
    Self::VERTICES.to_vec()
  }
  fn get_indices() -> Vec<u16> {
    Self::INDICES.to_vec()
  }
}
