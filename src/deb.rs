use macroquad::{prelude::*, telemetry};

use crate::wrld::World;

pub struct DebugState {
  pub draw_fps: bool,
  pub mouse: bool,
}
impl Default for DebugState {
  fn default() -> Self {
    DebugState {
      draw_fps: true,
      mouse: true,
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

pub fn draw_debugs(deb_state: &DebugState, wrld: &World) {
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
  }
}

pub fn print(s: DebugPrintSettings) {
  draw_text(&s.prefix.unwrap_or(String::new()), s.x, s.y, s.fs, s.color);
}
