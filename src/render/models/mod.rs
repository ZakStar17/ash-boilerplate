use crate::structures::Linear2dVec;

use super::objects::Vertex;

mod cube;
mod weird_square;

trait Model {
  fn get_vertices() -> Vec<Vertex>;
  fn get_indices() -> Vec<u16>;
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

  pub fn load() -> Self {
    let vertices = Self::load_vertices();
    let indices = Self::load_indices();
    Self { vertices, indices }
  }

  fn load_vertices() -> Linear2dVec<Vertex> {
    let vertices = [
      weird_square::WeirdSquare::get_vertices(),
      cube::Cube::get_vertices(),
    ];

    let mut iter = vertices.into_iter();
    let iter: &mut dyn ExactSizeIterator<Item = Vec<Vertex>> = &mut iter;
    Linear2dVec::from(iter)
  }

  fn load_indices() -> Linear2dVec<u16> {
    let indices = [
      weird_square::WeirdSquare::get_indices(),
      cube::Cube::get_indices(),
    ];

    let mut iter = indices.into_iter();
    let iter: &mut dyn ExactSizeIterator<Item = Vec<u16>> = &mut iter;
    Linear2dVec::from(iter)
  }

  pub fn get_property(&self, i: usize) -> ModelProperties {
    let (vertex_count, vertex_offset) = self.vertices.part(i).deconstruct();
    let (index_count, index_offset) = self.indices.part(i).deconstruct();
    ModelProperties {
      vertex_count: vertex_count as u32,
      vertex_offset: vertex_offset as i32,
      index_count: index_count as u32,
      index_offset: index_offset as u32,
    }
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
