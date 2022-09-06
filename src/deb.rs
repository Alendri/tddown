use lazy_static::lazy_static;
use macroquad::{prelude::*, telemetry};

use crate::wrld::World;

lazy_static! {
  pub static ref DEBUG: DebugSettings = DebugSettings {
    // zero_offset_initial_camera: true,
    // draw_rects: true,
    ..Default::default()
  };
}

pub struct DebugSettings {
  pub draw_fps: bool,
  pub mouse: bool,
  pub state: bool,
  pub draw_rects: bool,
  pub zero_offset_initial_camera: bool,
  pub spawns: bool,
}
impl Default for DebugSettings {
  fn default() -> Self {
    DebugSettings {
      draw_fps: true,
      mouse: true,
      state: true,
      spawns: true,
      draw_rects: false,
      zero_offset_initial_camera: false,
    }
  }
}

pub struct DebugPrintSettings {
  pub x: f32,
  pub y: f32,
  pub prefix: Option<String>,
  pub fs: f32,
  pub color: Color,
}

impl DebugPrintSettings {
  pub fn new(x: f32, y: f32, prefix: String) -> DebugPrintSettings {
    DebugPrintSettings {
      x,
      y,
      prefix: Some(prefix),
      ..Default::default()
    }
  }
}
impl Default for DebugPrintSettings {
  fn default() -> Self {
    DebugPrintSettings {
      x: 10.0,
      y: 10.0,
      prefix: None,
      fs: 16.0,
      color: YELLOW,
    }
  }
}

pub fn draw_debug_texts(deb_state: &DebugSettings, wrld: &World) {
  let mut y = 10f32;
  if deb_state.draw_fps {
    let f = telemetry::frame();
    print(DebugPrintSettings::new(
      10.0,
      y,
      format!(
        "ft: {:.3} ({:.1}fps)",
        f.full_frame_time,
        1.0 / f.full_frame_time
      ),
    ));
    y += 15.0;
  }

  if deb_state.mouse {
    print(DebugPrintSettings::new(
      10.0,
      y,
      format!(
        "mouse: ({}, {})  grid:{:?}",
        wrld.mouse_pos.0,
        wrld.mouse_pos.1,
        wrld.get_mouse_grid(),
      ),
    ));
    y += 15.0;
  }

  if deb_state.state {
    print(DebugPrintSettings::new(
      10.0,
      y,
      format!(
        "hp:{}  speed:x{}   selected type:{:?}",
        wrld.health, wrld.speed, wrld.selected_tower_type
      ),
    ));
    y += 15.0;
  }
  if deb_state.spawns {
    print(DebugPrintSettings::new(
      10.0,
      y,
      format!(
        "spaned:{}/{}  ({}/sec)",
        wrld.get_lvl().spawner.get_spawned_count(),
        wrld.get_lvl().spawner.get_total_to_spawn(),
        wrld.get_lvl().spawner.get_spawns_per_second()
      ),
    ));
  }
}

pub fn print(s: DebugPrintSettings) {
  draw_text(&s.prefix.unwrap_or(String::new()), s.x, s.y, s.fs, s.color);
}
