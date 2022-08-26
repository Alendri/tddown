use enum_dispatch::enum_dispatch;
use macroquad::{
  prelude::{vec2, Vec2, RED, WHITE},
  texture::{draw_texture_ex, DrawTextureParams, Texture2D},
  time::get_frame_time,
};

use crate::{
  emath::{grid_pos_to_pos, pos_to_grid_pos},
  loading::Textures,
  rect::{Collidable, Rect},
  tower::FrameDrawing,
  wrld::World,
};

#[enum_dispatch(Effect)]
pub enum Effects {
  LavaDrop,
  LavaSplash,
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
  /** Returns keep: false and spawn: none. */
  pub fn abandon() -> EffectUpdateReturn {
    EffectUpdateReturn {
      spawn: None,
      keep: false,
    }
  }
}
#[enum_dispatch]
pub trait Effect {
  fn get_draw_pos(&self) -> Rect;
  fn get_pos(&self) -> &(usize, usize);
  fn get_rect(&self) -> &Rect;
  fn get_atlas(&mut self) -> &mut Option<FrameDrawing>;
  fn get_default_texture(&self) -> Texture2D;
  fn get_kind(&self) -> &EffectKind;
  fn update(&mut self, wrld: &World) -> EffectUpdateReturn;

  fn get_texture(&mut self) -> Texture2D {
    if let Some(atlas) = self.get_atlas() {
      atlas.timer -= get_frame_time();
      if atlas.timer <= 0.0 {
        atlas.reset_timer();
        atlas.frame += 1;
      }
      return atlas.frames[atlas.frame % atlas.count];
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

pub fn spawn_effect(
  effects: &mut Vec<Effects>,
  textures: &Textures,
  kind: EffectKind,
  grid_pos: (usize, usize),
) {
  match kind {
    EffectKind::LavaDrop => effects.push(Effects::LavaDrop(LavaDrop::new(grid_pos, textures))),
    EffectKind::LavaSplash => {
      effects.push(Effects::LavaSplash(LavaSplash::new(grid_pos, textures)))
    }
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
  pub fn new(grid_pos: (usize, usize), textures: &Textures) -> LavaDrop {
    let pos = grid_pos_to_pos(&grid_pos);
    LavaDrop {
      kind: EffectKind::LavaDrop,
      pos,
      _pos: vec2(pos.0 as f32, pos.1 as f32),
      hitbox: Rect::new(pos.0, pos.1, pos.0 + 32, pos.1 + 32),
      draw_pos: Rect::new(8, 10, 24, 38),
      atlas: None,
      default_texture: textures.lava_drop,
    }
  }
  /** Returns false when not falling */
  fn move_y(&mut self, wrld: &World) -> bool {
    //Update position and check for collisions.
    self._pos.y += wrld.get_scaled_gravity();
    let ydiff = self._pos.y - self.pos.1 as f32;
    // println!("{:?}, {:?}, {}", self._pos.y, self.pos, ydiff);
    if ydiff > 1.0 {
      let mut y: usize = 0;

      while y <= ydiff as usize {
        let rect = Rect::new(
          self.hitbox.left,
          self.hitbox.top + 1,
          self.hitbox.right,
          self.hitbox.bottom + 1,
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
        self.hitbox.top += 1;
        self.hitbox.bottom += 1;
        y += 1;
      }
      self._pos = vec2(self.pos.0 as f32, self.pos.1 as f32);
    }
    true
  }
}

impl Collidable for LavaDrop {
  fn get_hitbox(&self) -> &Rect {
    &self.hitbox
  }
  fn collide(&self, other: &impl Collidable) -> bool {
    self.hitbox.intersecting(other.get_hitbox())
  }
}
impl Effect for LavaDrop {
  fn get_atlas(&mut self) -> &mut Option<FrameDrawing> {
    &mut self.atlas
  }
  fn get_rect(&self) -> &Rect {
    &self.hitbox
  }
  fn get_default_texture(&self) -> Texture2D {
    self.default_texture
  }
  fn get_draw_pos(&self) -> Rect {
    self.draw_pos + &self.pos
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
      return EffectUpdateReturn::new(
        false,
        Some((EffectKind::LavaSplash, (self.pos.0, self.pos.1))),
      );
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
      draw_pos: Rect::new(0, 13, 32, 45),
      default_texture: textures.lava_splash[0],
      atlas: Some(FrameDrawing::new(textures.lava_splash.clone(), 0.1)),
    }
  }
}

impl Collidable for LavaSplash {
  fn get_hitbox(&self) -> &Rect {
    &self.hitbox
  }
  fn collide(&self, other: &impl Collidable) -> bool {
    self.hitbox.intersecting(other.get_hitbox())
  }
}
impl Effect for LavaSplash {
  fn get_atlas(&mut self) -> &mut Option<FrameDrawing> {
    &mut self.atlas
  }
  fn get_rect(&self) -> &Rect {
    &self.hitbox
  }
  fn get_default_texture(&self) -> Texture2D {
    self.default_texture
  }
  fn get_draw_pos(&self) -> Rect {
    self.draw_pos + &self.pos
  }
  fn get_kind(&self) -> &EffectKind {
    &self.kind
  }
  fn get_pos(&self) -> &(usize, usize) {
    &self.pos
  }
  fn update(&mut self, wrld: &World) -> EffectUpdateReturn {
    self.draw(wrld);
    // println!("Update LavaSplash {:?}", self.get_pos());
    self.get_hitbox().debug_draw(RED);

    if let Some(atlas) = self.get_atlas() {
      if atlas.is_end() {
        EffectUpdateReturn::abandon()
      } else {
        EffectUpdateReturn::keep()
      }
    } else {
      EffectUpdateReturn::abandon()
    }
  }
}

pub fn has_effect_collision(effects: &Vec<Effects>, hitbox: Rect) -> bool {
  effects.iter().any(|e| e.get_rect().collide(&hitbox))
}
