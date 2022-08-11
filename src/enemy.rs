use macroquad::{
  prelude::{vec2, Vec2, GREEN, WHITE},
  texture::{draw_texture_ex, DrawTextureParams, Texture2D},
};

use crate::{
  deb::DEBUG,
  emath::pos_to_grid_pos,
  rect::{Collidable, Rect},
  tile::TileType,
  tower::Towers,
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
  /** Offset and size in pixels. */
  hitbox: Rect,
  facing: Facing,
  rotation: f32,
  rect: Rect,
  draw_pos: Rect,
}

impl Enemy {
  pub fn new(pos: (usize, usize), hitbox: Rect, draw_pos: Rect, texture: Texture2D) -> Enemy {
    let mut rect = hitbox.clone();
    rect.left += pos.0;
    rect.top += pos.1;
    rect.right += pos.0;
    rect.bottom += pos.1;

    Enemy {
      _pos: vec2(pos.0 as f32, pos.1 as f32),
      pos,
      grid_pos: pos_to_grid_pos(&pos),
      texture,
      hitbox,
      rotation: 0.0,
      facing: Facing::Left,
      rect,
      draw_pos,
    }
  }
  pub fn update_rect(&mut self) {
    self.rect = Rect::new(
      self.pos.0 + self.hitbox.left,
      self.pos.1 + self.hitbox.top,
      self.pos.0 + self.hitbox.right,
      self.pos.1 + self.hitbox.bottom,
    );
  }

  /** Returns (falling, keep) */
  fn move_y(&mut self, wrld: &mut World, towers: &Towers) -> (bool, bool) {
    //Update position and check for collisions.
    self._pos.y += wrld.get_scaled_gravity();
    let ydiff = self._pos.y - self.pos.1 as f32;
    let mut falling = true;
    if ydiff > 1.0 {
      let mut y: usize = 0;

      while y <= ydiff as usize {
        let rect = Rect::new(
          self.pos.0 + self.hitbox.left,
          self.pos.1 + self.hitbox.top + 1,
          self.pos.0 + self.hitbox.right,
          self.pos.1 + self.hitbox.bottom + 1,
        );
        let rect_grid_pos = pos_to_grid_pos(&rect.tl());

        if let Some(x) = towers.get_collided_tower(&rect) {
          println!("{}", x);
          //We have collided with a tower.
          self._pos.y = self.pos.1 as f32;
          falling = false;
          break;
        }
        for offset in -1..2 {
          if let Some(tile_below) = wrld.get_tile(
            &((rect_grid_pos.0 as isize + offset) as usize),
            &(rect_grid_pos.1 + 1),
          ) {
            if tile_below.collide(&rect) {
              //We have collided.
              if tile_below.kind() == &TileType::Goal {
                wrld.health -= 1;
                return (true, false);
              }
              self._pos.y = self.pos.1 as f32;
              return (false, true);
            }
          }
        }
        self.pos.1 += 1;
        self.grid_pos = pos_to_grid_pos(&self.pos);
        y += 1;
      }
    }
    (falling, true)
  }

  /** Returns keep */
  fn move_x(&mut self, wrld: &mut World, towers: &Towers) -> bool {
    let mut keep = true;
    let xdir: isize = if self.facing == Facing::Left { -1 } else { 1 };
    self._pos.x += xdir as f32 * WALKING_SPEED * wrld.dt;
    // self._pos.x += (xdir as f32 * WALKING_SPEED * wrld.dt).max(1.0);
    let xdiff = (self._pos.x - self.pos.0 as f32).abs();
    if xdiff > 1.0 {
      //Update position and check for collisions.
      let mut x: usize = 0;
      while x <= xdiff as usize {
        let rect = Rect::new(
          (self.pos.0 as isize + self.hitbox.left as isize + 1 * xdir).max(0) as usize,
          self.pos.1 + self.hitbox.top,
          (self.pos.0 as isize + self.hitbox.right as isize + 1 * xdir).max(4) as usize,
          self.pos.1 + self.hitbox.bottom,
        );
        let rect_grid_pos = pos_to_grid_pos(&rect.tl());
        if let Some(_twr) = towers.get_collided_tower(&rect) {
          //We have collided with a tower.
          self._pos.x = self.pos.0 as f32;
          self.facing = if self.facing == Facing::Right {
            Facing::Left
          } else {
            Facing::Right
          };
          break;
        }

        if let Some(next_tile) = wrld.get_tile(
          &((rect_grid_pos.0 as isize + xdir) as usize),
          &rect_grid_pos.1,
        ) {
          // println!(
          //   "{:?}, {:?}, {:?}",
          //   next_tile.kind(),
          //   next_tile.grid_pos(),
          //   rect
          // );
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
        self.pos.0 = (self.pos.0 as isize + xdir).max(1) as usize;
        self.grid_pos = pos_to_grid_pos(&self.pos);
        x += 1;
      }
    }
    keep
  }

  /** Returns keep */
  pub fn update(&mut self, wrld: &mut World, towers: &Towers) -> bool {
    let (falling, mut keep) = self.move_y(wrld, towers);
    if keep && !falling {
      keep = self.move_x(wrld, towers);
    }

    if keep {
      self.update_rect();
      self.draw(&wrld);
      return true;
    } else {
      return false;
    }
  }

  pub fn draw(&self, wrld: &World) {
    if DEBUG.draw_rects {
      self.rect.debug_draw(GREEN);
    }
    let tl = self.draw_pos.tl();
    draw_texture_ex(
      self.texture,
      ((self.pos.0 + tl.0) as f32 + wrld.scroll_pos.x) * wrld.zoom,
      ((self.pos.1 + tl.1) as f32 + wrld.scroll_pos.y) * wrld.zoom,
      WHITE,
      DrawTextureParams {
        dest_size: Some(vec2(
          wrld.zoom * self.draw_pos.width() as f32,
          wrld.zoom * self.draw_pos.height() as f32,
        )),
        rotation: self.rotation,
        ..Default::default()
      },
    )
  }
}

impl Collidable for Enemy {
  fn get_rect(&self) -> &Rect {
    &self.hitbox
  }
  fn collide(&self, other: &impl Collidable) -> bool {
    self.hitbox.intersecting(other.get_rect())
  }
}
