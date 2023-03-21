use crate::structures::partition::Partition;
use std::{ops::Index, slice::Iter, vec::IntoIter};

macro_rules! sized_flatten {
  ($x:expr, $s:ty) => {{
    let mut offset: $s = 0;
    let mut part = Vec::with_capacity($x.len());
    for item in $x.iter() {
      let size = item.len() as $s;
      part.push(Partition::new(size, offset));
      offset += size;
    }
    ($x.into_iter().flatten().collect(), part)
  }};
}

pub struct SizesIter<'a, T> {
  vec: &'a Linear2dVec<T>,
  i: usize,
}

impl<'a, T> Iterator for SizesIter<'a, T> {
  type Item = usize;
  fn next(&mut self) -> Option<Self::Item> {
    if self.i < self.vec.parts.len() {
      self.i += 1;
      Some(self.vec.parts[self.i].size)
    } else {
      None
    }
  }
}

impl<'a, T> ExactSizeIterator for SizesIter<'a, T> {
  fn len(&self) -> usize {
    self.vec.parts.len() - self.i
  }
}

pub struct Linear2dVec<T> {
  data: Vec<T>,
  parts: Vec<Partition<usize>>,
}

impl<T> Linear2dVec<T> {
  pub fn from_vec(v: Vec<Vec<T>>) -> Self {
    let (data, parts) = sized_flatten!(v, usize);
    Self { data, parts }
  }

  pub fn len(&self) -> usize {
    self.data.len()
  }

  pub fn sizes<'a>(&'a self) -> SizesIter<'a, T> {
    SizesIter { vec: &self, i: 0 }
  }

  pub fn offset(&self, i: usize) -> usize {
    self.parts[i].offset
  }

  pub fn part(&self, i: usize) -> Partition<usize> {
    self.parts[i]
  }

  pub fn parts(&self) -> &Vec<Partition<usize>> {
    &self.parts
  }

  pub fn parts_iter(&self) -> Iter<Partition<usize>> {
    self.parts.iter()
  }

  pub fn into_parts_iter(self) -> IntoIter<Partition<usize>> {
    self.parts.into_iter()
  }

  pub fn as_ptr(&self) -> *const T {
    self.data.as_ptr()
  }

  pub fn iter(&self) -> Iter<'_, T> {
    self.data.iter()
  }

  pub fn deconstruct(self) -> (Vec<T>, Vec<Partition<usize>>) {
    (self.data, self.parts)
  }
}

impl<T> IntoIterator for Linear2dVec<T> {
  type Item = T;
  type IntoIter = std::vec::IntoIter<Self::Item>;

  fn into_iter(self) -> Self::IntoIter {
    self.data.into_iter()
  }
}

impl<T> Index<usize> for Linear2dVec<T> {
  type Output = [T];

  fn index(&self, index: usize) -> &Self::Output {
    let (size, offset) = self.parts[index].deconstruct();
    &self.data[offset..(offset + size)]
  }
}

impl<T> From<&mut dyn ExactSizeIterator<Item = Vec<T>>> for Linear2dVec<T> {
  fn from(iter: &mut dyn ExactSizeIterator<Item = Vec<T>>) -> Self {
    let mut offset: usize = 0;
    let mut parts = Vec::with_capacity(iter.len());
    let data: Vec<T> = iter
      .flat_map(|item| {
        let size = item.len();
        parts.push(Partition::new(size, offset));
        offset += size;
        item
      })
      .collect();
    Self { data, parts }
  }
}

impl<T> From<&mut dyn Iterator<Item = Vec<T>>> for Linear2dVec<T> {
  fn from(iter: &mut dyn Iterator<Item = Vec<T>>) -> Self {
    let mut offset: usize = 0;
    let mut parts = match iter.size_hint() {
      (_, Some(size)) => Vec::with_capacity(size),
      _ => Vec::new(),
    };
    let data: Vec<T> = iter
      .flat_map(|item| {
        let size = item.len() as usize;
        parts.push(Partition::new(size, offset));
        offset += size;
        item
      })
      .collect();
    Self { data, parts }
  }
}
