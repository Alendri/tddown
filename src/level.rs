use crate::{tile::Tile, wrld::World};
use macroquad::{
  prelude::{vec2, WHITE},
  texture::{draw_texture_ex, DrawTextureParams},
};

pub struct Level {
  index: u8,
  width: usize,
  height: usize,
  tiles: Vec<Tile>,
}

impl Level {
  pub fn get_tile(&self, x: usize, y: usize) -> &Tile {
    &self.tiles[x + y * self.width]
  }
  pub fn new(width: usize, tiles: Vec<Tile>) -> Level {
    Level {
      index: 0,
      width: width,
      height: tiles.len() / width,
      tiles,
    }
  }
  pub fn draw(&self, wrld: &World) {
    for x in 0..self.width {
      for y in 0..self.height {
        let t = self.get_tile(x, y);
        let scr_size = 32.0 * wrld.zoom as f32;
        draw_texture_ex(
          t.texture,
          ((x * 32) as f32 + wrld.scroll_pos.x) * wrld.zoom,
          ((y * 32) as f32 + wrld.scroll_pos.y) * wrld.zoom,
          WHITE,
          DrawTextureParams {
            dest_size: Some(vec2(scr_size, scr_size)),
            ..Default::default()
          },
        )
      }
    }
  }
}
