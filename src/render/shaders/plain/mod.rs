use std::path::Path;

use ash::vk;

use super::{load_shader, GraphicsShader};

const VERT_SHADER_PATH: &'static str = "./assets/shaders/plain/vert.spv";
const FRAG_SHADER_PATH: &'static str = "./assets/shaders/plain/frag.spv";

pub struct Shader {
  vert: vk::ShaderModule,
  frag: vk::ShaderModule,
}

impl Shader {
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

impl GraphicsShader for Shader {
  fn get_frag(&self) -> vk::ShaderModule {
    self.frag
  }
  fn get_vert(&self) -> vk::ShaderModule {
    self.vert
  }
}
