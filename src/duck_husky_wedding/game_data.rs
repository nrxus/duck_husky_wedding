use errors::*;

use glm;
use serde_yaml;

use std::fs::File;

#[derive(Debug,Deserialize,Clone,Copy)]
pub struct DimensionData {
    pub x: u32,
    pub y: u32,
}

#[derive(Debug, Deserialize)]
pub struct SpriteData {
    pub file_name: String,
    pub frames: u32,
    pub tiles: DimensionData,
}

#[derive(Debug, Deserialize)]
pub struct TextureData {
    pub file_name: String,
}

#[derive(Debug, Deserialize)]
pub struct PlayerData {
    pub animation: SpriteData,
    pub texture: TextureData,
    pub out_size: DimensionData,
}

#[derive(Debug,Deserialize)]
pub struct GameData {
    pub duck: PlayerData,
    pub husky: PlayerData,
}

impl GameData {
    pub fn load(path: &'static str) -> Result<GameData> {
        let f = File::open(path)?;
        serde_yaml::from_reader(&f).map_err(Into::into)
    }
}

impl<'a> From<DimensionData> for glm::UVec2 {
    fn from(data: DimensionData) -> glm::UVec2 {
        let DimensionData { x, y } = data;
        glm::uvec2(x, y)
    }
}
