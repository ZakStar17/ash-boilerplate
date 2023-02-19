mod compute;
mod graphics;

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
