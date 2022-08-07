use macroquad::{
  rand::RandomRange,
  texture::{load_image, load_texture, Texture2D},
};

use crate::{
  emath::i_to_xy,
  level::Level,
  tile::{BaseTile, TileType},
};

static ASSET_PATH: &str = "assets";
static TEXTURE_PATH: &str = "assets/textures";
pub struct Textures {
  pub bg_0: Texture2D,
  pub bg_1: Texture2D,
  pub bg_2: Texture2D,
  pub bg_3: Texture2D,
  pub blocker_down: Texture2D,
  pub blocker_up: Texture2D,
  pub border_bottom_left: Texture2D,
  pub border_bottom_right: Texture2D,
  pub border_bottom: Texture2D,
  pub border_left: Texture2D,
  pub border_right: Texture2D,
  pub border_top_left: Texture2D,
  pub border_top_right: Texture2D,
  pub border_top: Texture2D,
  pub build_down: Texture2D,
  pub build_up: Texture2D,
  pub empty: Texture2D,
  pub goal: Texture2D,
  pub spawn: Texture2D,
  pub terrain_center: Texture2D,
  pub terrain_down: Texture2D,
  pub terrain_up: Texture2D,
  pub turret_down: Texture2D,
  pub turret_up: Texture2D,
}

fn tex_path(name: &str) -> String {
  format!("{}/{}.png", TEXTURE_PATH, name)
}

pub async fn load_textures() -> Textures {
  Textures {
    bg_0: load_texture(&tex_path("bg_0")).await.unwrap(),
    bg_1: load_texture(&tex_path("bg_1")).await.unwrap(),
    bg_2: load_texture(&tex_path("bg_2")).await.unwrap(),
    bg_3: load_texture(&tex_path("bg_3")).await.unwrap(),
    blocker_down: load_texture(&tex_path("missing")).await.unwrap(),
    blocker_up: load_texture(&tex_path("missing")).await.unwrap(),
    border_bottom_left: load_texture(&tex_path("border_bottom_left")).await.unwrap(),
    border_bottom_right: load_texture(&tex_path("border_bottom_right"))
      .await
      .unwrap(),
    border_bottom: load_texture(&tex_path("border_bottom")).await.unwrap(),
    border_left: load_texture(&tex_path("border_left")).await.unwrap(),
    border_right: load_texture(&tex_path("border_right")).await.unwrap(),
    border_top_left: load_texture(&tex_path("border_top_left")).await.unwrap(),
    border_top_right: load_texture(&tex_path("border_top_right")).await.unwrap(),
    border_top: load_texture(&tex_path("border_top")).await.unwrap(),
    build_down: load_texture(&tex_path("build_down")).await.unwrap(),
    build_up: load_texture(&tex_path("build_up")).await.unwrap(),
    empty: load_texture(&tex_path("empty")).await.unwrap(),
    goal: load_texture(&tex_path("missing")).await.unwrap(),
    spawn: load_texture(&tex_path("missing")).await.unwrap(),
    terrain_center: load_texture(&tex_path("center")).await.unwrap(),
    terrain_down: load_texture(&tex_path("down")).await.unwrap(),
    terrain_up: load_texture(&tex_path("up")).await.unwrap(),
    turret_down: load_texture(&tex_path("missing")).await.unwrap(),
    turret_up: load_texture(&tex_path("missing")).await.unwrap(),
  }
}

pub async fn load_levels(textures: Textures) -> Level {
  let lvl = load_image(&format!("{}/levels/level1.png", ASSET_PATH))
    .await
    .unwrap();

  println!("w:{}, h:{}", lvl.width, lvl.height);
  let mut i = 0;
  let tiles = lvl
    .get_image_data()
    .iter()
    .map(|p| {
      let t = match p {
        [0, 0, 0, 255] => (TileType::BorderTopLeft, textures.border_top_left),
        [30, 30, 30, 255] => (TileType::BorderTop, textures.border_top),
        [60, 60, 60, 255] => (TileType::BorderTopRight, textures.border_top_right),
        [90, 90, 90, 255] => (TileType::BorderRight, textures.border_right),
        [120, 120, 120, 255] => (TileType::BorderBottomRight, textures.border_bottom_right),
        [150, 150, 150, 255] => (TileType::BorderBottom, textures.border_bottom),
        [180, 180, 180, 255] => (TileType::BorderBottomLeft, textures.border_bottom_left),
        [210, 210, 210, 255] => (TileType::BorderLeft, textures.border_left),
        [213, 0, 0, 255] => (TileType::Spawn, textures.spawn),
        [113, 0, 0, 255] => (TileType::Goal, textures.goal),
        [0, 200, 0, 255] => (TileType::TerrainUp, textures.terrain_up),
        [0, 155, 0, 255] => (TileType::TerrainCenter, textures.terrain_center),
        [0, 109, 0, 255] => (TileType::TerrainDown, textures.terrain_down),
        [0, 0, 200, 255] => (TileType::BuildUp, textures.build_up),
        [0, 0, 109, 255] => (TileType::BuildDown, textures.build_down),
        _ => {
          let r = RandomRange::gen_range(0, 7);
          (
            TileType::Empty,
            match r {
              1 => textures.bg_1,
              2 => textures.bg_1,
              3 => textures.bg_2,
              4 => textures.bg_2,
              5 => textures.bg_3,
              _ => textures.bg_0,
            },
          )
        }
      };

      let pos = i_to_xy(&(lvl.width as usize), &i);
      // println!("{}, {:?}", i, pos);
      i += 1;

      BaseTile {
        kind: t.0,
        texture: t.1,
        grid_pos: pos,
        index: &i - 1,
        size: (1, 1),
      }
    })
    .collect::<Vec<_>>();

  Level::new(lvl.width as usize, tiles)
}
