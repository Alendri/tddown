use crate::tile::BaseTile;

pub struct Level {
  index: u8,
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
}
