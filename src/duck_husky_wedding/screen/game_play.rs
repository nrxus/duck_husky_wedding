use duck_husky_wedding::player::Player;
use duck_husky_wedding::game_data::GameData;
use duck_husky_wedding::world::World;
use duck_husky_wedding::camera::ViewPort;
use errors::*;

use glm;
use moho::animation::{self, animator, TileSheet};
use moho::input;
use moho::errors as moho_errors;
use moho::renderer::{Renderer, Scene};
use moho::renderer::{options, Texture, TextureLoader, TextureManager};
use moho::shape::{Rectangle, Shape};

use std::rc::Rc;
use std::time::Duration;

pub enum PlayerKind {
    Duck,
    Husky,
}

pub struct GamePlay<T> {
    player: Player<T>,
    world: World<T>,
    background: Rc<T>,
    viewport: ViewPort,
}

pub struct Data<T> {
    tile: (Rc<T>, glm::DVec2),
    background: Rc<T>,
    data: GameData,
}

impl<T: Texture> Data<T> {
    pub fn load<'t, TL>(texture_manager: &mut TextureManager<'t, TL>,
                        data: GameData)
                        -> Result<Self>
        where TL: TextureLoader<'t, Texture = T>
    {
        let file_name: &str = &format!("media/sprites/{}", data.ground.file_name);
        let texture = texture_manager.load(file_name)?;
        let file_name: &str = &format!("media/environment/{}", data.background.file_name);
        let background = texture_manager.load(file_name)?;
        let dims = glm::dvec2(data.ground.out_size.x as f64, data.ground.out_size.y as f64);
        let tile = (texture, dims);
        Ok(Data {
               data,
               tile,
               background,
           })
    }

    pub fn activate<'t, TL>(&self,
                            texture_manager: &mut TextureManager<'t, TL>,
                            kind: PlayerKind)
                            -> Result<GamePlay<T>>
        where TL: TextureLoader<'t, Texture = T>
    {
        let player = match kind {
            PlayerKind::Duck => &self.data.duck,
            PlayerKind::Husky => &self.data.husky,
        };
        let body = Rectangle {
            top_left: glm::dvec2(100., 300.),
            dims: glm::dvec2(player.out_size.x as f64, player.out_size.y as f64),
        };

        let animation = &player.animation;
        let file_name: &str = &format!("media/sprites/{}", animation.file_name);
        let texture = texture_manager.load(file_name)?;
        let sheet = TileSheet::new(animation.tiles.into(), texture);
        let animator = animator::Data::new(animation.frames, Duration::from_millis(40));
        let animation = animation::Data::new(animator, sheet);

        let file_name: &str = &format!("media/sprites/{}", player.texture.file_name);
        let texture = texture_manager.load(file_name)?;

        let player = Player::new(animation, texture, body);
        let world = World::new((self.tile.0.clone(), self.tile.1));
        let background = self.background.clone();
        let viewport = ViewPort::new(glm::ivec2(1280, 720));
        Ok(GamePlay {
               player,
               world,
               background,
               viewport,
           })
    }
}

impl<T> GamePlay<T> {
    pub fn update(&mut self, delta: Duration, input: &input::State) -> Option<super::Kind> {
        self.player.process(input);
        let force = self.world.force(&self.player);
        self.player.update(force, delta);
        let center = self.player.body.center();
        self.viewport.center(glm::to_ivec2(center));
        None
    }
}

impl<'t, T, R> Scene<R> for GamePlay<T>
    where T: Texture,
          R: Renderer<'t, Texture = T>
{
    fn show(&self, renderer: &mut R) -> moho_errors::Result<()> {
        let mut camera = self.viewport.camera(renderer);
        camera
            .copy(&*self.background, options::at(&glm::ivec4(0, 0, 2560, 720)))?;
        camera.show(&self.world)?;
        camera.show(&self.player)
    }
}
