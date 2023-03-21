use cgmath::{Euler, Matrix4, Point3, Rad};

use crate::render::{Models, Renderable3dObject};

pub trait RenderableIn3d {
  fn obj(&self) -> &Renderable3dObject;
  fn into_obj(self) -> Renderable3dObject;
}

pub struct Square {
  render: Renderable3dObject,
}

impl Square {
  pub const MODEL_INDEX: usize = Models::SQUARE_INDEX;

  pub fn from_full(position: Point3<f32>, rotation: Euler<Rad<f32>>, scale: f32) -> Self {
    Self {
      render: Renderable3dObject::from_full(position, rotation, scale),
    }
  }

  pub fn model(&self) -> &Matrix4<f32> {
    self.render.model()
  }
}

impl RenderableIn3d for Square {
  fn obj(&self) -> &Renderable3dObject {
    &self.render
  }

  fn into_obj(self) -> Renderable3dObject {
    self.render
  }
}

pub struct Cube {
  render: Renderable3dObject,
}

impl Cube {
  pub const MODEL_INDEX: usize = 1;
}

impl RenderableIn3d for Cube {
  fn obj(&self) -> &Renderable3dObject {
    &self.render
  }

  fn into_obj(self) -> Renderable3dObject {
    self.render
  }
}
