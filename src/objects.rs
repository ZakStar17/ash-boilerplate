use crate::render::Renderable3dObject;

pub trait RenderableIn3d {
  fn obj(&self) -> &Renderable3dObject;
  fn into_obj(self) -> Renderable3dObject;
}

pub struct Square {
  render: Renderable3dObject,
}

impl Square {
  pub const MODEL_INDEX: usize = 0;
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
