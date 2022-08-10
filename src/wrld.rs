use macroquad::{
  prelude::{
    get_last_key_pressed, is_key_down, is_key_released, is_mouse_button_down, mouse_position,
    mouse_wheel, vec2, KeyCode, MouseButton, Vec2,
  },
  time::get_frame_time,
};

use crate::{
  emath::xy_to_i, enemy::Enemy, level::Level, loading::Textures, spawner::spawn, tile::Tile,
  tower::TowerType, ui::UI_WIDTH,
};

const GRAVITY: f32 = 9.87;
const BASE_MOVEMENT_SPEED: f32 = 300.0;

pub struct World {
  level: Level,
  mouse_grid: Option<(usize, usize)>,
  prev_mouse_pos: (f32, f32),
  pub grid_size: f32,
  pub mouse_pos: (f32, f32),
  pub scroll_pos: Vec2,
  _scroll_pos: Vec2,
  pub sensitivity: f32,
  pub tiles: Vec<Tile>,
  pub zoom: f32,
  _zoom: f32,
  spawns: Vec<(usize, usize)>,
  pub textures: Textures,
  gravity: f32,
  pub frame: usize,
  pub health: usize,
  pub selected_tower_type: TowerType,
  pub speed: f32,
  pub dt: f32,
}

impl World {
  pub fn new(lvl: Level, texs: Textures) -> World {
    let tiles: Vec<Tile> = lvl.tiles.iter().map(|bt| Tile::new(bt)).collect();

    World {
      scroll_pos: vec2(UI_WIDTH + 32.0, 0.0),
      _scroll_pos: vec2(UI_WIDTH + 32.0, 0.0),
      zoom: 1.0,
      _zoom: 1.0,
      sensitivity: 0.005,
      mouse_pos: (0.0, 0.0),
      prev_mouse_pos: (0.0, 0.0),
      mouse_grid: None,
      grid_size: 32.0,
      spawns: lvl.find_spawns(),
      tiles,
      level: lvl,
      textures: texs,
      gravity: 0.0,
      frame: 0,
      health: 100,
      selected_tower_type: TowerType::BlockerUp,
      speed: 1.0,
      dt: 0.0,
    }
  }
  pub fn set_level(&mut self, lvl: Level) {
    self.tiles = lvl.tiles.iter().map(|bt| Tile::new(bt)).collect();
    self.level = lvl;
  }

  pub fn get_scaled_gravity(&self) -> f32 {
    self.gravity
  }
  pub fn get_mouse_grid(&self) -> Option<(usize, usize)> {
    self.mouse_grid
  }
  pub fn px_to_grid(&self, mut pos: (f32, f32)) -> Option<(usize, usize)> {
    pos.0 -= self.scroll_pos.x;
    pos.1 -= self.scroll_pos.y;
    let width = self.level.width as f32 * self.grid_size;
    let height = self.level.height as f32 * self.grid_size;
    if pos.0 < 0.0 || pos.0 > width || pos.1 < 0.0 || pos.1 > height {
      //Cursor is out of bounds.
      None
    } else {
      Some((
        (pos.0 / self.grid_size).floor() as usize,
        (pos.1 / self.grid_size).floor() as usize,
      ))
    }
  }
  pub fn get_spawns(&self) -> &Vec<(usize, usize)> {
    &self.spawns
  }
  pub fn get_tile(&self, x: &usize, y: &usize) -> Option<&Tile> {
    if x < &self.level.width && y < &self.level.height {
      return Some(&self.tiles[xy_to_i(&self.level.width, x, y)]);
    }
    None
  }
  pub fn get_mouse_tile(&self) -> Option<&Tile> {
    if let Some((x, y)) = self.mouse_grid {
      if x < self.level.width && y < self.level.height {
        return Some(&self.tiles[xy_to_i(&self.level.width, &x, &y)]);
      }
    }
    None
  }
  pub fn get_tile_index(&self, x: &usize, y: &usize) -> usize {
    xy_to_i(&self.level.width, x, y)
  }

