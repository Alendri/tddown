use macroquad::{
  prelude::{vec2, Vec2, WHITE},
  texture::{draw_texture_ex, DrawTextureParams, Texture2D},
};

use crate::{
  emath::pos_to_grid_pos,
  rect::{Collidable, Rect},
  tile::TileType,
  wrld::World,
};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Facing {
  Left,
  Right,
}

const WALKING_SPEED: f32 = 32.0;

pub struct Enemy {
  //Fractional position.
  _pos: Vec2,
  //Pixel position.
  pub pos: (usize, usize),
  pub grid_pos: (usize, usize),
  texture: Texture2D,
  size: (usize, usize),
  facing: Facing,
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
      facing: Facing::Left,
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

  /** Returns (falling, keep) */
  fn move_y(&mut self, wrld: &mut World) -> (bool, bool) {
    //Update position and check for collisions.
    self._pos.y += wrld.get_scaled_gravity();
    let ydiff = self._pos.y - self.pos.1 as f32;
    let mut falling = true;
    let mut keep = true;
    if ydiff > 1.0 {
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
            //We have collided.
            if tile_below.kind() == &TileType::Goal {
              wrld.health -= 1;
              keep = false;
              break;
            }
            self._pos.y = self.pos.1 as f32;
            falling = false;
            break;
          }
        }
        self.pos.1 += 1;
        self.grid_pos = pos_to_grid_pos(&self.pos);
        y += 1;
      }
    }
    (falling, keep)
  }
  /** Returns keep */
  fn move_x(&mut self, wrld: &mut World) -> bool {
    let mut keep = true;
    let xdir: isize = if self.facing == Facing::Left { -1 } else { 1 };
    self._pos.x += xdir as f32 * WALKING_SPEED * wrld.dt;
    let xdiff = (self._pos.x - self.pos.0 as f32).abs();
    if xdiff > 1.0 {
      //Update position and check for collisions.
      let mut x: usize = 0;
      while x <= xdiff as usize {
        let rect = Rect::new(
          self.pos.0 + 1,
          self.pos.1,
          self.pos.0 + 1 + self.size.0,
          self.pos.1 + self.size.1,
        );

        if let Some(next_tile) = wrld.get_tile(
          &((self.grid_pos.0 as isize + xdir) as usize),
          &self.grid_pos.1,
        ) {
          // if wrld.frame % 60 == 0 {
          //   println!("{:?}, {:?}", next_tile.kind(), next_tile.grid_pos());
          // }
          if next_tile.collide(&rect) {
            //We have collided with a tile.
            if next_tile.kind() == &TileType::Goal {
              wrld.health -= 1;
              keep = false;
              break;
            }

            self._pos.x = self.pos.0 as f32;
            self.facing = if self.facing == Facing::Right {
              Facing::Left
            } else {
              Facing::Right
            };
            break;
          }
        }
        self.pos.0 = (self.pos.0 as isize + xdir) as usize;
        self.grid_pos = pos_to_grid_pos(&self.pos);
        x += 1;
      }
    }
    keep
  }

  /** Returns keep */
  pub fn update(&mut self, wrld: &mut World) -> bool {
    let (falling, mut keep) = self.move_y(wrld);
    if keep && !falling {
      keep = self.move_x(wrld);
    }

    if keep {
      self.draw(&wrld);
      return true;
    } else {
      return false;
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
