use duck_husky_wedding::player::Player;
use duck_husky_wedding::game_data::GameData;
use duck_husky_wedding::world::World;
use errors::*;

use glm;
use moho::animation::{self, animator, TileSheet};
use moho::input;
use moho::errors as moho_errors;
use moho::renderer::{Renderer, Scene, Show};
use moho::renderer::{Texture, TextureLoader, TextureManager};
use moho::shape::{Rectangle, Shape};

use std::time::Duration;

pub struct GamePlay<T> {
    player: Player<T>,
    world: World<T>,
}

impl<T> GamePlay<T> {
    pub fn load<'t, TL>(texture_manager: &mut TextureManager<'t, TL>,
                        data: GameData)
                        -> Result<Self>
        where T: Texture,
              TL: TextureLoader<'t, Texture = T>
    {
        let animation = data.duck.animation;
        let file_name: &str = &format!("media/sprites/{}", animation.file_name);
        let texture = texture_manager.load(file_name)?;
        let sheet = TileSheet::new(animation.tiles.into(), texture);
        let animator = animator::Data::new(animation.frames, Duration::from_millis(40));
        let animation = animation::Data::new(animator, sheet);

        let file_name: &str = &format!("media/sprites/{}", data.duck.texture.file_name);
        let texture = texture_manager.load(file_name)?;
        let body = Rectangle {
            top_left: glm::dvec2(0., 300.),
            dims: glm::dvec2(data.duck.out_size.x as f64, data.duck.out_size.y as f64),
        };
        let player = Player::new(animation, texture, body);
        let world = World::load(texture_manager, data.ground)?;
        Ok(GamePlay {
               player: player,
               world: world,
           })
    }

    pub fn update(&mut self, delta: Duration, input: &input::State) -> Option<super::Kind> {
        self.player.update(delta, input);
        let force = self.world.force(&self.player.body);
        self.player.body = self.player.body.nudge(force);
        None
    }
}

impl<'t, T, R> Scene<R> for GamePlay<T>
    where T: Texture,
          R: Renderer<'t, Texture = T> + Show
{
    fn show(&self, renderer: &mut R) -> moho_errors::Result<()> {
        renderer.show(&self.world)?;
        renderer.show(&self.player)
    }
}
