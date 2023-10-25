use std::ptr::{self};

use ash::vk;

use crate::render::{
  objects::{
    vertices::{
      enumerate_attribute_descriptions, enumerate_binding_descriptions,
      get_pipeline_vertex_input_state_ci, PipelineVertexInputStateCIOwned,
    },
    ColorVertex, TexVertex, Vertex,
  },
  shaders::{self, GraphicsShader},
  MatrixInstance,
};

use super::{
  get_no_multisample_state_ci, PipelineColorBlendStateCIOwned, PipelineViewportStateCIOwned,
};

pub struct GraphicsPipelines {
  pub layout: vk::PipelineLayout,
  pub color: vk::Pipeline,
  pub tex: vk::Pipeline,
}

impl GraphicsPipelines {
  pub fn create(
    device: &ash::Device,
    swapchain_extent: vk::Extent2D,
    render_pass: vk::RenderPass,
  ) -> Self {
    // empty layout
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

    let pipelines =
      Self::create_color_graphics_pipelines(device, layout, swapchain_extent, render_pass);

    Self {
      layout,
      color: pipelines[0],
      tex: pipelines[1],
    }
  }

  fn create_color_graphics_pipelines(
    device: &ash::Device,
    layout: vk::PipelineLayout,
    swapchain_extent: vk::Extent2D,
    render_pass: vk::RenderPass,
  ) -> Vec<vk::Pipeline> {
    let mut color = shaders::plain::Shader::load(device);
    let mut tex = shaders::tex_plain::Shader::load(device);
    let color_shader_stages = color.get_pipeline_shader_creation_info();
    let tex_shader_stages = tex.get_pipeline_shader_creation_info();

    let color_vertex_input_state =
      get_pipeline_vertex_input_state_ci!(ColorVertex, MatrixInstance,);
    let tex_vertex_input_state = get_pipeline_vertex_input_state_ci!(TexVertex, MatrixInstance,);

    let input_assembly_state = super::get_default_input_assembly_state_ci();

    let viewport_state = PipelineViewportStateCIOwned::new(swapchain_extent);
    let rasterization_state = super::get_no_depth_rasterization_state_ci();
    let multisample_state = get_no_multisample_state_ci();

    let color_blend_state = PipelineColorBlendStateCIOwned::no_blend_color();

    let default_ci = vk::GraphicsPipelineCreateInfo {
      s_type: vk::StructureType::GRAPHICS_PIPELINE_CREATE_INFO,
      p_next: ptr::null(),
      flags: vk::PipelineCreateFlags::empty(),
      stage_count: 0,                    // change
      p_stages: ptr::null(),             // change
      p_vertex_input_state: ptr::null(), // change
      p_input_assembly_state: &input_assembly_state,
      p_tessellation_state: ptr::null(),
      p_viewport_state: &viewport_state.ci,
      p_rasterization_state: &rasterization_state,
      p_multisample_state: &multisample_state,
      p_depth_stencil_state: ptr::null(),
      p_color_blend_state: &color_blend_state.ci,
      p_dynamic_state: ptr::null(),
      layout,
      render_pass,
      subpass: 0,
      base_pipeline_handle: vk::Pipeline::null(),
      base_pipeline_index: -1,
    };

    let mut color_ci = default_ci.clone();
    color_ci.stage_count = color_shader_stages.ci.len() as u32;
    color_ci.p_stages = color_shader_stages.ci.as_ptr();
    color_ci.p_vertex_input_state = &color_vertex_input_state.ci;

    let mut tex_ci = default_ci;
    tex_ci.stage_count = tex_shader_stages.ci.len() as u32;
    tex_ci.p_stages = tex_shader_stages.ci.as_ptr();
    tex_ci.p_vertex_input_state = &tex_vertex_input_state.ci;

    let create_infos = [color_ci, tex_ci];
    let pipelines = unsafe {
      device
        .create_graphics_pipelines(vk::PipelineCache::null(), &create_infos, None)
        .expect("Failed to create graphics pipelines")
    };

    unsafe {
      color.destroy_self(device);
      tex.destroy_self(device);
    }

    pipelines
  }

  pub unsafe fn destroy_self(&mut self, device: &ash::Device) {
    device.destroy_pipeline(self.color, None);
    device.destroy_pipeline(self.tex, None);
    device.destroy_pipeline_layout(self.layout, None);
  }
}
