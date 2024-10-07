mod vector;

use speedy2d::color::Color;
use speedy2d::window::{self, WindowHandler, WindowHelper};
use speedy2d::{Graphics2D, Window};
use vector::Vector;

fn main() {
  let window = Window::new_centered("HYPNOTIC PENDULUM", (800, 600)).unwrap();
  window.run_loop(MyWindowHandler {
    pendulums: vec![
      Pendulum::new(400.0, 0.0, 200.0, 1.0),
      Pendulum::new(400.0, 0.0, 200.0, 0.5),
    ],
  });
}

struct MyWindowHandler {
  pendulums: Vec<Pendulum>,
}

impl WindowHandler for MyWindowHandler {
  fn on_draw(&mut self, helper: &mut WindowHelper<()>, graphics: &mut Graphics2D) {
    graphics.clear_screen(Color::WHITE);
    for pendulum in &mut self.pendulums {
      pendulum.update();
      pendulum.draw(graphics);
    }
    helper.request_redraw();
  }
}

struct Pendulum {
  origin: Vector,
  position: Vector,
  angle: f32,
  angular_velocity: f32,
  angular_acceleration: f32,
  r: f32,
  m: f32,
  g: f32,
}

impl Pendulum {
  fn new(x: f32, y: f32, r: f32, angle_offset: f32) -> Self {
    Self {
      origin: Vector::new(x, y),
      position: Vector::new(x, y),
      angle: 1.0 + angle_offset, // Adiciona um deslocamento ao ângulo inicial
      angular_velocity: 0.0,
      angular_acceleration: 0.0,
      r,
      m: 1.0,
      g: 0.5, // Diminuindo a força gravitacional para desacelerar o movimento
    }
  }

  fn update(&mut self) {
    self.angular_acceleration = -1.0 * self.g * self.angle.sin() / self.r;
    self.angular_velocity += self.angular_acceleration;
    self.angle += self.angular_velocity;
    self
      .position
      .set(self.r * self.angle.sin(), self.r * self.angle.cos());
    self.position.add(&self.origin);
  }

  fn draw(&self, graphics: &mut Graphics2D) {
    graphics.draw_line(
      (self.origin.x, self.origin.y),
      (self.position.x, self.position.y),
      3.0,
      Color::BLACK,
    );
    graphics.draw_circle((self.position.x, self.position.y), 30.0, Color::BLACK);
  }
}
