use rand::{seq::SliceRandom, thread_rng};

use crate::{enemy::Enemy, wrld::World};

pub fn spawn(wrld: &World) -> Enemy {
  let spawns = wrld.get_spawns();
  let spawn = spawns.choose(&mut thread_rng()).unwrap().to_owned();
  Enemy::new(spawn, (1, 1), wrld.textures.enemy)
}
