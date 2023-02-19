use ash::vk;
use std::{path::Path, ptr};

pub fn load_shader(device: &ash::Device, shader_path: &Path) -> vk::ShaderModule {
  let code = read_shader_code(shader_path);
  create_shader_module(device, code)
}

fn read_shader_code(shader_path: &Path) -> Vec<u8> {
  use std::{fs::File, io::Read};

  let spv_file =
    File::open(shader_path).expect(&format!("Failed to find spv file at {:?}", shader_path));
  let bytes_code: Vec<u8> = spv_file.bytes().filter_map(|byte| byte.ok()).collect();

  bytes_code
}

fn create_shader_module(device: &ash::Device, code: Vec<u8>) -> vk::ShaderModule {
  let shader_module_create_info = vk::ShaderModuleCreateInfo {
    s_type: vk::StructureType::SHADER_MODULE_CREATE_INFO,
    p_next: ptr::null(),
    flags: vk::ShaderModuleCreateFlags::empty(),
    code_size: code.len(),
    p_code: code.as_ptr() as *const u32,
  };

  unsafe {
    device
      .create_shader_module(&shader_module_create_info, None)
      .expect("Failed to create Shader Module!")
  }
}

mod compute;
mod triangle;
pub use compute::ComputeShaders;
pub use triangle::TriangleShader;
