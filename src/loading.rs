use enum_map::EnumMap;
use futures::future::join_all;
use macroquad::{
  rand::RandomRange,
  texture::{load_image, load_texture, Texture2D},
};
use std::fs::read_to_string;
use toml::{self, de::Error};

use crate::{
  emath::i_to_xy,
  level::{Level, LevelConfig, Levels},
  tile::{BaseTile, TileType},
  tower::TowerType,
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
  pub enemy: Texture2D,
  pub goal: Texture2D,
  pub lava_drop: Texture2D,
  pub lava_splash: Vec<Texture2D>,
  pub spawn: Texture2D,
  pub terrain_center: Texture2D,
  pub terrain_down: Texture2D,
  pub terrain_up: Texture2D,
  pub tower_lava: Vec<Texture2D>,
  pub turret_down: Texture2D,
  pub turret_up: Texture2D,
  pub tower_buttons: EnumMap<TowerType, ButtonTexs>,
}

#[derive(Debug, Clone, Copy)]
pub struct ButtonTexs {
  pub normal: Texture2D,
  pub highlighted: Texture2D,
  pub selected: Texture2D,
}

fn tex_path(name: &str) -> String {
  format!("{}/{}.png", TEXTURE_PATH, name)
}
fn tower_path(name: &str) -> String {
  format!("{}/towers/{}.png", TEXTURE_PATH, name)
}
/** Name with .ext */
fn level_path(name: &str) -> String {
  format!("{}/levels/{}", ASSET_PATH, name)
}

pub async fn load_textures() -> Textures {
  // let mut map = ;
  Textures {
    bg_0: load_texture(&tex_path("bg_0")).await.unwrap(),
    bg_1: load_texture(&tex_path("bg_1")).await.unwrap(),
    bg_2: load_texture(&tex_path("bg_2")).await.unwrap(),
    bg_3: load_texture(&tex_path("bg_3")).await.unwrap(),
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
    goal: load_texture(&tex_path("goal")).await.unwrap(),
    spawn: load_texture(&tex_path("hole1")).await.unwrap(),
    terrain_center: load_texture(&tex_path("center")).await.unwrap(),
    terrain_down: load_texture(&tex_path("down")).await.unwrap(),
    terrain_up: load_texture(&tex_path("up")).await.unwrap(),
    turret_down: load_texture(&tex_path("missing")).await.unwrap(),
    turret_up: load_texture(&tex_path("missing")).await.unwrap(),

    //BUILDABLE
    blocker_down: load_texture(&tower_path("blocker_down")).await.unwrap(),
    blocker_up: load_texture(&tower_path("blocker_up")).await.unwrap(),
    tower_lava: vec![
      load_texture(&tower_path("lava_0")).await.unwrap(),
      load_texture(&tower_path("lava_1")).await.unwrap(),
    ],

    //Effects
    lava_drop: load_texture(&tower_path("lava_drop1_5")).await.unwrap(),
    lava_splash: vec![
      load_texture(&tower_path("lava_drop1_6")).await.unwrap(),
      load_texture(&tower_path("lava_drop1_7")).await.unwrap(),
      load_texture(&tower_path("lava_drop1_8")).await.unwrap(),
      load_texture(&tower_path("lava_drop1_9")).await.unwrap(),
    ],

    //Enemies
    enemy: load_texture(&tex_path("enemy")).await.unwrap(),

    //UI
    tower_buttons: enum_map! {
      TowerType::BlockerDown => ButtonTexs {
        normal: load_texture(&tower_path("blocker_down_btn")).await.unwrap(),
        highlighted: load_texture(&tower_path("blocker_down_btn_highlight")).await.unwrap(),
        selected: load_texture(&tower_path("blocker_down_btn_selected")).await.unwrap()
      },
      TowerType::BlockerUp => ButtonTexs {
        normal: load_texture(&tower_path("blocker_up_btn")).await.unwrap(),
        highlighted: load_texture(&tower_path("blocker_up_btn_highlight")).await.unwrap(),
        selected: load_texture(&tower_path("blocker_up_btn_selected")).await.unwrap()
      },
      TowerType::Collector => ButtonTexs {
        normal: load_texture(&tower_path("collector_btn")).await.unwrap(),
        highlighted: load_texture(&tower_path("collector_btn_highlight")).await.unwrap(),
        selected: load_texture(&tower_path("collector_btn_selected")).await.unwrap()
      },
      TowerType::Lava => ButtonTexs {
        normal: load_texture(&tower_path("lava_btn")).await.unwrap(),
        highlighted: load_texture(&tower_path("lava_btn_highlight")).await.unwrap(),
        selected: load_texture(&tower_path("lava_btn_selected")).await.unwrap()
      },
    },
  }
}

