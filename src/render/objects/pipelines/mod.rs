mod compute;
mod graphics;

use ash::vk;

use self::{compute::ComputePipelines, graphics::GraphicsPipeline};

use super::DescriptorSets;

pub struct Pipelines {
  graphics: GraphicsPipeline,
  pub compute: ComputePipelines,
}

impl Pipelines {
  pub fn new(
    device: &ash::Device,
    swapchain_extent: vk::Extent2D,
    render_pass: vk::RenderPass,
    descriptor_sets: &DescriptorSets,
  ) -> Self {
    let graphics = GraphicsPipeline::create(device, swapchain_extent, render_pass);
    let compute = ComputePipelines::create(device, descriptor_sets);

    Self { graphics, compute }
  }

  pub fn get_graphics(&self) -> vk::Pipeline {
    self.graphics.pipeline
  }

  pub fn recreate_graphics(
    &mut self,
    device: &ash::Device,
    swapchain_extent: vk::Extent2D,
    render_pass: vk::RenderPass,
  ) {
    // TODO: reimplement with pipeline cache
    unsafe {
      self.graphics.destroy_self(device);
    }
    self.graphics = GraphicsPipeline::create(device, swapchain_extent, render_pass);
  }

  pub unsafe fn destroy_self(&mut self, device: &ash::Device) {
    self.graphics.destroy_self(device);
    self.compute.destroy_self(device);
  }
}
