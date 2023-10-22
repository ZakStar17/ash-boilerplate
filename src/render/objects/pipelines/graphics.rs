use std::ptr;

use ash::vk;

use crate::render::{
  objects::{
    vertices::{
      enumerate_attribute_descriptions, enumerate_binding_descriptions,
      get_pipeline_vertex_input_state_ci,
    },
    ColorVertex, Vertex,
  },
  shaders::{self, GraphicsShader},
  MatrixInstance,
};

use super::get_no_multisample_state_ci;

pub struct GraphicsPipelines {
  pub layout: vk::PipelineLayout,
  pub main: vk::Pipeline,
}

impl GraphicsPipelines {
  pub fn create(
    device: &ash::Device,
    swapchain_extent: vk::Extent2D,
    render_pass: vk::RenderPass,
  ) -> Self {
    // Note: for some reason rust distinguishes between "_" and "_named" for variables that are created
    // but not used. In my tests, it seems that "_" are dropped right away while "_named" stay until the
    // end of scope. This is really important because most creation info have pointers (not references) to
    // other objects which should only be destroyed after these pointers get used (as to not make them dangling)

    let mut shader = shaders::plain::Shader::load(device);
    let (shader_stages, _shader_func_name) = shader.get_pipeline_shader_creation_info();

    let (vertex_input_state_ci, _binding_descriptions, _attribute_descriptions) =
      get_pipeline_vertex_input_state_ci!(ColorVertex, MatrixInstance,);
    let input_assembly_state_ci = super::get_default_input_assembly_state_ci();

    let (viewport_state_ci, _viewport, _scissor) = super::get_viewport_state_ci(swapchain_extent);
    let rasterization_state_ci = super::get_no_depth_rasterization_state_ci();
    let multisample_state_ci = get_no_multisample_state_ci();

    let (color_blend_state_ci, _color_blend_attachment) =
      super::get_no_blend_color_blend_state_ci();

    let layout_create_info = vk::PipelineLayoutCreateInfo {
      s_type: vk::StructureType::PIPELINE_LAYOUT_CREATE_INFO,
      p_next: ptr::null(),
      flags: vk::PipelineLayoutCreateFlags::empty(),
      set_layout_count: 0,
      p_set_layouts: ptr::null(),
      push_constant_range_count: 0,
      p_push_constant_ranges: ptr::null(),
    };

    let layout = unsafe {
      device
        .create_pipeline_layout(&layout_create_info, None)
        .expect("Failed to create pipeline layout")
    };

    let create_infos = [vk::GraphicsPipelineCreateInfo {
      s_type: vk::StructureType::GRAPHICS_PIPELINE_CREATE_INFO,
      p_next: ptr::null(),
      flags: vk::PipelineCreateFlags::empty(),
      stage_count: shader_stages.len() as u32,
      p_stages: shader_stages.as_ptr(),
      p_vertex_input_state: &vertex_input_state_ci,
      p_input_assembly_state: &input_assembly_state_ci,
      p_tessellation_state: ptr::null(),
      p_viewport_state: &viewport_state_ci,
      p_rasterization_state: &rasterization_state_ci,
      p_multisample_state: &multisample_state_ci,
      p_depth_stencil_state: ptr::null(),
      p_color_blend_state: &color_blend_state_ci,
      p_dynamic_state: ptr::null(),
      layout,
      render_pass,
      subpass: 0,
      base_pipeline_handle: vk::Pipeline::null(),
      base_pipeline_index: -1,
    }];

    let pipelines = unsafe {
      device
        .create_graphics_pipelines(vk::PipelineCache::null(), &create_infos, None)
        .expect("Failed to create graphics pipelines")
    };

    unsafe {
      shader.destroy_self(device);
    }

    Self {
      layout,
      main: pipelines[0],
    }
  }

  pub unsafe fn destroy_self(&mut self, device: &ash::Device) {
    device.destroy_pipeline(self.main, None);
    device.destroy_pipeline_layout(self.layout, None);
  }
}
