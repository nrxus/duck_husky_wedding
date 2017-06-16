use errors::*;
use moho::animation::{self, animator, TileSheet};
use moho::renderer::{TextureLoader, TextureManager};

use glm;
use serde_yaml;

use std::fs::File;
use std::time::Duration;

use std::rc::Rc;

#[derive(Debug, Deserialize, Clone, Copy)]
pub struct DimensionData {
    pub x: u32,
    pub y: u32,
}

#[derive(Debug, Deserialize)]
pub struct SpriteData {
    pub texture: TextureData,
    pub frames: u32,
    pub tiles: DimensionData,
    pub duration: u64,
}

impl SpriteData {
    pub fn load<'t, TL: TextureLoader<'t>>(
        &self,
        texture_manager: &mut TextureManager<'t, TL>,
    ) -> Result<animation::Data<TL::Texture>> {
        let texture = self.texture.load(texture_manager)?;
        let sheet = TileSheet::new(self.tiles.into(), texture);
        let duration = Duration::from_millis(self.duration / self.frames as u64);
        let animator = animator::Data::new(self.frames, duration);
        Ok(animation::Data::new(animator, sheet))
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct TextureData(String);

impl TextureData {
    pub fn load<'t, TL: TextureLoader<'t>>(
        &self,
        texture_manager: &mut TextureManager<'t, TL>,
    ) -> Result<Rc<TL::Texture>> {
        texture_manager
            .load(&format!("media/sprites/{}", self.0))
            .map_err(Into::into)
    }
}

#[derive(Debug, Deserialize)]
pub struct PlayerData {
    pub animation: SpriteData,
    pub idle_texture: TextureData,
    pub out_size: DimensionData,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ImageData {
    pub texture: TextureData,
    pub out_size: DimensionData,
}

#[derive(Debug, Deserialize)]
pub struct GameData {
    pub duck: PlayerData,
    pub husky: PlayerData,
    pub ground: ImageData,
    pub background: ImageData,
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
