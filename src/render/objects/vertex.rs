use ash::vk;
use memoffset::offset_of;

#[repr(C)]
#[derive(Debug, Clone)]
pub struct Vertex {
  pub pos: [f32; 2],
  pub color: [f32; 3],
}

impl Vertex {
  pub fn get_binding_description(binding: u32) -> vk::VertexInputBindingDescription {
    vk::VertexInputBindingDescription {
      binding,
      stride: std::mem::size_of::<Self>() as u32,
      input_rate: vk::VertexInputRate::VERTEX,
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
        format: vk::Format::R32G32B32_SFLOAT,
        offset: offset_of!(Self, color) as u32,
      },
    ]
  }
}
