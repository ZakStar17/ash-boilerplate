use std::ptr;

use ash::vk;

pub fn create_render_pass(device: &ash::Device, surface_format: vk::Format) -> vk::RenderPass {
  let color_attachment = vk::AttachmentDescription {
    flags: vk::AttachmentDescriptionFlags::empty(),
    format: surface_format,
    samples: vk::SampleCountFlags::TYPE_1,
    load_op: vk::AttachmentLoadOp::CLEAR,
    store_op: vk::AttachmentStoreOp::STORE,
    stencil_load_op: vk::AttachmentLoadOp::DONT_CARE,
    stencil_store_op: vk::AttachmentStoreOp::DONT_CARE,
    initial_layout: vk::ImageLayout::UNDEFINED,
    final_layout: vk::ImageLayout::PRESENT_SRC_KHR,
  };

  let color_attachment_ref = vk::AttachmentReference {
    attachment: 0,
    layout: vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL,
  };

  let subpass = vk::SubpassDescription {
    flags: vk::SubpassDescriptionFlags::empty(),
    pipeline_bind_point: vk::PipelineBindPoint::GRAPHICS,
    input_attachment_count: 0,
    p_input_attachments: ptr::null(),
    color_attachment_count: 1,
    p_color_attachments: &color_attachment_ref,
    p_resolve_attachments: ptr::null(),
    p_depth_stencil_attachment: ptr::null(),
    preserve_attachment_count: 0,
    p_preserve_attachments: ptr::null(),
  };

  let render_pass_attachments = [color_attachment];

  let subpass_dependencies = [vk::SubpassDependency {
    src_subpass: vk::SUBPASS_EXTERNAL,
    dst_subpass: 0,
    src_stage_mask: vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT,
    dst_stage_mask: vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT,
    src_access_mask: vk::AccessFlags::empty(),
    dst_access_mask: vk::AccessFlags::COLOR_ATTACHMENT_WRITE,
    dependency_flags: vk::DependencyFlags::empty(),
  }];

  let renderpass_create_info = vk::RenderPassCreateInfo {
    s_type: vk::StructureType::RENDER_PASS_CREATE_INFO,
    flags: vk::RenderPassCreateFlags::empty(),
    p_next: ptr::null(),
    attachment_count: render_pass_attachments.len() as u32,
    p_attachments: render_pass_attachments.as_ptr(),
    subpass_count: 1,
    p_subpasses: &subpass,
    dependency_count: subpass_dependencies.len() as u32,
    p_dependencies: subpass_dependencies.as_ptr(),
  };

  unsafe {
    device
      .create_render_pass(&renderpass_create_info, None)
      .expect("Failed to create render pass!")
  }
}
