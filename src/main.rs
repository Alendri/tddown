#![allow(dead_code)]
use deb::{draw_debugs, DebugState};
use enemy::Enemy;
use loading::{load_levels, load_textures};
use macroquad::prelude::*;
use wrld::World;

mod deb;
mod emath;
mod enemy;
mod level;
mod loading;
mod rect;
mod spawner;
mod tile;
mod wrld;

fn window_conf() -> Conf {
  Conf {
    window_title: "TD Down".to_owned(),
    window_height: 900,
    window_width: 1600,
    ..Default::default()
  }
}

#[macroquad::main(window_conf)]
async fn main() {
  let texs = load_textures().await;
  let lvl = load_levels(&texs).await;

  let mut wrld = World::new(lvl, texs);
  let mut enemies: Vec<Enemy> = Vec::new();
  let deb_state = DebugState {
    ..Default::default()
  };

  let mut dt: f32;

  loop {
    dt = get_frame_time();
    clear_background(BLACK);

    wrld.update(&dt, &mut enemies);

    draw_debugs(&deb_state, &wrld);
    next_frame().await
  }
}

pub fn line(a: &Vec2, b: &Vec2, color: Option<Color>, size: Option<f32>) {
  let color = color.unwrap_or(BLUE);
  let size = size.unwrap_or(1.0);
  draw_line(a.x, a.y, b.x, b.y, size, color);
}
