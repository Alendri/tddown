use macroquad::{
  prelude::{is_mouse_button_released, vec2, MouseButton, PINK, RED, WHITE},
  texture::{draw_texture_ex, DrawTextureParams, Texture2D},
  time::get_frame_time,
};
use std::{fmt::Display, iter::repeat_with};

use crate::{
  deb::DEBUG,
  effects::EffectKind,
  emath::grid_pos_to_pos,
  loading::Textures,
  rect::{Collidable, Rect},
  tile::TileType,
  wrld::World,
};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TowerType {
  BlockerDown,
  BlockerUp,
  Lava,
}
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Dir {
  Up,
  Down,
}

pub struct FrameDrawing {
  pub frames: Vec<Texture2D>,
  pub frame: usize,
  pub count: usize,
  pub timer: f32,
  pub timeout: f32,
}
impl FrameDrawing {
  pub fn new(frames: Vec<Texture2D>, timer: f32) -> FrameDrawing {
    FrameDrawing {
      count: frames.len(),
      frames,
      frame: 0,
      timer,
      timeout: timer,
    }
  }
  pub fn reset_timer(&mut self) {
    self.timer = self.timeout;
  }
  pub fn is_end(&self) -> bool {
    self.frame >= self.count - 1
  }
}

struct EffectSpawnData {
  pub timer: f32,
  pub kind: EffectKind,
  pub pos: (usize, usize),
}
impl EffectSpawnData {
  pub fn reset_timer(&mut self) {
    self.timer = 3.0;
  }
}

pub struct Tower {
  grid_pos: (usize, usize),
  kind: TowerType,
  /** Offset and size; in pixels units. */
  draw_pos: Rect,
  rect: Rect,
  texture: Texture2D,
  trigger: Option<Rect>,
  atlas: Option<FrameDrawing>,
  direction: Dir,
  spawn: Option<EffectSpawnData>,
}

impl Tower {
  pub fn get_texture(&mut self) -> Texture2D {
    if let Some(atlas) = &mut self.atlas {
      atlas.timer -= get_frame_time();
      if atlas.timer <= 0.0 {
        atlas.reset_timer();
        atlas.frame = (atlas.frame + 1) % atlas.count;
      }
      return atlas.frames[atlas.frame];
    }
    self.texture
  }
  pub fn draw(&mut self, wrld: &World) {
    let tl = self.draw_pos.tl();
    let width = self.draw_pos.width() as f32 * wrld.zoom;
    let height = self.draw_pos.height() as f32 * wrld.zoom;
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
      match self.kind {
        TowerType::BlockerDown | TowerType::BlockerUp => self.rect.debug_draw(RED),
        _ => self.rect.debug_draw(PINK),
      }
    }
  }
  pub fn get_spawn(&mut self, wrld: &World) -> Option<(EffectKind, (usize, usize))> {
    if let Some(s) = &mut self.spawn {
      s.timer -= wrld.dt;
      if s.timer <= 0.0 {
        s.reset_timer();
        return Some((s.kind.clone(), s.pos));
      }
    }
    None
  }
  pub fn new(textures: &Textures, kind: TowerType, grid_pos: (usize, usize)) -> Tower {
    let pos = grid_pos_to_pos(&grid_pos);
    match kind {
      TowerType::BlockerDown => Tower {
        grid_pos,
        kind,
        draw_pos: Rect::new(
          grid_pos.0 * 32,
          (grid_pos.1 + 1) * 32,
          (grid_pos.0 + 1) * 32,
          (grid_pos.1 + 3) * 32,
        ),
        rect: Rect::new(pos.0, pos.1 + 32, pos.0 + 32, pos.1 + 32 * 3),
        texture: textures.blocker_down,
        trigger: None,
        atlas: None,
        direction: Dir::Down,
        spawn: None,
      },
      TowerType::BlockerUp => Tower {
        grid_pos,
        kind,
        draw_pos: Rect::new(
          grid_pos.0 * 32,
          (grid_pos.1 - 2) * 32,
          (grid_pos.0 + 1) * 32,
          grid_pos.1 * 32,
        ),
        // draw_pos: Rect::new(grid_pos.0, grid_pos.1 - 2, grid_pos.0 + 1, grid_pos.1),
        rect: Rect::new(pos.0, pos.1 - 64, pos.0 + 32, pos.1),
        texture: textures.blocker_up,
        trigger: None,
        atlas: None,
        direction: Dir::Up,
        spawn: None,
      },
      TowerType::Lava => Tower {
        grid_pos,
        kind,
        draw_pos: Rect::new(
          grid_pos.0 * 32,
          grid_pos.1 * 32 + 20,
          (grid_pos.0 + 1) * 32,
          (grid_pos.1 + 1) * 32 + 20,
        ),
        rect: Rect::new(pos.0, pos.1 + 32, pos.0 + 32, pos.1 + 2 * 32),
        texture: textures.tower_lava[0],
        trigger: None,
        atlas: Some(FrameDrawing::new(textures.tower_lava.clone(), 0.4)),
        direction: Dir::Down,
        spawn: Some(EffectSpawnData {
          kind: EffectKind::LavaDrop,
          pos: (grid_pos.0, grid_pos.1 + 1),
          timer: 1.0,
        }),
      },
    }
  }
}

