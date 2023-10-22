use crate::render::RenderableIn3d;

pub struct ColorModelIndex(pub usize);
pub trait ColorModeled: RenderableIn3d {
  fn model_i() -> ColorModelIndex;
}

pub struct TexModelIndex(pub usize);
pub trait TexModeled: RenderableIn3d {
  fn model_i() -> TexModelIndex;
}
