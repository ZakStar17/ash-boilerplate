use cgmath::Point3;

use crate::{
  objects::{Cube, Square},
  render::{ColorModelIndex, ColorModeled, RenderableIn3d, TexModelIndex, TexModeled},
  structures::Linear2dVec,
};

pub struct StaticScene {
  pub color_obj_count: usize,
  pub tex_obj_count: usize,
  // color shader
  pub squares: Vec<Square>,
  pub cubes: Vec<Cube>,
  // texture shader
  pub boxes: Vec<Cube>,
}

impl StaticScene {
  pub fn load() -> Self {
    let mut squares = vec![Square::new(Point3::new(5.0, 5.0, 5.0))];
    squares[0].ren_mut().set_scale(3.0);
    let cubes = vec![Cube::new(Point3::new(3.0, 5.0, 5.0))];
    let boxes = vec![Cube::new(Point3::new(3.0, 8.0, 6.0))];
    Self {
      color_obj_count: squares.len() + cubes.len(),
      tex_obj_count: boxes.len(),
      squares,
      cubes,
      boxes,
    }
  }

  pub fn objects<'a>(
    &'a self,
  ) -> (
    Linear2dVec<&'a dyn RenderableIn3d>,
    (Vec<ColorModelIndex>, Vec<TexModelIndex>),
  ) {
    let squares: Vec<&'a (dyn RenderableIn3d)> = self
      .squares
      .iter()
      .map(|x| {
        let result: &'a dyn RenderableIn3d = x;
        result
      })
      .collect();
    let cubes: Vec<&'a dyn RenderableIn3d> = self
      .cubes
      .iter()
      .map(|x| {
        let result: &'a dyn RenderableIn3d = x;
        result
      })
      .collect();
    let boxes: Vec<&'a dyn RenderableIn3d> = self
      .boxes
      .iter()
      .map(|x| {
        let result: &'a dyn RenderableIn3d = x;
        result
      })
      .collect();

    let all = [squares, cubes, boxes];
    // should correspond to the above
    let indices = (
      vec![Square::model_i(), <Cube as ColorModeled>::model_i()],
      vec![<Cube as TexModeled>::model_i()],
    );
    let mut iter = all.into_iter();
    let iter: &mut dyn ExactSizeIterator<Item = Vec<&'a dyn RenderableIn3d>> = &mut iter;
    (Linear2dVec::from(iter), indices)
  }
}
