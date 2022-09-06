use rand::{seq::SliceRandom, thread_rng};
use serde::Deserialize;

use crate::{enemy::Enemy, rect::Rect, wrld::World};

#[derive(Deserialize, Debug, Clone)]
pub struct SpawnSpanSerialized {
  pub time: f32,
  pub count: isize,
}

#[derive(Debug, Clone)]
pub struct SpawnSpan {
  pub time: f32,
  pub count: usize,
  spawned: usize,
  timer: f32,
  time_between_spawns: f32,
}

impl SpawnSpan {
  pub fn new(time: f32, count: usize) -> SpawnSpan {
    SpawnSpan {
      time,
      count,
      spawned: 0,
      timer: 0.0,
      time_between_spawns: count as f32 / time,
    }
  }
  /** Returns (should_spawn, is_finished) */
  pub fn spawn(&mut self, dt: f32) -> (bool, bool) {
    self.timer += dt;
    if self.timer >= self.time_between_spawns {
      self.spawned += 1;
      self.timer = 0.0;
      return (true, self.spawned >= self.count);
    }
    (false, self.spawned >= self.count)
  }

  /** Returns spawns per second. */
  pub fn get_spawn_rate(&self) -> f32 {
    1.0 / self.time_between_spawns
  }
}

#[derive(Clone)]
pub struct Spawner {
  spans: Vec<SpawnSpan>,
  current_span_index: usize,
  spawned: usize,
  total_to_spawn: usize,
}

impl Spawner {
  pub fn new(spans: Vec<SpawnSpan>) -> Spawner {
    Spawner {
      total_to_spawn: spans.iter().fold(0, |acc, s| acc + s.count),
      spans,
      current_span_index: 0,
      spawned: 0,
    }
  }

  pub fn check_spawn(&mut self, dt: f32) -> bool {
    if self.current_span_index >= self.spans.len() {
      return false;
    }
    let (should_spawn, is_finished) = self.spans[self.current_span_index].spawn(dt);
    if is_finished {
      self.current_span_index += 1;
    }
    if should_spawn {
      self.spawned += 1;
    }
    should_spawn
  }

  pub fn get_total_to_spawn(&self) -> &usize {
    &self.total_to_spawn
  }
  pub fn get_spawned_count(&self) -> &usize {
    &self.spawned
  }
  pub fn get_spawns_per_second(&self) -> f32 {
    if self.current_span_index < self.spans.len() {
      return self.spans[self.current_span_index].get_spawn_rate();
    }
    0.0
  }
}

pub fn spawn(wrld: &World) -> Enemy {
  let spawns = wrld.get_spawns();
  let spawn = spawns.choose(&mut thread_rng()).unwrap().to_owned();
  Enemy::new(
    spawn,
    Rect::new(3, 12, 27, 32),
    Rect::new(3, 12, 27, 32),
    wrld.textures.enemy,
  )
}
