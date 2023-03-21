use num::{PrimInt, Unsigned};

#[derive(Clone, Copy)]
pub struct Partition<S> {
  pub size: S,
  pub offset: S,
}

impl<S: Unsigned + PrimInt> Partition<S> {
  pub fn new(size: S, offset: S) -> Self {
    Self { size, offset }
  }

  pub fn deconstruct(self) -> (S, S) {
    (self.size, self.offset)
  }
}
