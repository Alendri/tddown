use macroquad::{
  prelude::{vec2, Vec2, WHITE},
  texture::{draw_texture_ex, DrawTextureParams, Texture2D},
  time::get_frame_time,
};

use crate::{
  emath::pos_to_grid_pos,
  loading::Textures,
  rect::{Collidable, Rect},
  tower::FrameDrawing,
  wrld::World,
};

pub enum Effects {
  LavaDrop(LavaDrop),
  LavaSplash(LavaSplash),
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum EffectKind {
  LavaDrop,
  LavaSplash,
}
pub struct EffectUpdateReturn {
  pub spawn: Option<(EffectKind, (usize, usize))>,
  pub keep: bool,
}
impl EffectUpdateReturn {
  pub fn new(keep: bool, spawn: Option<(EffectKind, (usize, usize))>) -> EffectUpdateReturn {
    EffectUpdateReturn { spawn, keep }
  }
  pub fn keep() -> EffectUpdateReturn {
    EffectUpdateReturn {
      spawn: None,
      keep: true,
    }
  }
}
pub trait Effect {
  fn get_draw_pos(&self) -> &Rect;
  fn get_pos(&self) -> &(usize, usize);
  fn get_atlas(&mut self) -> &mut Option<FrameDrawing>;
  fn get_default_texture(&self) -> Texture2D;
  fn get_kind(&self) -> &EffectKind;
  fn update(&mut self, wrld: &World) -> EffectUpdateReturn;

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
    // if DEBUG.draw_rects {
    //   self.get_hitbox().debug_draw(RED);
    // }
  }
}

//
//
//
//
//
//
//
// LAVADROP
//

pub struct LavaDrop {
  kind: EffectKind,
  pos: (usize, usize),
  _pos: Vec2,
  draw_pos: Rect,
  atlas: Option<FrameDrawing>,
  default_texture: Texture2D,
  hitbox: Rect,
}
impl LavaDrop {
  pub fn new(pos: (usize, usize), textures: &Textures) -> LavaDrop {
    LavaDrop {
      kind: EffectKind::LavaDrop,
      pos,
      _pos: vec2(pos.0 as f32, pos.1 as f32),
      hitbox: Rect::new(pos.0, pos.1, pos.0 + 32, pos.1 + 32),
      draw_pos: Rect::new(0, 0, 32, 32),
      atlas: None,
      default_texture: textures.lava_drop,
    }
  }
  /** Returns false when not falling */
  fn move_y(&mut self, wrld: &World) -> bool {
    //Update position and check for collisions.
    self._pos.y += wrld.get_scaled_gravity();
    let ydiff = self._pos.y - self.pos.1 as f32;
    if ydiff > 1.0 {
      let mut y: usize = 0;

      while y <= ydiff as usize {
        let rect = Rect::new(
          self.pos.0 + self.hitbox.left,
          self.pos.1 + self.hitbox.top + 1,
          self.pos.0 + self.hitbox.right,
          self.pos.1 + self.hitbox.bottom + 1,
        );
        let rect_grid_pos = pos_to_grid_pos(&rect.tl());

        if let Some(tile_below) = wrld.get_tile(&(rect_grid_pos.0 as usize), &(rect_grid_pos.1 + 1))
        {
          if tile_below.collide(&rect) {
            //We have collided.
            return false;
          }
        }

        self.pos.1 += 1;
        y += 1;
      }
    }
    true
  }
}

impl Collidable for LavaDrop {
  fn get_hitbox(&self) -> &Rect {
    &self.hitbox
  }
  fn collide(&self, _other: &impl Collidable) -> bool {
    false
  }
}
impl Effect for LavaDrop {
  fn get_atlas(&mut self) -> &mut Option<FrameDrawing> {
    &mut self.atlas
  }
  fn get_default_texture(&self) -> Texture2D {
    self.default_texture
  }
  fn get_draw_pos(&self) -> &Rect {
    &self.draw_pos
  }
  fn get_kind(&self) -> &EffectKind {
    &self.kind
  }
  fn get_pos(&self) -> &(usize, usize) {
    &self.pos
  }
  fn update(&mut self, wrld: &World) -> EffectUpdateReturn {
    let falling = self.move_y(wrld);

    if !falling {
      return EffectUpdateReturn::new(false, Some((EffectKind::LavaSplash, self.pos)));
    }

    self.draw(wrld);
    EffectUpdateReturn::keep()
  }
}

//
//
//
// LAVASPLASH
//

pub struct LavaSplash {
  kind: EffectKind,
  pos: (usize, usize),
  _pos: Vec2,
  draw_pos: Rect,
  atlas: Option<FrameDrawing>,
  default_texture: Texture2D,
  hitbox: Rect,
}
impl LavaSplash {
  pub fn new(pos: (usize, usize), textures: &Textures) -> LavaSplash {
    LavaSplash {
      kind: EffectKind::LavaSplash,
      pos,
      _pos: vec2(pos.0 as f32, pos.1 as f32),
      hitbox: Rect::new(pos.0, pos.1, pos.0 + 32, pos.1 + 32),
      draw_pos: Rect::new(0, 0, 32, 32),
      default_texture: textures.lava_splash[0],
      atlas: Some(FrameDrawing::new(textures.lava_splash.clone())),
    }
  }
}

impl Collidable for LavaSplash {
  fn get_hitbox(&self) -> &Rect {
    &self.hitbox
  }
  fn collide(&self, _other: &impl Collidable) -> bool {
    false
  }
}
impl Effect for LavaSplash {
  fn get_atlas(&mut self) -> &mut Option<FrameDrawing> {
    &mut self.atlas
  }
  fn get_default_texture(&self) -> Texture2D {
    self.default_texture
  }
  fn get_draw_pos(&self) -> &Rect {
    &self.draw_pos
  }
  fn get_kind(&self) -> &EffectKind {
    &self.kind
  }
  fn get_pos(&self) -> &(usize, usize) {
    &self.pos
  }
  fn update(&mut self, wrld: &World) -> EffectUpdateReturn {
    self.draw(wrld);
    EffectUpdateReturn::keep()
  }
}
