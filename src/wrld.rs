use macroquad::prelude::{
  is_key_down, is_mouse_button_down, mouse_position, mouse_wheel, vec2, KeyCode, MouseButton, Vec2,
};

const BASE_MOVEMENT_SPEED: f32 = 300.0;

pub struct World {
  pub scroll_pos: Vec2,
  _scroll_pos: Vec2,
  pub zoom: f32,
  _zoom: f32,
  pub sensitivity: f32,
  mouse_pos: (f32, f32),
  prev_mouse_pos: (f32, f32),
}

impl World {
  pub fn new() -> World {
    World {
      scroll_pos: vec2(0.0, 0.0),
      _scroll_pos: vec2(0.0, 0.0),
      zoom: 1.0,
      _zoom: 1.0,
      sensitivity: 0.001,
      mouse_pos: (0.0, 0.0),
      prev_mouse_pos: (0.0, 0.0),
    }
  }
  pub fn update(&mut self, dt: &f32) {
    // println!("{:?}", mouse_wheel());
    self.prev_mouse_pos = self.mouse_pos;
    self.mouse_pos = mouse_position();
    let mouse_diff = (
      self.mouse_pos.0 - self.prev_mouse_pos.0,
      self.mouse_pos.1 - self.prev_mouse_pos.1,
    );

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
    }

    if is_mouse_button_down(MouseButton::Right) {
      self.scroll_pos.x += mouse_diff.0;
      self.scroll_pos.y += mouse_diff.1;
      self._scroll_pos = self.scroll_pos;
    } else {
      let x_vel = match (
        is_key_down(KeyCode::A) || is_key_down(KeyCode::Left),
        is_key_down(KeyCode::D) || is_key_down(KeyCode::Right),
      ) {
        (true, false) => -BASE_MOVEMENT_SPEED * dt,
        (false, true) => BASE_MOVEMENT_SPEED * dt,
        _ => 0f32,
      };
      let y_vel = match (
        is_key_down(KeyCode::W) || is_key_down(KeyCode::Up),
        is_key_down(KeyCode::S) || is_key_down(KeyCode::Down),
      ) {
        (true, false) => BASE_MOVEMENT_SPEED * dt,
        (false, true) => -BASE_MOVEMENT_SPEED * dt,
        _ => 0f32,
      };
      if x_vel > 0.1 || x_vel < -0.1 || y_vel > 0.1 || y_vel < -0.1 {
        self._scroll_pos = vec2(self._scroll_pos.x + x_vel, self._scroll_pos.y + y_vel);
        self.scroll_pos = vec2(self._scroll_pos.x.round(), self._scroll_pos.y.round());
      }
    }
  }
}
