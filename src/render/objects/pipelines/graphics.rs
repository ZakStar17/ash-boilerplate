use std::{ffi::CString, ptr};

use ash::vk;

use crate::render::{
  objects::{SquareInstance, Vertex},
  shaders,
};

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
    let mut shader = shaders::plain::Shader::load(device);
    let main_function_name = CString::new("main").unwrap(); // the beginning function name in shader code.

    let shader_stages = [
      vk::PipelineShaderStageCreateInfo {
        // Vertex shader
        s_type: vk::StructureType::PIPELINE_SHADER_STAGE_CREATE_INFO,
        p_next: ptr::null(),
        flags: vk::PipelineShaderStageCreateFlags::empty(),
        module: shader.vert,
        p_name: main_function_name.as_ptr(),
        p_specialization_info: ptr::null(),
        stage: vk::ShaderStageFlags::VERTEX,
      },
      vk::PipelineShaderStageCreateInfo {
        // Fragment shader
        s_type: vk::StructureType::PIPELINE_SHADER_STAGE_CREATE_INFO,
        p_next: ptr::null(),
        flags: vk::PipelineShaderStageCreateFlags::empty(),
        module: shader.frag,
        p_name: main_function_name.as_ptr(),
        p_specialization_info: ptr::null(),
        stage: vk::ShaderStageFlags::FRAGMENT,
      },
    ];

    // convoluted for now
    let binding_descriptions = [
      Vertex::get_binding_description(0),
      SquareInstance::get_binding_description(1),
    ];
    let attribute_descriptions: Vec<vk::VertexInputAttributeDescription> = [
      Vertex::get_attribute_descriptions(0, 0),
      SquareInstance::get_attribute_descriptions(2, 1),
    ]
    .into_iter()
    .flatten()
    .collect();
    let vertex_input_state_create_info = vk::PipelineVertexInputStateCreateInfo {
      s_type: vk::StructureType::PIPELINE_VERTEX_INPUT_STATE_CREATE_INFO,
      p_next: ptr::null(),
      flags: vk::PipelineVertexInputStateCreateFlags::empty(),
      vertex_attribute_description_count: attribute_descriptions.len() as u32,
      p_vertex_attribute_descriptions: attribute_descriptions.as_ptr(),
      vertex_binding_description_count: binding_descriptions.len() as u32,
      p_vertex_binding_descriptions: binding_descriptions.as_ptr(),
    };

    let vertex_input_assembly_state_info = vk::PipelineInputAssemblyStateCreateInfo {
      s_type: vk::StructureType::PIPELINE_INPUT_ASSEMBLY_STATE_CREATE_INFO,
      flags: vk::PipelineInputAssemblyStateCreateFlags::empty(),
      p_next: ptr::null(),
      primitive_restart_enable: vk::FALSE,
      topology: vk::PrimitiveTopology::TRIANGLE_LIST,
    };

    let viewports = [vk::Viewport {
      x: 0.0,
      y: 0.0,
      width: swapchain_extent.width as f32,
      height: swapchain_extent.height as f32,
      min_depth: 0.0,
      max_depth: 1.0,
    }];

    let scissors = [vk::Rect2D {
      offset: vk::Offset2D { x: 0, y: 0 },
      extent: swapchain_extent,
    }];

    let viewport_state_create_info = vk::PipelineViewportStateCreateInfo {
      s_type: vk::StructureType::PIPELINE_VIEWPORT_STATE_CREATE_INFO,
      p_next: ptr::null(),
      flags: vk::PipelineViewportStateCreateFlags::empty(),
      scissor_count: scissors.len() as u32,
      p_scissors: scissors.as_ptr(),
      viewport_count: viewports.len() as u32,
      p_viewports: viewports.as_ptr(),
    };

    let rasterization_statue_create_info = vk::PipelineRasterizationStateCreateInfo {
      s_type: vk::StructureType::PIPELINE_RASTERIZATION_STATE_CREATE_INFO,
      p_next: ptr::null(),
      flags: vk::PipelineRasterizationStateCreateFlags::empty(),
      depth_clamp_enable: vk::FALSE,
      cull_mode: vk::CullModeFlags::BACK,
      front_face: vk::FrontFace::CLOCKWISE,
      line_width: 1.0,
      polygon_mode: vk::PolygonMode::FILL,
      rasterizer_discard_enable: vk::FALSE,
      depth_bias_clamp: 0.0,
      depth_bias_constant_factor: 0.0,
      depth_bias_enable: vk::FALSE,
      depth_bias_slope_factor: 0.0,
    };
    let multisample_state_create_info = vk::PipelineMultisampleStateCreateInfo {
      s_type: vk::StructureType::PIPELINE_MULTISAMPLE_STATE_CREATE_INFO,
      flags: vk::PipelineMultisampleStateCreateFlags::empty(),
      p_next: ptr::null(),
      rasterization_samples: vk::SampleCountFlags::TYPE_1,
      sample_shading_enable: vk::FALSE,
      min_sample_shading: 0.0,
      p_sample_mask: ptr::null(),
      alpha_to_one_enable: vk::FALSE,
      alpha_to_coverage_enable: vk::FALSE,
    };

    let stencil_state = vk::StencilOpState {
      fail_op: vk::StencilOp::KEEP,
      pass_op: vk::StencilOp::KEEP,
      depth_fail_op: vk::StencilOp::KEEP,
      compare_op: vk::CompareOp::ALWAYS,
      compare_mask: 0,
      write_mask: 0,
      reference: 0,
    };

    let depth_state_create_info = vk::PipelineDepthStencilStateCreateInfo {
      s_type: vk::StructureType::PIPELINE_DEPTH_STENCIL_STATE_CREATE_INFO,
      p_next: ptr::null(),
      flags: vk::PipelineDepthStencilStateCreateFlags::empty(),
      depth_test_enable: vk::FALSE,
      depth_write_enable: vk::FALSE,
      depth_compare_op: vk::CompareOp::LESS_OR_EQUAL,
      depth_bounds_test_enable: vk::FALSE,
      stencil_test_enable: vk::FALSE,
      front: stencil_state,
      back: stencil_state,
      max_depth_bounds: 1.0,
      min_depth_bounds: 0.0,
    };

    let color_blend_attachment_states = [vk::PipelineColorBlendAttachmentState {
      blend_enable: vk::FALSE,
      color_write_mask: vk::ColorComponentFlags::RGBA,
      src_color_blend_factor: vk::BlendFactor::ONE,
      dst_color_blend_factor: vk::BlendFactor::ZERO,
      color_blend_op: vk::BlendOp::ADD,
      src_alpha_blend_factor: vk::BlendFactor::ONE,
      dst_alpha_blend_factor: vk::BlendFactor::ZERO,
      alpha_blend_op: vk::BlendOp::ADD,
    }];

    let color_blend_state = vk::PipelineColorBlendStateCreateInfo {
      s_type: vk::StructureType::PIPELINE_COLOR_BLEND_STATE_CREATE_INFO,
      p_next: ptr::null(),
      flags: vk::PipelineColorBlendStateCreateFlags::empty(),
      logic_op_enable: vk::FALSE,
      logic_op: vk::LogicOp::COPY,
      attachment_count: color_blend_attachment_states.len() as u32,
      p_attachments: color_blend_attachment_states.as_ptr(),
      blend_constants: [0.0, 0.0, 0.0, 0.0],
    };

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
      p_vertex_input_state: &vertex_input_state_create_info,
      p_input_assembly_state: &vertex_input_assembly_state_info,
      p_tessellation_state: ptr::null(),
      p_viewport_state: &viewport_state_create_info,
      p_rasterization_state: &rasterization_statue_create_info,
      p_multisample_state: &multisample_state_create_info,
      p_depth_stencil_state: &depth_state_create_info,
      p_color_blend_state: &color_blend_state,
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
