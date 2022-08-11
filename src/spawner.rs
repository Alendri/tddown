use rand::{seq::SliceRandom, thread_rng};

use crate::{enemy::Enemy, rect::Rect, wrld::World};

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
