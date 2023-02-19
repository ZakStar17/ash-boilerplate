use std::path::Path;

use ash::vk;

use super::load_shader;

const VERT_SHADER_PATH: &'static str = "./assets/shaders/triangle/vert.spv";
const FRAG_SHADER_PATH: &'static str = "./assets/shaders/triangle/frag.spv";

pub struct TriangleShader {
  pub vert: vk::ShaderModule,
  pub frag: vk::ShaderModule,
}

impl TriangleShader {
  pub fn load(device: &ash::Device) -> Self {
    Self {
      vert: load_shader(device, Path::new(VERT_SHADER_PATH)),
      frag: load_shader(device, Path::new(FRAG_SHADER_PATH)),
    }
  }

  pub unsafe fn destroy_self(&mut self, device: &ash::Device) {
    device.destroy_shader_module(self.vert, None);
    device.destroy_shader_module(self.frag, None);
  }
}
