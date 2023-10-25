mod compute;
mod graphics;

use std::{
  pin::Pin,
  ptr::{self, addr_of},
};

use ash::vk;

use self::{compute::ComputePipelines, graphics::GraphicsPipelines};

use super::DescriptorSets;

pub struct Pipelines {
  pub graphics: GraphicsPipelines,
  pub compute: ComputePipelines,
}

impl Pipelines {
  pub fn new(
    device: &ash::Device,
    swapchain_extent: vk::Extent2D,
    render_pass: vk::RenderPass,
    descriptor_sets: &DescriptorSets,
  ) -> Self {
    let graphics = GraphicsPipelines::create(device, swapchain_extent, render_pass);
    let compute = ComputePipelines::create(device, descriptor_sets);

    Self { graphics, compute }
  }

  pub fn recreate_main(
    &mut self,
    device: &ash::Device,
    swapchain_extent: vk::Extent2D,
    render_pass: vk::RenderPass,
  ) {
    // TODO: reimplement with pipeline cache
    unsafe {
      self.graphics.destroy_self(device);
    }
    self.graphics = GraphicsPipelines::create(device, swapchain_extent, render_pass);
  }

  pub unsafe fn destroy_self(&mut self, device: &ash::Device) {
    self.graphics.destroy_self(device);
    self.compute.destroy_self(device);
  }
}

fn get_default_input_assembly_state_ci() -> vk::PipelineInputAssemblyStateCreateInfo {
  vk::PipelineInputAssemblyStateCreateInfo {
    s_type: vk::StructureType::PIPELINE_INPUT_ASSEMBLY_STATE_CREATE_INFO,
    flags: vk::PipelineInputAssemblyStateCreateFlags::empty(),
    p_next: ptr::null(),
    primitive_restart_enable: vk::FALSE,
    topology: vk::PrimitiveTopology::TRIANGLE_LIST,
  }
}

struct PipelineViewportStateCIOwned {
  pub ci: vk::PipelineViewportStateCreateInfo,
  viewport: Pin<Box<vk::Viewport>>,
  scissor: Pin<Box<vk::Rect2D>>,
}

impl PipelineViewportStateCIOwned {
  fn new(swapchain_extent: vk::Extent2D) -> Self {
    // Make sure viewport and scissor location not change
    let viewport = Box::pin(vk::Viewport {
      x: 0.0,
      y: 0.0,
      width: swapchain_extent.width as f32,
      height: swapchain_extent.height as f32,
      min_depth: 0.0,
      max_depth: 1.0,
    });

    let scissor = Box::pin(vk::Rect2D {
      offset: vk::Offset2D { x: 0, y: 0 },
      extent: swapchain_extent,
    });

    let ci = vk::PipelineViewportStateCreateInfo {
      s_type: vk::StructureType::PIPELINE_VIEWPORT_STATE_CREATE_INFO,
      p_next: ptr::null(),
      flags: vk::PipelineViewportStateCreateFlags::empty(),
      scissor_count: 1,
      p_scissors: addr_of!(*scissor),
      viewport_count: 1,
      p_viewports: addr_of!(*viewport),
    };

    Self {
      ci,
      viewport,
      scissor,
    }
  }
}

fn get_no_depth_rasterization_state_ci() -> vk::PipelineRasterizationStateCreateInfo {
  vk::PipelineRasterizationStateCreateInfo {
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
  }
}

fn get_no_multisample_state_ci() -> vk::PipelineMultisampleStateCreateInfo {
  vk::PipelineMultisampleStateCreateInfo {
    s_type: vk::StructureType::PIPELINE_MULTISAMPLE_STATE_CREATE_INFO,
    flags: vk::PipelineMultisampleStateCreateFlags::empty(),
    p_next: ptr::null(),
    rasterization_samples: vk::SampleCountFlags::TYPE_1,
    sample_shading_enable: vk::FALSE,
    min_sample_shading: 0.0,
    p_sample_mask: ptr::null(),
    alpha_to_one_enable: vk::FALSE,
    alpha_to_coverage_enable: vk::FALSE,
  }
}

struct PipelineColorBlendStateCIOwned {
  pub ci: vk::PipelineColorBlendStateCreateInfo,
  attachment_state: Pin<Box<vk::PipelineColorBlendAttachmentState>>,
}

impl PipelineColorBlendStateCIOwned {
  fn no_blend_color() -> Self {
    let attachment_state = Box::pin(vk::PipelineColorBlendAttachmentState {
      blend_enable: vk::FALSE,
      color_write_mask: vk::ColorComponentFlags::RGBA,
      src_color_blend_factor: vk::BlendFactor::ONE,
      dst_color_blend_factor: vk::BlendFactor::ZERO,
      color_blend_op: vk::BlendOp::ADD,
      src_alpha_blend_factor: vk::BlendFactor::ONE,
      dst_alpha_blend_factor: vk::BlendFactor::ZERO,
      alpha_blend_op: vk::BlendOp::ADD,
    });

    let ci = vk::PipelineColorBlendStateCreateInfo {
      s_type: vk::StructureType::PIPELINE_COLOR_BLEND_STATE_CREATE_INFO,
      p_next: ptr::null(),
      flags: vk::PipelineColorBlendStateCreateFlags::empty(),
      logic_op_enable: vk::FALSE,
      logic_op: vk::LogicOp::COPY,
      attachment_count: 1,
      p_attachments: addr_of!(*attachment_state),
      blend_constants: [0.0, 0.0, 0.0, 0.0],
    };

    Self {
      ci,
      attachment_state,
    }
  }
}
