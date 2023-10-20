mod compute;
pub mod plain;
pub mod tex_plain;
pub use compute::ComputeShaders;

use ash::vk;
use std::{ffi::CString, path::Path, ptr};

pub trait GraphicsShader {
  fn get_vert(&self) -> vk::ShaderModule;
  fn get_frag(&self) -> vk::ShaderModule;

  fn get_pipeline_shader_creation_info(&self) -> ([vk::PipelineShaderStageCreateInfo; 2], CString) {
    // returned for lifetime reasons
    let main_function_name = CString::new("main").unwrap();
    (
      [
        vk::PipelineShaderStageCreateInfo {
          // Vertex shader
          s_type: vk::StructureType::PIPELINE_SHADER_STAGE_CREATE_INFO,
          p_next: ptr::null(),
          flags: vk::PipelineShaderStageCreateFlags::empty(),
          module: self.get_vert(),
          p_name: main_function_name.as_ptr(),
          p_specialization_info: ptr::null(),
          stage: vk::ShaderStageFlags::VERTEX,
        },
        vk::PipelineShaderStageCreateInfo {
          // Fragment shader
          s_type: vk::StructureType::PIPELINE_SHADER_STAGE_CREATE_INFO,
          p_next: ptr::null(),
          flags: vk::PipelineShaderStageCreateFlags::empty(),
          module: self.get_frag(),
          p_name: main_function_name.as_ptr(),
          p_specialization_info: ptr::null(),
          stage: vk::ShaderStageFlags::FRAGMENT,
        },
      ],
      main_function_name,
    )
  }
}

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
      .expect("Failed to create shader module")
  }
}
