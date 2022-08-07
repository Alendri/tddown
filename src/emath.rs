use crate::deb::DebugPrintSettings;
use macroquad::prelude::{draw_text, Vec2};
use std::{
  f32::consts::TAU,
  ops::{Add, Div, Mul, Sub},
};

pub fn map_range<T: Copy>(from_range: (T, T), to_range: (T, T), s: T) -> T
where
  T: Add<T, Output = T> + Sub<T, Output = T> + Mul<T, Output = T> + Div<T, Output = T>,
{
  to_range.0 + (s - from_range.0) * (to_range.1 - to_range.0) / (from_range.1 - from_range.0)
}

// pub fn map_range_slope<T: Copy>(from_start: T, to_start: T, slope: T, v: T) -> T
// where
//   T: Add<T, Output = T> + Sub<T, Output = T> + Mul<T, Output = T> + Div<T, Output = T>,
// {
//   to_start + (v - from_start) * slope
// }

/**
 * Returns angle in radians. Right: 0, Up: -1/4τ, Left: -1/2τ, Down: 1/4τ.
 */
pub fn angle(from: &Vec2, to: &Vec2, deb: Option<DebugPrintSettings>) -> f32 {
  let dx = to.x - from.x;
  let dy = to.y - from.y;

  let slope = dy / dx;

  if from.x == to.x {
    //Points are in vertical line.
    match () {
      _ if from.y > to.y => return -0.25 * TAU,
      _ if from.y < to.y => return 0.25 * TAU,
      _ => return 0.0, //If the points are exactly on top of each other.
    }
  }

  let angle = if dx > 0.0 {
    slope.atan()
  } else {
    slope.atan() - TAU * 0.5
  };

  if let Some(deb) = deb {
    draw_text(
      &format!(
        "{}   a: ({:.0},{:.0})   b: ({:.0}, {:.0})   slope {:.3}   angle {:.3}",
        deb.prefix.unwrap_or("Angle:".to_owned()),
        from.x,
        from.y,
        to.x,
        to.y,
        slope,
        angle,
      ),
      deb.x,
      deb.y,
      deb.fs,
      deb.color,
    );
  }
  angle
}

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

  //NOTE: Coordinate system is from top left to bottom right.
  #[test]
  fn angle_45() {
    //0,0 20,20 should be 45 degrees to the bottom right or 1/8 TAU
    let result = angle(&ZX_ZY, &PX_PY, None);
    assert_eq!(result, TAU / 8.0);
  }
  #[test]
  fn angle_down() {
    let result = angle(&ZX_ZY, &ZX_PY, None);
    assert_eq!(result, TAU / 4.0);
  }
  #[test]
  fn angle_right() {
    let result = angle(&ZX_ZY, &PX_ZY, None);
    assert_eq!(result, 0.0);
  }
  #[test]
  fn angle_left() {
    let result = angle(&PX_ZY, &ZX_ZY, None);
    assert_eq!(result, -TAU / 2.0);
  }
  #[test]
  fn angle_up() {
    let result = angle(&ZX_PY, &ZX_ZY, None);
    assert_eq!(result, -TAU / 4.0);
  }
  #[test]
  fn angle_same_from_to() {
    let result = angle(&ZX_PY, &ZX_PY, None);
    assert_eq!(result, 0.0);
  }
}
