pub struct Vector {
  pub x: f32,
  pub y: f32,
}

impl Vector {
  pub fn new(x: f32, y: f32) -> Self {
    Vector { x, y }
  }
  pub fn add(&mut self, update: &Vector) -> &Self {
    self.x += update.x;
    self.y += update.y;
    self
  }
  pub fn set(&mut self, x: f32, y: f32) -> &Self {
    self.x = x;
    self.y = y;
    self
  }
}
