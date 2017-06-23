use errors::*;
use moho::animation::{self, animator, TileSheet};
use moho::renderer::{TextureLoader, TextureManager};

use glm;
use serde_yaml;

use std::fs::File;
use std::rc::Rc;
use std::time::Duration;

#[derive(Debug, Deserialize, Clone, Copy)]
pub struct Dimension {
    pub x: u32,
    pub y: u32,
}

#[derive(Debug, Deserialize)]
pub struct Sprite {
    pub texture: Texture,
    pub frames: u32,
    pub tiles: Dimension,
    pub duration: u64,
}

impl Sprite {
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
pub struct Texture(String);

impl Texture {
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
pub struct Player {
    pub animation: Sprite,
    pub idle_texture: Texture,
    pub out_size: Dimension,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Image {
    pub texture: Texture,
    pub out_size: Dimension,
}

#[derive(Debug, Deserialize)]
pub struct Ground {
    pub center: Texture,
    pub left: Texture,
    pub right: Texture,
    pub top: Texture,
    pub top_left: Texture,
    pub top_right: Texture,
    pub out_size: Dimension,
}

#[derive(Debug, Deserialize)]
pub struct Game {
    pub duck: Player,
    pub husky: Player,
    pub ground: Ground,
    pub background: Image,
}

impl Game {
    pub fn load(path: &'static str) -> Result<Game> {
        let f = File::open(path)?;
        serde_yaml::from_reader(&f).map_err(Into::into)
    }
}

impl<'a> From<Dimension> for glm::UVec2 {
    fn from(dim: Dimension) -> glm::UVec2 {
        let Dimension { x, y } = dim;
        glm::uvec2(x, y)
    }
}
