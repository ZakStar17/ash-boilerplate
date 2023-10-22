use std::{fs::File, time::Instant};

use image::{codecs::png::PngDecoder, ImageDecoder};
use log::info;

pub struct NikoTexture;

impl NikoTexture {
  pub fn load() -> Vec<u8> {
    let start = Instant::now();
    let texture_file =
      File::open("./assets/models/niko/tex/baked.png").expect("Failed to open Niko texture file");
    let texture = PngDecoder::new(texture_file).expect("Failed to decode Niko texture");
    let size = texture.total_bytes();
    assert!(size < u64::MAX);
    let mut texture_bytes = Vec::with_capacity(size as usize);
    texture
      .read_image(&mut texture_bytes)
      .expect("Failed to read Niko texture");
    info!("Loaded Niko texture in {:?}", start.elapsed());

    texture_bytes
  }
}
