use cgmath::{Euler, Point3, Rad};

use crate::render::{Models, Renderable3dObject, RenderableIn3d};

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

pub struct Cube {
  pub render: Renderable3dObject,
}

impl Cube {
  pub const MODEL_INDEX: usize = 1;

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
