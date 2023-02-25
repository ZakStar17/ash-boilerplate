pub struct Cursor {
  pub delta_x: f64,
  pub delta_y: f64,
  pub in_window: bool,
  pub getting_grabbed: bool,
}

impl Cursor {
  pub fn new() -> Self {
    Self {
      delta_x: 0.0,
      delta_y: 0.0,
      in_window: false,
      getting_grabbed: false,
    }
  }
}