pub async fn load_levels(textures: &Textures) -> Levels {
  let lvls = join_all(vec!["level1", "level2"].iter().map(|file_name| async {
    let lvl_data = load_image(&level_path(&format!("{}.png", file_name.clone())))
      .await
      .unwrap();

    println!("level w:{}, h:{}", lvl_data.width, lvl_data.height);
    let mut i = 0;
    let tiles = lvl_data
      .get_image_data()
      .iter()
      .map(|p| {
        let pos = i_to_xy(&(lvl_data.width as usize), &i);
        let mut basetile = BaseTile {
          kind: TileType::Empty,
          texture: textures.empty,
          grid_pos: pos,
          index: i,
          size: (1, 1),
        };
        match p {
          [0, 0, 0, 255] => {
            basetile.kind = TileType::BorderTopLeft;
            basetile.texture = textures.border_top_left;
          }
          [30, 30, 30, 255] => {
            basetile.kind = TileType::BorderTop;
            basetile.texture = textures.border_top;
          }
          [60, 60, 60, 255] => {
            basetile.kind = TileType::BorderTopRight;
            basetile.texture = textures.border_top_right;
          }
          [90, 90, 90, 255] => {
            basetile.kind = TileType::BorderRight;
            basetile.texture = textures.border_right;
          }
          [120, 120, 120, 255] => {
            basetile.kind = TileType::BorderBottomRight;
            basetile.texture = textures.border_bottom_right;
          }
          [150, 150, 150, 255] => {
            basetile.kind = TileType::BorderBottom;
            basetile.texture = textures.border_bottom;
          }
          [180, 180, 180, 255] => {
            basetile.kind = TileType::BorderBottomLeft;
            basetile.texture = textures.border_bottom_left;
          }
          [210, 210, 210, 255] => {
            basetile.kind = TileType::BorderLeft;
            basetile.texture = textures.border_left;
          }
          [213, 0, 0, 255] => {
            basetile.kind = TileType::Spawn;
            basetile.texture = textures.spawn;
          }
          [113, 0, 0, 255] => {
            basetile.kind = TileType::Goal;
            basetile.texture = textures.goal;
          }
          [0, 200, 0, 255] => {
            basetile.kind = TileType::TerrainUp;
            basetile.texture = textures.terrain_up;
          }
          [0, 155, 0, 255] => {
            basetile.kind = TileType::TerrainCenter;
            basetile.texture = textures.terrain_center;
          }
          [0, 109, 0, 255] => {
            basetile.kind = TileType::TerrainDown;
            basetile.texture = textures.terrain_down;
          }
          [0, 0, 200, 255] => {
            basetile.kind = TileType::BuildUp;
            basetile.texture = textures.build_up;
          }
          [0, 0, 109, 255] => {
            basetile.kind = TileType::BuildDown;
            basetile.texture = textures.build_down;
          }
          _ => {
            let r = RandomRange::gen_range(0, 7);
            basetile.kind = TileType::Empty;
            basetile.texture = match r {
              1 => textures.bg_1,
              2 => textures.bg_1,
              3 => textures.bg_2,
              4 => textures.bg_2,
              5 => textures.bg_3,
              _ => textures.bg_0,
            };
          }
        };

        i += 1;

        basetile
      })
      .collect::<Vec<_>>();
    let lvl_config = read_toml(file_name);

    // LevelConfig {
    //   enemies: Vec::new(),
    //   health: 100,
    //   towers: TowerSettings {
    //     block_down: Some(1),
    //     block_up: Some(1),
    //     lava: Some(1),
    //   },
    // }

    Level::new(lvl_data.width as usize, tiles, lvl_config)
  }))
  .await;

  Levels::new(lvls)
}

fn read_toml(file_name: &str) -> LevelConfig {
  let toml_str =
    read_to_string(level_path(&format!("{}.toml", file_name))).unwrap_or(String::new());
  let cfg: Result<LevelConfig, Error> = toml::from_str(&toml_str);
  if let Ok(lvl_config) = cfg {
    println!("cfg: {:?}", lvl_config);
    return lvl_config;
  }
  panic!("Could not read level config file! {:?}", cfg);
}