impl Display for Tower {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{:?}@{:?}", self.kind, self.grid_pos)
  }
}

impl Collidable for Tower {
  fn collide(&self, other: &impl Collidable) -> bool {
    match self.kind {
      TowerType::BlockerDown | TowerType::BlockerUp => self.rect.collide(other),
      _ => false,
    }
  }
  fn get_hitbox(&self) -> &Rect {
    &self.rect
  }
}

/////////////
/////////////
/////////////
/////////////

pub struct Towers {
  towers: Vec<Option<Tower>>,
}
impl Towers {
  pub fn new(wrld: &World) -> Towers {
    Towers {
      towers: repeat_with(|| None)
        .take(wrld.tiles.len())
        .collect::<Vec<_>>(),
    }
  }
  pub fn get_collided_tower(&self, other: &Rect) -> &Option<Tower> {
    let twr = self.towers.iter().find(|t| {
      if let Some(tower) = t {
        let c = tower.collide(other);
        // println!("Collide: {:?}: {}", tower.kind, c);
        c
      } else {
        false
      }
    });
    match twr {
      Some(t) => t,
      _ => &None,
    }
  }

  pub fn tower_dir(kind: &TowerType) -> Dir {
    match kind {
      TowerType::BlockerDown => Dir::Down,
      TowerType::BlockerUp => Dir::Up,
      TowerType::Lava => Dir::Down,
    }
  }
  pub fn get_spawns(&mut self, wrld: &World) -> Vec<(EffectKind, (usize, usize))> {
    self
      .towers
      .iter_mut()
      .filter_map(|t| {
        if let Some(tower) = t {
          tower.get_spawn(wrld)
        } else {
          None
        }
      })
      .collect()
  }
  pub fn update(&mut self, wrld: &World) {
    for tower in &mut self.towers {
      if let Some(t) = tower {
        t.draw(wrld);
      }
    }
    if !is_mouse_button_released(MouseButton::Left) {
      return;
    }
    if let Some(mouse_tile) = wrld.get_mouse_tile() {
      if let Some(selected_kind) = wrld.selected_tower_type {
        let kind = mouse_tile.kind();
        if kind == &TileType::BuildDown || kind == &TileType::BuildUp {
          let (x, y) = &mouse_tile.grid_pos();
          let tile_index = wrld.get_tile_index(x, y);
          if self.towers[tile_index].is_none() {
            //Buildable tile. Check type validity.
            let is_valid = match Towers::tower_dir(&selected_kind) {
              Dir::Up => kind == &TileType::BuildUp,
              _ => kind == &TileType::BuildDown,
            };
            if is_valid {
              self.towers[tile_index] = Some(Tower::new(&wrld.textures, selected_kind, (*x, *y)));
            }
          }
        }
      }
    }
  }
}
