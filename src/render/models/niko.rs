use std::{fs::File, io::BufReader, time::Instant};

use log::info;
use obj::{load_obj, Obj};

use crate::render::objects::TexVertex;

use super::TexModel;

pub struct Niko {}

impl TexModel for Niko {
  fn load() -> (Vec<TexVertex>, Vec<u16>) {
    let start = Instant::now();
    let input = BufReader::new(
      File::open("./assets/models/niko/niko.obj").expect("Failed to open Niko model file"),
    );
    let obj: Obj<obj::TexturedVertex, u16> = load_obj(input).expect("Failed to load Niko model");
    let vertices = obj
      .vertices
      .into_iter()
      .map(|v| TexVertex {
        pos: v.position,
        normal: v.normal,
        tex: [v.texture[0], v.texture[1]],
      })
      .collect();
    info!("Loaded Niko model in {:?}", start.elapsed());
    (vertices, obj.indices)
  }

  fn tex_index() -> usize {
    0
  }
}
