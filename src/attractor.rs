use crate::vector3::Vector3;
use crate::constants::*;
use macroquad::prelude::rand;

pub struct Attractor {
  pub points: Vec<Vector3>,
  pub sigma: f32,
  pub rho: f32,
  pub beta: f32,
}

impl Attractor {
  pub fn new(num_points: usize, sigma: f32, rho: f32, beta: f32) -> Self {
    let mut points = Vec::new();
    for _ in 0..num_points {
      let point = Vector3 {
        x: rand::gen_range(0.0, 10.0),
        y: rand::gen_range(0.0, 10.0),
        z: rand::gen_range(10.0, 20.0),
      };
      points.push(point);
    }
    Attractor { points, sigma, rho, beta }
  }

  pub fn update(&mut self, dt: f32) {
    // TODO: use equation to update points
    for point in &mut self.points {
      let dx = self.sigma * (point.y - point.x);
      let dy = point.x * (self.rho - point.z) - point.y;
      let dz = point.x * point.y - self.beta * point.z;
      point.x += dx * dt;
      point.y += dy * dt;
      point.z += dz * dt;
    }
  }
}