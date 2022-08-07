use macroquad::texture::Texture2D;

#[derive(Debug, PartialEq, Clone)]
pub enum TileType {
  BlockerDown,
  BlockerUp,
  BorderBottom,
  BorderBottomLeft,
  BorderBottomRight,
  BorderLeft,
  BorderRight,
  BorderTop,
  BorderTopLeft,
  BorderTopRight,
  BuildDown,
  BuildUp,
  Empty,
  Goal,
  Spawn,
  TerrainCenter,
  TerrainDown,
  TerrainUp,
  TurretDown,
  TurretUp,
}

pub struct Tile {
  pub kind: TileType,
  pub texture: Texture2D,
}
