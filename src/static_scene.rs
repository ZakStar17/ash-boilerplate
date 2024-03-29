use cgmath::Point3;

use crate::{
  objects::{Cube, Square},
  render::{Models, RenderableIn3d},
  structures::Linear2dVec,
};

pub struct StaticScene {
  pub total_obj_count: usize,
  pub squares: Vec<Square>,
  pub cubes: Vec<Cube>,
}

impl StaticScene {
  pub fn load() -> Self {
    let mut squares = vec![Square::new(Point3::new(5.0, 5.0, 5.0))];
    squares[0].ren_mut().set_scale(3.0);
    let cubes = vec![Cube::new(Point3::new(3.0, 5.0, 5.0))];
    Self {
      total_obj_count: squares.len() + cubes.len(),
      squares,
      cubes,
    }
  }

  pub fn objects<'a>(&'a self) -> (Linear2dVec<&'a dyn RenderableIn3d>, Vec<usize>) {
    let squares: Vec<&'a dyn RenderableIn3d> = self
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

    let all = [squares, cubes];
    // shoud correspond to the above
    let model_indices = vec![Models::SQUARE_INDEX, Models::CUBE_INDEX];
    let mut iter = all.into_iter();
    let iter: &mut dyn ExactSizeIterator<Item = Vec<&'a dyn RenderableIn3d>> = &mut iter;
    (Linear2dVec::from(iter), model_indices)
  }
}
