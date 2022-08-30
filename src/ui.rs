use macroquad::{
  prelude::{is_mouse_button_pressed, Color, MouseButton, Vec2, WHITE},
  shapes::draw_rectangle,
  text::draw_text,
  texture::{draw_texture_ex, DrawTextureParams},
};

use crate::wrld::World;

pub const UI_WIDTH: f32 = 210.0; //Should be divisible by 3.
pub const UI_HEIGHT: f32 = 300.0;
const BTN_MARGIN: f32 = 4.0;
const BTN_BOX: f32 = UI_WIDTH / 3.0;
const BTN_SIZE: f32 = BTN_BOX - 2.0 * BTN_MARGIN;

pub fn draw(wrld: &mut World) {
  let x_offset = wrld.scroll_pos.x * wrld.zoom;
  let y_offset = wrld.scroll_pos.y * wrld.zoom + 30.0;
  let check_mouse = true;
  draw_rectangle(
    -UI_WIDTH - 7.0 + x_offset,
    y_offset,
    UI_WIDTH + 4.0,
    UI_HEIGHT + 4.0,
    Color::from_rgba(151, 113, 74, 255),
  );
  draw_rectangle(
    -UI_WIDTH - 5.0 + x_offset,
    2.0 + y_offset,
    UI_WIDTH,
    UI_HEIGHT,
    Color::from_rgba(109, 86, 54, 255),
  );
  draw_text(
    &format!("Health: {}", wrld.health),
    -UI_WIDTH + x_offset,
    20.0 + y_offset,
    24.0,
    WHITE,
  );

  let btns_per_row = (UI_WIDTH / BTN_BOX).floor();
  let mut i: f32 = 0.0;
  let mut row: f32 = 0.0;
  let mp_x = wrld.mouse_pos.0 - x_offset;
  let mp_y = wrld.mouse_pos.1 - y_offset;

  for (twr, &texs) in &wrld.textures.tower_buttons {
    let x = -UI_WIDTH + (i % btns_per_row) * BTN_BOX;
    let y = UI_HEIGHT - BTN_BOX - row * BTN_BOX;
    let highlighted = if check_mouse {
      // let x_hit = mp_x > x && mp_x < x + BTN_BOX;
      // let y_hit = mp_y > y && mp_y < y + BTN_BOX;
      let inside = mp_x > x && mp_x < x + BTN_BOX && mp_y > y && mp_y < y + BTN_BOX;
      if inside && is_mouse_button_pressed(MouseButton::Left) {
        wrld.selected_tower_type = Some(twr);
      }
      inside
    } else {
      false
    };
    let t = match (highlighted, wrld.selected_tower_type == Some(twr)) {
      (false, false) => texs.normal,
      (true, false) => texs.highlighted,
      _ => texs.selected,
    };
    // println!("{:?}, x{} y{}", twr, x, y);
    draw_texture_ex(
      t,
      x + x_offset,
      y + BTN_MARGIN + y_offset,
      WHITE,
      DrawTextureParams {
        dest_size: Some(Vec2::new(BTN_SIZE, BTN_SIZE)),
        ..Default::default()
      },
    );
    i += 1.0;
    if i % btns_per_row == 0.0 {
      row += 1.0;
    }
  }
  // draw_texture_ex(texture, x, y, color, params)
}
