use macroquad::{
  prelude::{vec2, Vec2, WHITE},
  texture::{draw_texture_ex, DrawTextureParams, Texture2D},
};

use crate::{
  rect::{Collidable, Rect},
  wrld::World,
};

pub struct Enemy {
  //Fractional position.
  _pos: Vec2,
  //Pixel position.
  pub pos: (usize, usize),
  texture: Texture2D,
  size: (usize, usize),
  rotation: f32,
  rect: Rect,
}

impl Enemy {
  pub fn new(pos: (usize, usize), size: (usize, usize), texture: Texture2D) -> Enemy {
    Enemy {
      _pos: vec2(pos.0 as f32, pos.1 as f32),
      pos,
      texture,
      size,
      rotation: 0.0,
      rect: Rect::new(pos.0, pos.1, pos.0 + size.0, pos.1 + size.1),
    }
  }
  pub fn draw(&self, wrld: &World) {
    draw_texture_ex(
      self.texture,
      (self.pos.0 as f32 + wrld.scroll_pos.x) * wrld.zoom,
      (self.pos.1 as f32 + wrld.scroll_pos.y) * wrld.zoom,
      WHITE,
      DrawTextureParams {
        dest_size: Some(vec2(
          wrld.grid_size * self.size.0 as f32,
          wrld.grid_size * self.size.1 as f32,
        )),
        rotation: self.rotation,
        ..Default::default()
      },
    )
  }
}

impl Collidable for Enemy {
  fn get_rect(&self) -> &Rect {
    &self.rect
  }
  fn collide(&self, other: &impl Collidable) -> bool {
    self.rect.collide(other.get_rect())
  }
}

// pub fn update_enemies(wrld: &World, )
