use crate::vector3::Vector3;
use crate::trajectory::Trajectory;
use crate::constants::PHYSICS_SCALE_FACTOR;
use macroquad::prelude::rand;

pub struct Attractor {
  pub trajectories: Vec<Trajectory>,
  pub sigma: f32,
  pub rho: f32,
  pub beta: f32,
}

impl Attractor {
  pub fn new(num_points: usize, sigma: f32, rho: f32, beta: f32) -> Self {
    let mut trajectories = Vec::new();
      for _ in 0..num_points {
        let initial_point = Vector3 {
          x: rand::gen_range(0.0, 10.0),
          y: rand::gen_range(0.0, 10.0),
          z: rand::gen_range(10.0, 20.0),
        };
        trajectories.push(Trajectory::new(initial_point, 50, true));
      }
      Attractor { trajectories, sigma, rho, beta }
  }

  pub fn update(&mut self, dt: f32) {
    for trajectory in &mut self.trajectories {
      if let Some(last_point) = trajectory.points.last() {
        let new_point = {
          let dx = self.sigma * (last_point.y - last_point.x);
          let dy = last_point.x * (self.rho - last_point.z) - last_point.y;
          let dz = last_point.x * last_point.y - self.beta * last_point.z;

          Vector3 {
            x: last_point.x + dx * dt * PHYSICS_SCALE_FACTOR,
            y: last_point.y + dy * dt * PHYSICS_SCALE_FACTOR,
            z: last_point.z + dz * dt * PHYSICS_SCALE_FACTOR,
          }
        };
        trajectory.add_point(new_point);
      }
    }
  }
}