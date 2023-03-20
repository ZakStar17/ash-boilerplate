use std::ops::Index;

use crate::structures::Linear2dVec;

use super::objects::Vertex;

mod cube;
mod weird_square;

trait Model {
  fn get_vertices() -> Vec<Vertex>;
  fn get_indices() -> Vec<u16>;
}

pub struct ModelInstance<'a> {
  pub vertices: &'a [Vertex],
  pub vertex_offset: usize,
  pub indices: &'a [u16],
  pub index_offset: usize,
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
    let indices = [weird_square::WeirdSquare::get_indices()];

    let mut iter = indices.into_iter();
    let iter: &mut dyn ExactSizeIterator<Item = Vec<u16>> = &mut iter;
    Linear2dVec::from(iter)
  }
}

impl<'a> Index<usize> for &'a Models {
  type Output = ModelInstance<'a>;

  fn index(&self, i: usize) -> &Self::Output {
    let vertices = &self.vertices[i];
    let vertex_offset = self.vertices.offset(i);
    let indices = &self.indices[i];
    let index_offset = self.indices.offset(i);
    &ModelInstance {
      vertices,
      vertex_offset,
      indices,
      index_offset,
    }
  }
}