  fn update_selected_tower_kind(&mut self) {
    if let Some(key) = get_last_key_pressed() {
      match key {
        KeyCode::Key1 => self.selected_tower_type = TowerType::BlockerUp,
        KeyCode::Key2 => self.selected_tower_type = TowerType::BlockerDown,
        KeyCode::Key3 => self.selected_tower_type = TowerType::Lava,
        _ => {}
      }
    }
  }
  fn update_mouse(&mut self) {
    self.prev_mouse_pos = self.mouse_pos;
    self.mouse_pos = mouse_position();

    //Calc grid position.
    self.mouse_grid = self.px_to_grid(mouse_position());
  }
  fn update_panning(&mut self) {
    let mouse_diff = (
      self.mouse_pos.0 - self.prev_mouse_pos.0,
      self.mouse_pos.1 - self.prev_mouse_pos.1,
    );
    if is_mouse_button_down(MouseButton::Right) {
      self.scroll_pos.x += mouse_diff.0 / self.zoom;
      self.scroll_pos.y += mouse_diff.1 / self.zoom;
      self._scroll_pos = self.scroll_pos;
    } else {
      let x_vel = match (
        is_key_down(KeyCode::A) || is_key_down(KeyCode::Left),
        is_key_down(KeyCode::D) || is_key_down(KeyCode::Right),
      ) {
        (true, false) => BASE_MOVEMENT_SPEED * get_frame_time(),
        (false, true) => -BASE_MOVEMENT_SPEED * get_frame_time(),
        _ => 0f32,
      };
      let y_vel = match (
        is_key_down(KeyCode::W) || is_key_down(KeyCode::Up),
        is_key_down(KeyCode::S) || is_key_down(KeyCode::Down),
      ) {
        (true, false) => BASE_MOVEMENT_SPEED * get_frame_time(),
        (false, true) => -BASE_MOVEMENT_SPEED * get_frame_time(),
        _ => 0f32,
      };
      if x_vel > 0.1 || x_vel < -0.1 || y_vel > 0.1 || y_vel < -0.1 {
        self._scroll_pos = vec2(self._scroll_pos.x + x_vel, self._scroll_pos.y + y_vel);
        self.scroll_pos = vec2(self._scroll_pos.x.round(), self._scroll_pos.y.round());
      }
    }
  }
  fn update_speed(&mut self) {
    if is_key_released(KeyCode::KpAdd) || is_key_released(KeyCode::Enter) {
      self.speed = (self.speed + 1.0).min(5.0).floor();
    }
    if is_key_released(KeyCode::KpSubtract) || is_key_released(KeyCode::Backspace) {
      self.speed = (self.speed - 1.0).max(0.5);
    }
  }

  pub fn update(&mut self, enemies: &mut Vec<Enemy>) {
    self.frame += 1;

    self.update_speed();

    self.dt = get_frame_time() * self.speed;

    self.update_selected_tower_kind();
    self.update_mouse();

    //UPDATE GRAVITY
    self.gravity = GRAVITY * (32.0 / 2.0) * self.dt;

    //ZOOMING
    //Positive; scroll up, negative; scroll down.
    let y = mouse_wheel().1;
    if y > 20.0 || y < -20.0 {
      self._zoom = (self._zoom + y * self.sensitivity).clamp(0.5, 3.0);

      self.zoom = if self._zoom < 0.75 {
        0.5
      } else {
        self._zoom.round()
      };
      self.grid_size = 32.0 * self.zoom;
    }

    if is_key_released(KeyCode::Space) {
      enemies.push(spawn(&self))
    }

    self.update_panning();

    //DRAW TILES
    for t in &self.tiles {
      t.draw(&self);
    }

    //UPDATE ENEMIES
    enemies.retain_mut(|e| e.update(self));
  }
}
