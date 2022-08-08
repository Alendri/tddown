use crate::{
  emath::grid_pos_to_pos,
  tile::{BaseTile, TileType},
};

pub struct Level {
  pub index: u8,
  pub width: usize,
  pub height: usize,
  pub tiles: Vec<BaseTile>,
}

impl Level {
  pub fn new(width: usize, tiles: Vec<BaseTile>) -> Level {
    Level {
      index: 0,
      width: width,
      height: tiles.len() / width,
      tiles,
    }
  }
  pub fn find_spawns(&self) -> Vec<(usize, usize)> {
    let spawns: Vec<(usize, usize)> = self
      .tiles
      .iter()
      .filter_map(|t| match t.kind {
        TileType::Spawn => Some(grid_pos_to_pos(&t.grid_pos)),
        _ => None,
      })
      .collect();

    if spawns.len() == 0 {
      panic!("Invalid level: {}, could not find any spawns.", self.index);
    }

    spawns
  }
}
