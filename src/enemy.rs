use macroquad::{
  prelude::{vec2, Vec2, WHITE},
  texture::{draw_texture_ex, DrawTextureParams, Texture2D},
  time::get_frame_time,
};

use crate::{
  emath::pos_to_grid_pos,
  rect::{Collidable, Rect},
  wrld::World,
};

pub struct Enemy {
  //Fractional position.
  _pos: Vec2,
  //Pixel position.
  pub pos: (usize, usize),
  pub grid_pos: (usize, usize),
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
      grid_pos: pos_to_grid_pos(&pos),
      texture,
      size,
      rotation: 0.0,
      rect: Rect::new(pos.0, pos.1, pos.0 + size.0, pos.1 + size.1),
    }
  }
  pub fn update_rect(&mut self) {
    self.rect = Rect::new(
      self.pos.0,
      self.pos.1,
      self.pos.0 + self.size.0,
      self.pos.1 + self.size.1,
    );
  }
  pub fn update(&mut self, wrld: &World) {
    self._pos.x += 0.0 * 4.0 * get_frame_time();

    self._pos.y += wrld.get_scaled_gravity();
    let ydiff = self._pos.y - self.pos.1 as f32;
    if ydiff > 1.0 {
      //Update position and check for collisions.
      let mut y: usize = 0;
      while y <= ydiff as usize {
        let rect = Rect::new(
          self.pos.0,
          self.pos.1 + 1,
          self.pos.0 + self.size.0,
          self.pos.1 + 1 + self.size.1,
        );

        if let Some(tile_below) = wrld.get_tile(&self.grid_pos.0, &(self.grid_pos.1 + 1)) {
          // if wrld.frame % 60 == 0 {
          //   println!("{:?}, {:?}", tile_below.kind(), tile_below.grid_pos());
          // }
          if tile_below.collide(&rect) {
            //We have collided with a solid tile.
            self._pos.y = self.pos.1 as f32;
            break;
          }
        }
        self.pos.1 += 1;
        self.grid_pos = pos_to_grid_pos(&self.pos);
        y += 1;
      }
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
    self.rect.intersecting(other.get_rect())
  }
}
