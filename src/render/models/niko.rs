use std::{fs::File, io::BufReader, time::Instant};

use log::info;
use obj::{load_obj, Obj};

use crate::render::objects::ColorVertex;

use super::Model;

pub struct Niko {}

impl Model for Niko {
  fn load() -> (Vec<ColorVertex>, Vec<u16>) {
    let start = Instant::now();
    let input = BufReader::new(
      File::open("./assets/models/niko/niko.obj").expect("Failed to open custom model file"),
    );
    let obj: Obj<obj::TexturedVertex, u16> = load_obj(input).expect("Failed to load custom model");
    let vertices = obj
      .vertices
      .into_iter()
      .map(|v| ColorVertex {
        pos: v.position,
        color: v.texture,
      })
      .collect();
    info!("Loaded custom model in {:?}", start.elapsed());
    (vertices, obj.indices)
  }
}
