use macroquad::{prelude::Color, shapes::draw_rectangle_lines};

#[derive(Debug, Clone, Copy)]
pub struct Rect {
  pub left: usize,
  pub top: usize,
  pub right: usize,
  pub bottom: usize,
}

impl Rect {
  pub fn tl(&self) -> (usize, usize) {
    (self.left, self.top)
  }
  pub fn br(&self) -> (usize, usize) {
    (self.right, self.bottom)
  }
  // pub fn as_vec2(&self) -> (Vec2, Vec2) {
  //   (
  //     vec2(self.left as f32, self.top as f32),
  //     vec2(self.right as f32, self.bottom as f32),
  //   )
  // }
  pub fn rect(&self) -> ((usize, usize), (usize, usize)) {
    (self.tl(), self.br())
  }
  pub fn new(left: usize, top: usize, right: usize, bottom: usize) -> Rect {
    Rect {
      left,
      top,
      right,
      bottom,
    }
  }

  pub fn width(&self) -> usize {
    self.right - self.left
  }
  pub fn height(&self) -> usize {
    self.bottom - self.top
  }

  pub fn intersecting(&self, other: &Rect) -> bool {
    let a = self;
    let b = other;

    a.left <= b.right && a.right >= b.left && a.top <= b.bottom && a.bottom >= b.top
  }
  pub fn debug_draw(&self, color: Color) {
    draw_rectangle_lines(
      self.left as f32,
      self.top as f32,
      self.width() as f32,
      self.height() as f32,
      1.0,
      color,
    );
  }
}

impl Collidable for Rect {
  fn get_hitbox(&self) -> &Rect {
    &self
  }
}

pub trait Collidable {
  fn get_hitbox(&self) -> &Rect;
  fn collide(&self, other: &impl Collidable) -> bool {
    self.get_hitbox().intersecting(other.get_hitbox())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn collide_same() {
    let a = Rect::new(0, 0, 10, 10);
    let b = a;
    let result = a.intersecting(&b);
    assert_eq!(result, true);
  }

  #[test]
  fn collide_overlap() {
    let a = Rect::new(0, 0, 10, 10);
    let b = Rect::new(5, 5, 15, 15);
    let result = a.intersecting(&b);
    assert_eq!(result, true);
  }

  #[test]
  fn collide_touching_side() {
    let a = Rect::new(0, 0, 10, 10);
    let b = Rect::new(10, 0, 20, 20);
    let result = a.intersecting(&b);
    assert_eq!(result, true);
  }
  #[test]
  fn collide_touching_top() {
    let a = Rect::new(0, 0, 10, 10);
    let b = Rect::new(0, 10, 20, 20);
    let result = a.intersecting(&b);
    assert_eq!(result, true);
  }

  #[test]
  fn collide_not_overlap() {
    let a = Rect::new(0, 0, 10, 10);
    let b = Rect::new(15, 15, 25, 25);
    let result = a.intersecting(&b);
    assert_eq!(result, false);
  }

  #[test]
  fn collide_not_overlap_reverse() {
    let a = Rect::new(480, 224, 512, 288);
    let b = Rect::new(33, 32, 33, 34);
    let result = a.intersecting(&b);
    assert_eq!(result, false);
  }
}
