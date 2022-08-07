use macroquad::prelude::Vec2;
use std::ops::{Add, Div, Mul, Sub};

pub fn map_range<T: Copy>(from_range: (T, T), to_range: (T, T), s: T) -> T
where
  T: Add<T, Output = T> + Sub<T, Output = T> + Mul<T, Output = T> + Div<T, Output = T>,
{
  to_range.0 + (s - from_range.0) * (to_range.1 - to_range.0) / (from_range.1 - from_range.0)
}

pub fn i_to_xy(width: &usize, index: &usize) -> (usize, usize) {
  (index % width, index / width)
}

// pub fn map_range_slope<T: Copy>(from_start: T, to_start: T, slope: T, v: T) -> T
// where
//   T: Add<T, Output = T> + Sub<T, Output = T> + Mul<T, Output = T> + Div<T, Output = T>,
// {
//   to_start + (v - from_start) * slope
// }

pub fn vec_sub(pos: &Vec2, minus: &Vec2) -> Vec2 {
  Vec2::new(pos.x - minus.x, pos.y - minus.y)
}
pub fn vec_add(a: &Vec2, b: &Vec2) -> Vec2 {
  Vec2::new(a.x + b.x, a.y + b.y)
}

#[cfg(test)]
mod tests {
  use super::*;
  use lazy_static::lazy_static;

  lazy_static! {
    static ref ZX_ZY: Vec2 = Vec2::new(0.0, 0.0);
    static ref ZX_PY: Vec2 = Vec2::new(0.0, 20.0);
    static ref PX_ZY: Vec2 = Vec2::new(20.0, 0.0);
    static ref NX_NY: Vec2 = Vec2::new(-10.0, -10.0);
    static ref PX_PY: Vec2 = Vec2::new(20.0, 20.0);
    static ref PX_NY: Vec2 = Vec2::new(20.0, -10.0);
    static ref NX_PY: Vec2 = Vec2::new(-10.0, 20.0);
  }

  #[test]
  fn vec_subtraction_negative_values() {
    let result = vec_sub(&NX_NY, &NX_NY);
    assert_eq!(result, Vec2::new(0.0, 0.0));
  }
  #[test]
  fn vec_subtraction_positive_values() {
    let result = vec_sub(&NX_NY, &PX_PY);
    assert_eq!(result, Vec2::new(-30.0, -30.0));
  }
  #[test]
  fn vec_subtraction_mixed_sign() {
    let result = vec_sub(&PX_NY, &NX_PY);
    assert_eq!(result, Vec2::new(30.0, -30.0));
  }
  #[test]
  fn vec_addition_negative_sign() {
    let result = vec_add(&NX_NY, &NX_NY);
    assert_eq!(result, Vec2::new(-20.0, -20.0));
  }
  #[test]
  fn vec_addition_positive_sign() {
    let result = vec_add(&PX_PY, &PX_NY);
    assert_eq!(result, Vec2::new(40.0, 10.0));
  }
  #[test]
  fn vec_addition_mixed_sign() {
    let result = vec_add(&NX_PY, &PX_NY);
    assert_eq!(result, Vec2::new(10.0, 10.0));
  }
}
