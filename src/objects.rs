use cgmath::{Euler, Point3, Rad};

use crate::render::{
  ColorModelIndex, ColorModeled, ColorModels, Renderable3dObject, RenderableIn3d, TexModelIndex,
  TexModeled, TexModels,
};

// probably all of these will be just an object with a changeable model index
pub struct Square {
  render: Renderable3dObject,
}

impl Square {
  pub const MODEL_INDEX: ColorModelIndex = ColorModels::SQUARE_INDEX;

  pub fn new(position: Point3<f32>) -> Self {
    Self {
      render: Renderable3dObject::new(position),
    }
  }
}

impl RenderableIn3d for Square {
  fn ren(&self) -> &Renderable3dObject {
    &self.render
  }

  fn into_ren(self) -> Renderable3dObject {
    self.render
  }

  fn ren_mut(&mut self) -> &mut Renderable3dObject {
    &mut self.render
  }
}

impl ColorModeled for Square {
  fn model_i() -> ColorModelIndex {
    Self::MODEL_INDEX
  }
}

pub struct Cube {
  pub render: Renderable3dObject,
}

impl Cube {
  const COLOR_MODEL_INDEX: ColorModelIndex = ColorModels::CUBE_INDEX;
  const TEX_MODEL_INDEX: TexModelIndex = TexModels::BOX_INDEX;

  pub fn new(position: Point3<f32>) -> Self {
    Self {
      render: Renderable3dObject::new(position),
    }
  }
}

impl RenderableIn3d for Cube {
  fn ren(&self) -> &Renderable3dObject {
    &self.render
  }

  fn into_ren(self) -> Renderable3dObject {
    self.render
  }

  fn ren_mut(&mut self) -> &mut Renderable3dObject {
    &mut self.render
  }
}

impl ColorModeled for Cube {
  fn model_i() -> ColorModelIndex {
    Self::COLOR_MODEL_INDEX
  }
}

impl TexModeled for Cube {
  fn model_i() -> TexModelIndex {
    Self::TEX_MODEL_INDEX
  }
}

pub struct Niko {
  pub render: Renderable3dObject,
}

impl Niko {
  pub const MODEL_INDEX: TexModelIndex = TexModels::NIKO_INDEX;

  pub fn from_full(position: Point3<f32>, rotation: Euler<Rad<f32>>, scale: f32) -> Self {
    Self {
      render: Renderable3dObject::from_full(position, rotation, scale),
    }
  }
}

impl RenderableIn3d for Niko {
  fn ren(&self) -> &Renderable3dObject {
    &self.render
  }

  fn into_ren(self) -> Renderable3dObject {
    self.render
  }

  fn ren_mut(&mut self) -> &mut Renderable3dObject {
    &mut self.render
  }
}

impl TexModeled for Niko {
  fn model_i() -> TexModelIndex {
    Self::MODEL_INDEX
  }
}
