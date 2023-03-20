use crate::{
  objects::{Cube, RenderableIn3d, Square},
  render::Models,
  structures::Linear2dVec,
};

pub struct StaticScene {
  pub total_obj_count: usize,
}

impl StaticScene {
  const SQUARES: [Square; 0] = [];
  const CUBES: [Cube; 0] = [];

  pub fn load() -> Self {
    Self {
      total_obj_count: Self::SQUARES.len() + Self::CUBES.len(),
    }
  }

  pub fn objects<'a>(&'a self) -> (Linear2dVec<&'a dyn RenderableIn3d>, Vec<usize>) {
    let squares: Vec<&'a dyn RenderableIn3d> = Self::SQUARES
      .iter()
      .map(|x| {
        let result: &'a dyn RenderableIn3d = x;
        result
      })
      .collect();
    let cubes: Vec<&'a dyn RenderableIn3d> = Self::CUBES
      .iter()
      .map(|x| {
        let result: &'a dyn RenderableIn3d = x;
        result
      })
      .collect();

    let all = [squares, cubes];
    // shoud correspond to the above
    let model_indices = vec![Models::SQUARE_INDEX, Models::CUBE_INDEX];
    let iter = all.into_iter();
    let iter: &mut dyn ExactSizeIterator<Item = Vec<&'a dyn RenderableIn3d>> = &mut iter;
    (Linear2dVec::from(iter), model_indices)
  }
}
