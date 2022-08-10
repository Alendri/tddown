use macroquad::{
  prelude::{is_mouse_button_released, vec2, MouseButton, WHITE},
  texture::{draw_texture_ex, DrawTextureParams, Texture2D},
};
use std::iter::repeat_with;

use crate::{
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
  frames: Vec<Texture2D>,
  frame: usize,
  count: usize,
  timer: f32,
}
impl FrameDrawing {
  pub fn new(frames: Vec<Texture2D>) -> FrameDrawing {
    FrameDrawing {
      count: frames.len(),
      frames,
      frame: 0,
      timer: 0.4,
    }
  }
}

struct Tower {
  grid_pos: (usize, usize),
  kind: TowerType,
  /** Offset and size; in grid units. */
  draw_pos: Rect,
  rect: Rect,
  texture: Texture2D,
  trigger: Option<Rect>,
  atlas: Option<FrameDrawing>,
  direction: Dir,
}

impl Tower {
  pub fn draw(&self, wrld: &World) {
    let t = match &self.atlas {
      Some(frames) => frames.frames[frames.frame],
      None => self.texture,
    };
    let tl = self.draw_pos.tl();
    let width = self.draw_pos.width() as f32 * wrld.grid_size;
    let height = self.draw_pos.height() as f32 * wrld.grid_size;
    draw_texture_ex(
      t,
      (tl.0 as f32 * 32.0 + wrld.scroll_pos.x) * wrld.zoom,
      (tl.1 as f32 * 32.0 + wrld.scroll_pos.y) * wrld.zoom,
      WHITE,
      DrawTextureParams {
        dest_size: Some(vec2(width, height)),
        ..Default::default()
      },
    )
  }
  pub fn new(textures: &Textures, kind: TowerType, grid_pos: (usize, usize)) -> Tower {
    let pos = grid_pos_to_pos(&grid_pos);
    match kind {
      TowerType::BlockerDown => Tower {
        grid_pos,
        kind,
        draw_pos: Rect::new(grid_pos.0, grid_pos.1 + 1, grid_pos.0 + 1, grid_pos.1 + 3),
        rect: Rect::new(pos.0, pos.1 + 32, pos.0 + 32, pos.1 + 32 * 3),
        texture: textures.blocker_down,
        trigger: None,
        atlas: None,
        direction: Dir::Down,
      },
      TowerType::BlockerUp => Tower {
        grid_pos,
        kind,
        draw_pos: Rect::new(grid_pos.0, grid_pos.1 - 2, grid_pos.0 + 1, grid_pos.1),
        rect: Rect::new(pos.0, pos.1 - 64, pos.0 + 32, pos.1),
        texture: textures.blocker_up,
        trigger: None,
        atlas: None,
        direction: Dir::Up,
      },
      TowerType::Lava => Tower {
        grid_pos,
        kind,
        draw_pos: Rect::new(grid_pos.0, grid_pos.1 + 1, grid_pos.0 + 1, grid_pos.1 + 2),
        rect: Rect::new(pos.0, pos.1 + 32, pos.0 + 32, pos.1 + 2 * 32),
        texture: textures.tower_lava[0],
        trigger: None,
        atlas: Some(FrameDrawing::new(textures.tower_lava.clone())),
        direction: Dir::Down,
      },
    }
  }
}

impl Collidable for Tower {
  fn collide(&self, other: &impl Collidable) -> bool {
    match self.kind {
      TowerType::BlockerDown | TowerType::BlockerUp => self.rect.collide(other),
      _ => false,
    }
  }
  fn get_rect(&self) -> &Rect {
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
  pub fn update(&mut self, wrld: &World) {
    for tower in &self.towers {
      if let Some(t) = tower {
        t.draw(wrld);
      }
    }
    if !is_mouse_button_released(MouseButton::Left) {
      return;
    }
    if let Some(mouse_tile) = wrld.get_mouse_tile() {
      if mouse_tile.kind() == &TileType::BuildDown || mouse_tile.kind() == &TileType::BuildUp {
        let (x, y) = &mouse_tile.grid_pos();
        let tile_index = wrld.get_tile_index(x, y);
        if self.towers[tile_index].is_none() {
          //Buildable tile.
          self.towers[tile_index] =
            Some(Tower::new(&wrld.textures, TowerType::BlockerUp, (*x, *y)));
        }
      }
    }
  }
}
