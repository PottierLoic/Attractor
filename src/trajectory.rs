use crate::vector3::Vector3;

pub struct Trajectory {
  points: Vec<Vector3>,
  max_points: usize,
  show_path: bool,
}

impl Trajectory {
  pub fn new(initial_point: Vector3, max_points: usize, show_path: bool) -> Self {
    Self {
      points: vec![initial_point],
      max_points,
      show_path,
    }
  }

  pub fn add_point(&mut self, point: Vector3) {
    if self.points.len() >= self.max_points {
      self.points.remove(0);
    }
    self.points.push(point);
  }
}