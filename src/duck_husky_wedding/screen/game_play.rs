use duck_husky_wedding::player::Player;
use duck_husky_wedding::game_data::SpriteData;
use errors::*;

use moho::input;
use moho::errors as moho_errors;
use moho::renderer::{Renderer, Scene, Show};
use moho::renderer::{Texture, TextureLoader, TextureManager};

use std::time::Duration;

pub struct GamePlay<T> {
    player: Player<T>,
}

impl<T> GamePlay<T> {
    pub fn load<'t, TL>(texture_manager: &mut TextureManager<'t, TL>,
                        data: SpriteData)
                        -> Result<Self>
        where T: Texture,
              TL: TextureLoader<'t, Texture = T>
    {
        let file_name: &str = &format!("media/sprites/{}", data.file_name);
        let texture = texture_manager.load(file_name)?;
        let player = Player::new(data, texture);
        Ok(GamePlay { player: player })
    }

    pub fn update(&mut self, delta: Duration, input: &input::State) -> Option<super::Kind> {
        self.player.animate(delta);
        self.player.update();
        None
    }
}

impl<'t, T, R> Scene<R> for GamePlay<T>
    where T: Texture,
          R: Renderer<'t, Texture = T> + Show
{
    fn show(&self, renderer: &mut R) -> moho_errors::Result<()> {
        renderer.show(&self.player)
    }
}
