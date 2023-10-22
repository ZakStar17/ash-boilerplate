pub struct NikoTexture;
pub struct BoxTexture;

impl NikoTexture {
  pub fn load() -> Vec<u8> {
    super::load_texture("./assets/models/niko/tex/baked.png", "Niko")
  }
}

impl BoxTexture {
  pub fn load() -> Vec<u8> {
    super::load_texture("./assets/textures/box.png", "Box")
  }
}
