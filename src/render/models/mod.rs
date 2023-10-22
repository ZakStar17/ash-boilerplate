use crate::structures::Linear2dVec;

use super::{
  objects::{ColorVertex, TexVertex},
  textures::Textures,
};

mod modeled;

pub use modeled::{ColorModelIndex, ColorModeled, TexModelIndex, TexModeled};

mod r#box;
mod cube;
mod niko;
mod weird_square;

use niko::Niko;
use r#box::Box;

// This probably will be simplified by some macros
trait ColorModel {
  fn load() -> (Vec<ColorVertex>, Vec<u16>);
}

trait TexModel {
  fn load() -> (Vec<TexVertex>, Vec<u16>);
  fn tex_index() -> usize;
}

#[derive(Debug)]
pub struct ColorModelProperties {
  pub vertex_count: u32,
  pub vertex_offset: i32,
  pub index_count: u32,
  pub index_offset: u32,
}

pub struct ColorModels {
  pub vertices: Linear2dVec<ColorVertex>,
  pub indices: Linear2dVec<u16>,
}

impl ColorModels {
  // indexes equal to order of data loading
  pub const SQUARE_INDEX: ColorModelIndex = ColorModelIndex(0);
  pub const CUBE_INDEX: ColorModelIndex = ColorModelIndex(1);

  pub fn load() -> Self {
    let data = [weird_square::WeirdSquare::load(), cube::Cube::load()];
    let (vertices, indices): (Vec<Vec<ColorVertex>>, Vec<Vec<u16>>) = data.into_iter().unzip();
    let vertices = Linear2dVec::from(vertices);
    let indices = Linear2dVec::from(indices);
    Self { vertices, indices }
  }

  pub fn into_properties(self) -> Vec<ColorModelProperties> {
    self
      .vertices
      .into_parts_iter()
      .zip(self.indices.into_parts_iter())
      .map(|(vertex_p, index_p)| ColorModelProperties {
        vertex_count: vertex_p.size as u32,
        vertex_offset: vertex_p.offset as i32,
        index_count: index_p.size as u32,
        index_offset: index_p.offset as u32,
      })
      .collect()
  }
}

pub struct TexModelProperties {
  pub vertex_count: u32,
  pub vertex_offset: i32,
  pub index_count: u32,
  pub index_offset: u32,
  pub tex_size: u64,
  pub tex_offset: u64,
}
pub struct TexModels {
  pub vertices: Linear2dVec<TexVertex>,
  pub indices: Linear2dVec<u16>,
}

impl TexModels {
  pub const NIKO_INDEX: TexModelIndex = TexModelIndex(0);
  pub const BOX_INDEX: TexModelIndex = TexModelIndex(1);

  pub fn load() -> Self {
    let data = [niko::Niko::load()];
    let (vertices, indices): (Vec<Vec<TexVertex>>, Vec<Vec<u16>>) = data.into_iter().unzip();
    let vertices = Linear2dVec::from(vertices);
    let indices = Linear2dVec::from(indices);
    Self { vertices, indices }
  }

  pub fn into_properties(self, textures: &Textures) -> Vec<TexModelProperties> {
    itertools::izip!(
      self.vertices.into_parts_iter(),
      self.indices.into_parts_iter(),
      [Niko::tex_index(), r#Box::tex_index()]
    )
    .map(|(vertex_p, index_p, tex_i)| {
      let tex_p = textures.part(tex_i);
      TexModelProperties {
        vertex_count: vertex_p.size as u32,
        vertex_offset: vertex_p.offset as i32,
        index_count: index_p.size as u32,
        index_offset: index_p.offset as u32,
        tex_size: tex_p.size as u64,
        tex_offset: tex_p.offset as u64,
      }
    })
    .collect()
  }
}

pub struct ModelProperties {
  pub color: Vec<ColorModelProperties>,
  pub tex: Vec<TexModelProperties>,
}

pub struct Models {
  pub color: ColorModels,
  pub tex: TexModels,
}

impl Models {
  pub fn load() -> Self {
    Models {
      color: ColorModels::load(),
      tex: TexModels::load(),
    }
  }

  pub fn into_properties(self, textures: &Textures) -> ModelProperties {
    ModelProperties {
      color: self.color.into_properties(),
      tex: self.tex.into_properties(textures),
    }
  }
}
