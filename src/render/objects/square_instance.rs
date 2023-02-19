use ash::vk;
use memoffset::offset_of;

#[repr(C)]
#[derive(Debug, Clone)]
pub struct SquareInstance {
  pub pos: [f32; 2],
  pub size: f32,
  // see std430 layout rules
  // https://www.oreilly.com/library/view/opengl-programming-guide/9780132748445/app09lev1sec3.html
  pub _padding: f32,
}

impl SquareInstance {
  pub fn new(pos: [f32; 2], size: f32) -> Self {
    Self {
      pos,
      size,
      _padding: 0.0,
    }
  }

  pub fn get_binding_description(binding: u32) -> vk::VertexInputBindingDescription {
    vk::VertexInputBindingDescription {
      binding,
      stride: std::mem::size_of::<Self>() as u32,
      input_rate: vk::VertexInputRate::INSTANCE,
    }
  }

  pub fn get_attribute_descriptions(
    start_location: u32,
    binding: u32,
  ) -> Vec<vk::VertexInputAttributeDescription> {
    vec![
      vk::VertexInputAttributeDescription {
        location: start_location,
        binding,
        format: vk::Format::R32G32_SFLOAT,
        offset: offset_of!(Self, pos) as u32,
      },
      vk::VertexInputAttributeDescription {
        location: start_location + 1,
        binding,
        format: vk::Format::R32_SFLOAT,
        offset: offset_of!(Self, size) as u32,
      },
    ]
  }
}
