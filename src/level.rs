use enum_map::EnumMap;
use serde::Deserialize;

use crate::{
  emath::grid_pos_to_pos,
  spawner::{SpawnSpan, SpawnSpanSerialized, Spawner},
  tile::{BaseTile, TileType},
  tower::TowerType,
};

#[derive(Deserialize, Debug, Clone)]
pub struct TowerSettings {
  pub collector: Option<isize>,
  pub block_down: Option<isize>,
  pub block_up: Option<isize>,
  pub lava: Option<isize>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct LevelConfig {
  pub enemies: Vec<SpawnSpanSerialized>,
  pub health: Option<usize>,
  pub towers: TowerSettings,
}

#[derive(Clone)]
pub struct Level {
  pub index: u8,
  pub width: usize,
  pub height: usize,
  pub tiles: Vec<BaseTile>,
  pub health: usize,
  pub config: LevelConfig,
  pub twr_supply: EnumMap<TowerType, usize>,
  pub spawner: Spawner,
}

impl Level {
  pub fn new(width: usize, tiles: Vec<BaseTile>, config: LevelConfig) -> Level {
    Level {
      index: 0,
      width,
      height: tiles.len() / width,
      tiles,
      twr_supply: Level::calc_tower_supply(&config),
      spawner: Spawner::new(get_valid_spawn_spans(&config.enemies)),
      health: config.health.unwrap_or(100),
      config,
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
  fn calc_tower_supply(cfg: &LevelConfig) -> EnumMap<TowerType, usize> {
    enum_map! {
      TowerType::Collector => as_usize(cfg.towers.collector, 0),
      TowerType::BlockerDown => as_usize(cfg.towers.block_down, 0),
      TowerType::BlockerUp => as_usize(cfg.towers.block_up, 0),
      TowerType::Lava => as_usize(cfg.towers.lava, 0),
    }
  }
  pub fn get_tower_supply(&self, kind: &TowerType) -> usize {
    self.twr_supply[*kind]
  }
}

pub struct Levels {
  levels: Vec<Level>,
}

impl Levels {
  pub fn new(levels: Vec<Level>) -> Levels {
    Levels { levels }
  }
  pub fn get_level(&self, index: usize) -> Level {
    self.levels.get(index).unwrap_or(&self.levels[0]).clone()
  }
}

fn get_valid_spawn_spans(span_data: &Vec<SpawnSpanSerialized>) -> Vec<SpawnSpan> {
  let mut spans = span_data.clone();
  spans.retain(|s| s.count > 0 && s.time > 0.0);
  spans
    .iter_mut()
    .map(|s| SpawnSpan::new(s.time, s.count as usize))
    .collect()
}

fn as_usize(val: Option<isize>, default: usize) -> usize {
  if let Some(v) = val {
    if v >= 0 {
      return v as usize;
    }
  }
  default
}
