use macroquad::{
  prelude::{vec2, PINK, RED, WHITE},
  texture::{draw_texture_ex, DrawTextureParams, Texture2D},
  time::get_frame_time,
};

use crate::{
  deb::DEBUG,
  rect::Rect,
  tower::{Dir, FrameDrawing, TowerType},
  wrld::World,
};

pub trait Buildable {
  fn get_atlas(&mut self) -> Option<&mut FrameDrawing>;
  fn get_default_texture(&self) -> Texture2D;
  fn get_draw_pos(&self) -> &Rect;
  fn get_grid_pos(&self) -> &(usize, usize);
  fn get_hitbox(&self) -> &Rect;
  fn get_kind(&self) -> &TowerType;
  fn get_direction(&self) -> &Dir;

  fn get_texture(&mut self) -> Texture2D {
    if let Some(atlas) = self.get_atlas() {
      atlas.timer -= get_frame_time();
      if atlas.timer <= 0.0 {
        atlas.reset_timer();
        atlas.frame = (atlas.frame + 1) % atlas.count;
      }
      return atlas.frames[atlas.frame];
    }
    self.get_default_texture()
  }
  fn draw(&mut self, wrld: &World) {
    let dp = self.get_draw_pos();
    let tl = dp.tl();
    let width = dp.width() as f32 * wrld.zoom;
    let height = dp.height() as f32 * wrld.zoom;
    draw_texture_ex(
      self.get_texture(),
      (tl.0 as f32 + wrld.scroll_pos.x) * wrld.zoom,
      (tl.1 as f32 + wrld.scroll_pos.y) * wrld.zoom,
      WHITE,
      DrawTextureParams {
        dest_size: Some(vec2(width, height)),
        ..Default::default()
      },
    );
    if DEBUG.draw_rects {
      match self.get_kind() {
        TowerType::BlockerDown | TowerType::BlockerUp => self.get_hitbox().debug_draw(RED),
        _ => self.get_hitbox().debug_draw(PINK),
      }
    }
  }
}
