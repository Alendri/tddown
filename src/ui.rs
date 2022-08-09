use macroquad::{
  prelude::{Color, WHITE},
  shapes::draw_rectangle,
  text::draw_text,
};

use crate::wrld::World;

pub const UI_WIDTH: f32 = 200.0;
pub const UI_HEIGHT: f32 = 300.0;

pub fn draw(wrld: &World) {
  let x_offset = wrld.scroll_pos.x * wrld.zoom;
  let y_offset = wrld.scroll_pos.y * wrld.zoom;
  draw_rectangle(
    -UI_WIDTH - 7.0 + x_offset,
    30.0 + y_offset,
    UI_WIDTH + 4.0,
    UI_HEIGHT + 4.0,
    Color::from_rgba(151, 113, 74, 255),
  );
  draw_rectangle(
    -UI_WIDTH - 5.0 + x_offset,
    32.0 + y_offset,
    UI_WIDTH,
    UI_HEIGHT,
    Color::from_rgba(109, 86, 54, 255),
  );
  draw_text(
    &format!("Health: {}", wrld.health),
    -UI_WIDTH + x_offset,
    50.0 + y_offset,
    24.0,
    WHITE,
  );
}
