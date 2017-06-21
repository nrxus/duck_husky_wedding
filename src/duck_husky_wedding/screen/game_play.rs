use duck_husky_wedding::player::Player;
use duck_husky_wedding::world::World;
use duck_husky_wedding::camera::ViewPort;
use duck_husky_wedding::try::Try;
use game_data::GameData;
use errors::*;

use glm;
use moho::input;
use moho::errors as moho_errors;
use moho::renderer::{Renderer, Scene};
use moho::renderer::{options, Texture, TextureLoader, TextureManager};
use moho::shape::{Rectangle, Shape};

use std::rc::Rc;
use std::time::Duration;

struct Background<T> {
    texture: Rc<T>,
    dimensions: glm::UVec2,
}

impl<T> Clone for Background<T> {
    fn clone(&self) -> Self {
        Background {
            texture: self.texture.clone(),
            dimensions: self.dimensions,
        }
    }
}

impl<'t, T, R> Scene<R> for Background<T>
where
    T: Texture,
    R: Renderer<'t, Texture = T>,
{
    fn show(&self, renderer: &mut R) -> moho_errors::Result<()> {
        (0..4)
            .map(|i| {
                glm::ivec4(
                    self.dimensions.x as i32 * i,
                    0,
                    self.dimensions.x as i32,
                    self.dimensions.y as i32,
                )
            })
            .map(|d| renderer.copy(&*self.texture, options::at(&d)))
            .try()
    }
}

pub enum PlayerKind {
    Duck,
    Husky,
}

pub struct GamePlay<T> {
    player: Player<T>,
    world: World<T>,
    background: Background<T>,
    viewport: ViewPort,
}

pub struct Data<T> {
    world: World<T>,
    background: Background<T>,
    data: GameData,
}

impl<T: Texture> Data<T> {
    pub fn load<'t, TL>(
        texture_manager: &mut TextureManager<'t, TL>,
        data: GameData,
    ) -> Result<Self>
    where
        TL: TextureLoader<'t, Texture = T>,
    {
        let background = {
            let texture = data.background.texture.load(texture_manager)?;
            let dimensions = data.background.out_size.into();
            Background {
                texture,
                dimensions,
            }
        };
        let world = {
            let texture = data.ground.texture.load(texture_manager)?;
            World::new(texture, data.ground.out_size.into())
        };
        Ok(Data {
            data,
            world,
            background,
        })
    }

    pub fn activate<'t, TL>(
        &self,
        texture_manager: &mut TextureManager<'t, TL>,
        kind: PlayerKind,
    ) -> Result<GamePlay<T>>
    where
        TL: TextureLoader<'t, Texture = T>,
    {
        let player = match kind {
            PlayerKind::Duck => &self.data.duck,
            PlayerKind::Husky => &self.data.husky,
        };
        let body = Rectangle {
            top_left: glm::dvec2(100., 300.),
            dims: glm::dvec2(player.out_size.x as f64, player.out_size.y as f64),
        };

        let animation = player.animation.load(texture_manager)?;
        let texture = player.idle_texture.load(texture_manager)?;

        let player = Player::new(animation, texture, body);
        let world = self.world.clone();
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
where
    T: Texture,
    R: Renderer<'t, Texture = T>,
{
    fn show(&self, renderer: &mut R) -> moho_errors::Result<()> {
        let mut camera = self.viewport.camera(renderer);
        camera.show(&self.background)?;
        camera.show(&self.world)?;
        camera.show(&self.player)
    }
}
