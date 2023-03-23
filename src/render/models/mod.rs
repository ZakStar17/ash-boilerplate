use crate::structures::Linear2dVec;

use super::objects::Vertex;

mod cube;
mod niko;
mod weird_square;

trait Model {
  fn load() -> (Vec<Vertex>, Vec<u16>);
}

#[derive(Debug)]
pub struct ModelProperties {
  pub vertex_count: u32,
  pub vertex_offset: i32,
  pub index_count: u32,
  pub index_offset: u32,
}

pub struct Models {
  pub vertices: Linear2dVec<Vertex>,
  pub indices: Linear2dVec<u16>,
}

impl Models {
  pub const SQUARE_INDEX: usize = 0;
  pub const CUBE_INDEX: usize = 1;
  pub const NIKO_INDEX: usize = 2;

  pub fn load() -> Self {
    let data = [
      weird_square::WeirdSquare::load(),
      cube::Cube::load(),
      niko::Niko::load(),
    ];
    let (vertices, indices) = data.into_iter().unzip();
    let vertices = Self::load_vertices(vertices);
    let indices = Self::load_indices(indices);
    Self { vertices, indices }
  }

  fn load_vertices(vertices: Vec<Vec<Vertex>>) -> Linear2dVec<Vertex> {
    let mut iter = vertices.into_iter();
    let iter: &mut dyn ExactSizeIterator<Item = Vec<Vertex>> = &mut iter;
    Linear2dVec::from(iter)
  }

  fn load_indices(indices: Vec<Vec<u16>>) -> Linear2dVec<u16> {
    let mut iter = indices.into_iter();
    let iter: &mut dyn ExactSizeIterator<Item = Vec<u16>> = &mut iter;
    Linear2dVec::from(iter)
  }

  pub fn into_properties(self) -> Vec<ModelProperties> {
    self
      .vertices
      .into_parts_iter()
      .zip(self.indices.into_parts_iter())
      .map(|(vertex_p, index_p)| ModelProperties {
        vertex_count: vertex_p.size as u32,
        vertex_offset: vertex_p.offset as i32,
        index_count: index_p.size as u32,
        index_offset: index_p.offset as u32,
      })
      .collect()
  }
}
