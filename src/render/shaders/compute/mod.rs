use std::path::Path;

use ash::vk;

use super::load_shader;

const INSTANCE_SHADER_PATH: &'static str = "./assets/shaders/compute/instance.spv";

pub struct ComputeShaders {
  pub instance: vk::ShaderModule,
}

impl ComputeShaders {
  pub fn load(device: &ash::Device) -> Self {
    Self {
      instance: load_shader(device, Path::new(INSTANCE_SHADER_PATH)),
    }
  }

  pub unsafe fn destroy_self(&mut self, device: &ash::Device) {
    device.destroy_shader_module(self.instance, None);
  }
}
