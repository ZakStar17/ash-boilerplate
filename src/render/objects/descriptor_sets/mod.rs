use log::debug;

use self::{layouts::DescriptorSetLayouts, pool::DescriptorSetPool};

mod layouts;
mod pool;

pub struct DescriptorSets {
  pub layouts: DescriptorSetLayouts,
  pub pool: DescriptorSetPool,
}

impl DescriptorSets {
  pub fn new(device: &ash::Device) -> Self {
    debug!("Creating descriptor set layouts");
    let layouts = DescriptorSetLayouts::new(device);
    debug!("Creating descriptor set pool");
    let pool = DescriptorSetPool::new(device, &layouts);
    Self { layouts, pool }
  }

  pub unsafe fn destroy_self(&mut self, device: &ash::Device) {
    self.pool.destroy_self(device);
    self.layouts.destroy_self(device);
  }
}
