use std::ptr;

use ash::vk;

pub fn create_framebuffers(
  device: &ash::Device,
  render_pass: vk::RenderPass,
  image_views: &Vec<vk::ImageView>,
  extent: &vk::Extent2D,
) -> Vec<vk::Framebuffer> {
  let mut framebuffers = Vec::with_capacity(image_views.len());

  for &image_view in image_views.iter() {
    let attachments = [image_view];

    let framebuffer_create_info = vk::FramebufferCreateInfo {
      s_type: vk::StructureType::FRAMEBUFFER_CREATE_INFO,
      p_next: ptr::null(),
      flags: vk::FramebufferCreateFlags::empty(),
      render_pass,
      attachment_count: attachments.len() as u32,
      p_attachments: attachments.as_ptr(),
      width: extent.width,
      height: extent.height,
      layers: 1,
    };

    let framebuffer = unsafe {
      device
        .create_framebuffer(&framebuffer_create_info, None)
        .expect("Failed to create framebuffer")
    };

    framebuffers.push(framebuffer);
  }

  framebuffers
}
