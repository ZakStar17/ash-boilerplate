use ash::vk;
use cgmath::Matrix4;
use memoffset::offset_of;

use super::Vertex;

#[repr(C)]
#[derive(Debug, Clone)]
pub struct MatrixInstance {
  pub matrix: Matrix4<f32>,
  // see std430 layout rules
  // https://www.oreilly.com/library/view/opengl-programming-guide/9780132748445/app09lev1sec3.html
  pub _padding: (),
}

impl MatrixInstance {
  pub fn new(matrix: Matrix4<f32>) -> Self {
    Self {
      matrix,
      _padding: (),
    }
  }
}

impl Vertex for MatrixInstance {
  fn get_binding_description(binding: u32) -> vk::VertexInputBindingDescription {
    vk::VertexInputBindingDescription {
      binding,
      stride: std::mem::size_of::<Self>() as u32,
      input_rate: vk::VertexInputRate::INSTANCE,
    }
  }

  fn get_attribute_descriptions(
    start_location: u32,
    binding: u32,
  ) -> Vec<vk::VertexInputAttributeDescription> {
    vec![
      vk::VertexInputAttributeDescription {
        location: start_location,
        binding,
        format: vk::Format::R32G32B32A32_SFLOAT,
        offset: offset_of!(Self, matrix) as u32 + offset_of!(Matrix4<f32>, x) as u32,
      },
      vk::VertexInputAttributeDescription {
        location: start_location + 1,
        binding,
        format: vk::Format::R32G32B32A32_SFLOAT,
        offset: offset_of!(Self, matrix) as u32 + offset_of!(Matrix4<f32>, y) as u32,
      },
      vk::VertexInputAttributeDescription {
        location: start_location + 2,
        binding,
        format: vk::Format::R32G32B32A32_SFLOAT,
        offset: offset_of!(Self, matrix) as u32 + offset_of!(Matrix4<f32>, z) as u32,
      },
      vk::VertexInputAttributeDescription {
        location: start_location + 3,
        binding,
        format: vk::Format::R32G32B32A32_SFLOAT,
        offset: offset_of!(Self, matrix) as u32 + offset_of!(Matrix4<f32>, w) as u32,
      },
    ]
  }

  fn attribute_size() -> u32 {
    4
  }
}
