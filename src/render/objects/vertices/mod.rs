mod color;
mod instance;
mod tex;

use ash::vk;
pub use color::ColorVertex;
pub use instance::MatrixInstance;
pub use tex::TexVertex;

pub trait Vertex {
  fn get_binding_description(binding: u32) -> vk::VertexInputBindingDescription;
  fn get_attribute_descriptions(
    start_location: u32,
    binding: u32,
  ) -> Vec<vk::VertexInputAttributeDescription>;
  fn attribute_size() -> u32;
}

// transforms
// enumerate_arr!(ty1, ty2, ty3);
// into
// [ty::get_binding_description(0), ty::get_binding_description(1), ty::get_binding_description(2)]
// https://stackoverflow.com/questions/32817193/how-to-get-index-of-macro-repetition-single-element
// https://stackoverflow.com/questions/31195529/escaping-commas-in-macro-output
macro_rules! enumerate_binding_descriptions {
    (@out $($out:expr,)* @step $_i:expr,) => {
        [$($out,)*]
    };
    (@out $($out:expr,)* @step $i:expr, $head:tt, $($tail:tt,)*) => {
        enumerate_binding_descriptions!(@out $($out,)* $head::get_binding_description($i), @step $i + 1u32, $($tail,)*)
    };
    ($($vertices:tt,)+) => {
        enumerate_binding_descriptions!(@out @step 0u32, $($vertices,)+)
    }
}

pub(crate) use enumerate_binding_descriptions;

macro_rules! enumerate_attribute_descriptions {
    (@out $($out:expr,)* @step $_i:expr, $_offset:expr, @prev) => {
        [$($out,)*]
    };
    (@out $($out:expr,)* @step $i:expr, $offset:expr, @prev $head:tt, $($tail:tt,)*) => {
        enumerate_attribute_descriptions!(
          @out $($out,)* $head::get_attribute_descriptions($offset, $i), @step $i + 1u32, $head::attribute_size(), @prev $($tail,)*
        )
    };
    ($($vertices:tt,)+) => {
        enumerate_attribute_descriptions!(@out @step 0u32, 0u32, @prev $($vertices,)+)
    }
}

pub(crate) use enumerate_attribute_descriptions;

macro_rules! get_pipeline_vertex_input_state_ci {
  ($($vertices:tt,)+) => {
    {
      let binding_descriptions = Box::pin(enumerate_binding_descriptions!($($vertices,)+));
      let attribute_descriptions: Vec<vk::VertexInputAttributeDescription> =
        enumerate_attribute_descriptions!($($vertices,)+)
        .into_iter()
        .flatten()
        .collect();
      let info = vk::PipelineVertexInputStateCreateInfo {
        s_type: vk::StructureType::PIPELINE_VERTEX_INPUT_STATE_CREATE_INFO,
        p_next: ptr::null(),
        flags: vk::PipelineVertexInputStateCreateFlags::empty(),
        vertex_attribute_description_count: attribute_descriptions.len() as u32,
        p_vertex_attribute_descriptions: attribute_descriptions.as_ptr(),
        vertex_binding_description_count: binding_descriptions.len() as u32,
        p_vertex_binding_descriptions: binding_descriptions.as_ptr(),
      };
      (info, binding_descriptions, attribute_descriptions)
    }
  };
}

pub(crate) use get_pipeline_vertex_input_state_ci;
