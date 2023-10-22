use std::{fs::File, time::Instant};

use image::{codecs::png::PngDecoder, ImageDecoder};
use log::info;

use crate::structures::{Linear2dVec, Partition};

use self::textures::{BoxTexture, NikoTexture};

mod textures;

pub struct Textures {
  bytes: Linear2dVec<u8>,
}

impl Textures {
  pub const NIKO_INDEX: usize = 0;
  pub const BOX_INDEX: usize = 1;

  pub fn load() -> Self {
    let niko = NikoTexture::load();
    let box_ = BoxTexture::load();

    let textures = vec![niko, box_];
    let bytes = Linear2dVec::from(textures);

    Textures { bytes }
  }

  pub fn part(&self, i: usize) -> Partition<usize> {
    self.bytes.part(i)
  }
}

pub fn load_texture(loc: &'static str, name: &'static str) -> Vec<u8> {
  let start = Instant::now();
  let texture_file = File::open(loc).expect(&format!("Failed to open \"{}\" texture file", name));
  let texture =
    PngDecoder::new(texture_file).expect(&format!("Failed to decode \"{}\" texture", name));
  let size = texture.total_bytes();
  assert!(size < u64::MAX); // Big images (size == max) require a different approach
  let mut texture_bytes = Vec::with_capacity(size as usize);
  texture
    .read_image(&mut texture_bytes)
    .expect(&format!("Failed to read \"{}\" texture", name));
  info!("Loaded \"{}\" texture in {:?}", name, start.elapsed());

  texture_bytes
}
