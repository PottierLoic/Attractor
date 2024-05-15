use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign};

/// Represents a vector in 3D space.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vector3 {
  pub x: f32,
  pub y: f32,
  pub z: f32,
}

impl Add for Vector3 {
  type Output = Self;
  fn add(self, other: Self) -> Self {
    Self {
      x: self.x + other.x,
      y: self.y + other.y,
      z: self.z + other.z,
    }
  }
}

impl AddAssign for Vector3 {
  fn add_assign(&mut self, other: Self) {
    self.x += other.x;
    self.y += other.y;
    self.z += other.z;
  }
}

impl Sub for Vector3 {
  type Output = Self;
  fn sub(self, other: Self) -> Self {
    Self {
      x: self.x - other.x,
      y: self.y - other.y,
      z: self.z - other.z,
    }
  }
}

impl SubAssign for Vector3 {
  fn sub_assign(&mut self, other: Self) {
    self.x -= other.x;
    self.y -= other.y;
    self.z -= other.z;
  }
}

impl Mul<f32> for Vector3 {
  type Output = Self;
  fn mul(self, scalar: f32) -> Self {
    Self {
      x: self.x * scalar,
      y: self.y * scalar,
      z: self.z * scalar,
    }
  }
}

impl MulAssign<f32> for Vector3 {
  fn mul_assign(&mut self, scalar: f32) {
    self.x *= scalar;
    self.y *= scalar;
    self.z *= scalar;
  }
}

impl Div<f32> for Vector3 {
  type Output = Self;
  fn div(self, scalar: f32) -> Self {
    if scalar == 0.0 { panic!("Attempted to divide by zero"); }
    Self {
      x: self.x / scalar,
      y: self.y / scalar,
      z: self.z / scalar,
    }
  }
}

impl DivAssign<f32> for Vector3 {
  fn div_assign(&mut self, scalar: f32) {
    if scalar == 0.0 { panic!("Attempted to divide by zero"); }
    self.x /= scalar;
    self.y /= scalar;
    self.z /= scalar;
  }
}

impl Vector3 {
  pub fn magnitude(&self) -> f32 {
    (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
  }

  pub fn normalize(&self) -> Vector3 {
    let mag = self.magnitude();
    Vector3 {
      x: self.x / mag,
      y: self.y / mag,
      z: self.z / mag,
    }
}

  pub fn distance_to(&self, other: &Self) -> f32 {
    (*self - *other).magnitude()
  }

  pub fn dot(&self, other: &Self) -> f32 {
    self.x * other.x + self.y * other.y + self.z * other.z
  }

  pub fn cross(&self, other: &Self) -> Vector3 {
    Vector3 {
      x: self.y * other.z - self.z * other.y,
      y: self.z * other.x - self.x * other.z,
      z: self.x * other.y - self.y * other.x,
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_addition() {
    let v1 = Vector3 { x: 1.0, y: 2.0, z: 3.0 };
    let v2 = Vector3 { x: 4.0, y: 5.0, z: 6.0 };
    assert_eq!(v1 + v2, Vector3 { x: 5.0, y: 7.0, z: 9.0 });
  }

  #[test]
  fn test_subtraction() {
    let v1 = Vector3 { x: 5.0, y: 7.0, z: 9.0 };
    let v2 = Vector3 { x: 1.0, y: 2.0, z: 3.0 };
    assert_eq!(v1 - v2, Vector3 { x: 4.0, y: 5.0, z: 6.0 });
  }

  #[test]
  fn test_scalar_multiplication() {
    let v = Vector3 { x: 1.0, y: 2.0, z: 3.0 };
    assert_eq!(v * 3.0, Vector3 { x: 3.0, y: 6.0, z: 9.0 });
  }

  #[test]
  fn test_scalar_division() {
    let v = Vector3 { x: 9.0, y: 6.0, z: 3.0 };
    assert_eq!(v / 3.0, Vector3 { x: 3.0, y: 2.0, z: 1.0 });
  }

  #[test]
  fn test_magnitude() {
    let v = Vector3 { x: 0.0, y: 3.0, z: 4.0 };
    assert!((v.magnitude() - 5.0).abs() < 1e-5);
  }

  #[test]
  fn test_normalization() {
    let v = Vector3 { x: 0.0, y: 3.0, z: 4.0 };
    let norm = v.normalize();
    let expected = Vector3 { x: 0.0, y: 0.6, z: 0.8 };
    assert!((norm.x - expected.x).abs() < 1e-5 &&
            (norm.y - expected.y).abs() < 1e-5 &&
            (norm.z - expected.z).abs() < 1e-5);
  }

  #[test]
  fn test_distance_to() {
    let v1 = Vector3 { x: 1.0, y: 2.0, z: 3.0 };
    let v2 = Vector3 { x: 1.0, y: 3.0, z: 4.0 };
    assert!((v1.distance_to(&v2) - 1.41421356).abs() < 1e-5);
  }

  #[test]
  fn test_dot_product() {
    let v1 = Vector3 { x: 1.0, y: 3.0, z: -5.0 };
    let v2 = Vector3 { x: 4.0, y: -2.0, z: -1.0 };
    assert_eq!(v1.dot(&v2), 3.0);
  }

  #[test]
  fn test_cross_product() {
    let v1 = Vector3 { x: 2.0, y: 3.0, z: 4.0 };
    let v2 = Vector3 { x: 5.0, y: 6.0, z: 7.0 };
    let result = v1.cross(&v2);
    let expected = Vector3 { x: -3.0, y: 6.0, z: -3.0 };
    assert_eq!(result, expected, "Cross product is incorrect");
    assert_eq!(result.dot(&v1), 0.0, "Result vector is not perpendicular to the first vector");
    assert_eq!(result.dot(&v2), 0.0, "Result vector is not perpendicular to the second vector");
  }
}