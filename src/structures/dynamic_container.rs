use std::ops::Deref;

use super::partition::Partition;

pub struct DynamicContainer<D> {
  data: D,
  parts: Vec<Partition<u32>>,
}

impl<D> DynamicContainer<D> {
  pub fn new(data: D, parts: Vec<Partition<u32>>) -> Self {
    Self { data, parts }
  }
}

impl<D> Deref for DynamicContainer<D> {
  type Target = D;

  fn deref(&self) -> &Self::Target {
    &self.data
  }
}
