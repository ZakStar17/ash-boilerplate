use crate::structures::{Linear2dVec, Partition};

use self::niko::NikoTexture;

mod niko;

pub struct Textures {
  bytes: Linear2dVec<u8>,
}

impl Textures {
  pub const NIKO_INDEX: usize = 0;

  pub fn load() -> Self {
    let niko = NikoTexture::load();

    let textures = vec![niko];
    let bytes = Linear2dVec::from(textures);

    Textures { bytes }
  }

  pub fn part(&self, i: usize) -> Partition<usize> {
    self.bytes.part(i)
  }
}
