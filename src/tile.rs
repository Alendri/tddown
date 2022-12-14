use macroquad::{
  prelude::{vec2, GREEN, PURPLE, WHITE},
  texture::{draw_texture_ex, DrawTextureParams, Texture2D},
};

use crate::{
  emath::grid_pos_to_pos,
  rect::{Collidable, Rect},
  tower::{Dir, Towers},
  wrld::World,
};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TileType {
  BlockerDown,
  BlockerUp,
  BorderBottom,
  BorderBottomLeft,
  BorderBottomRight,
  BorderLeft,
  BorderRight,
  BorderTop,
  BorderTopLeft,
  BorderTopRight,
  BuildDown,
  BuildUp,
  Empty,
  Goal,
  Spawn,
  TerrainCenter,
  TerrainDown,
  TerrainUp,
  TurretDown,
  TurretUp,
}

#[derive(Clone, Copy)]
pub struct BaseTile {
  pub kind: TileType,
  pub texture: Texture2D,
  pub size: (usize, usize),
  pub grid_pos: (usize, usize),
  pub index: usize,
}
impl BaseTile {
  pub fn from_other(other: &BaseTile) -> BaseTile {
    BaseTile {
      kind: other.kind,
      texture: other.texture,
      size: other.size,
      grid_pos: other.grid_pos,
      index: other.index,
    }
  }
}

pub struct Tile {
  base: BaseTile,
  rect: Rect,
  passable: bool,
}
impl Collidable for Tile {
  fn get_hitbox(&self) -> &Rect {
    &self.rect
  }
  fn collide(&self, other: &impl Collidable) -> bool {
    // if !self.passable {
    //   println!(
    //     "Collide: {:?}  {:?}  {:?}   ||   {:?}",
    //     self.kind(),
    //     self.base.grid_pos,
    //     self.rect,
    //     other.get_rect()
    //   );
    // }
    !self.passable && self.rect.intersecting(other.get_hitbox())
  }
}
impl Tile {
  pub fn new(base: &BaseTile) -> Tile {
    let left = base.grid_pos.0 * 32;
    let top = base.grid_pos.1 * 32;
    let right = left + base.size.0 * 32;
    let bottom = top + base.size.1 * 32;
    Tile {
      base: BaseTile::from_other(base),
      rect: Rect::new(left, top, right, bottom),
      passable: base.kind == TileType::Empty,
    }
  }
  pub fn kind(&self) -> &TileType {
    &self.base.kind
  }
  pub fn pos(&self) -> (usize, usize) {
    grid_pos_to_pos(&self.base.grid_pos)
  }
  pub fn grid_pos(&self) -> (usize, usize) {
    self.base.grid_pos
  }

  pub fn debug_draw(&self) {
    if !self.passable {
      self.rect.debug_draw(PURPLE);
    }
  }

  pub fn draw(&self, wrld: &World) {
    let color = if let Some(selected_kind) = wrld.selected_tower_type {
      match (self.kind(), Towers::tower_dir(&selected_kind)) {
        (TileType::BuildDown, Dir::Down) => GREEN,
        (TileType::BuildUp, Dir::Up) => GREEN,
        _ => WHITE,
      }
    } else {
      WHITE
    };
    draw_texture_ex(
      self.base.texture,
      ((self.base.grid_pos.0 * 32) as f32 + wrld.scroll_pos.x) * wrld.zoom,
      ((self.base.grid_pos.1 * 32) as f32 + wrld.scroll_pos.y) * wrld.zoom,
      // Color::from_rgba(20, 20, 20, 255),
      color,
      DrawTextureParams {
        dest_size: Some(vec2(
          wrld.grid_size * self.base.size.0 as f32,
          wrld.grid_size * self.base.size.1 as f32,
        )),
        ..Default::default()
      },
    )
  }
}
