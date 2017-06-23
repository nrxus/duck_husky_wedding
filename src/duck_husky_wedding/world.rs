use data;
use errors::*;
use duck_husky_wedding::background::Background;
use duck_husky_wedding::player::Player;
use duck_husky_wedding::obstacle::{self, Obstacle};

use glm;
use moho::shape::Shape;
use moho::renderer::{Scene, Renderer, Texture, TextureLoader, TextureManager};
use moho::errors as moho_errors;

pub struct World<T> {
    background: Background<T>,
    ground: Obstacle<T>,
    border: Obstacle<T>,
}

impl<T> Clone for World<T> {
    fn clone(&self) -> Self {
        World {
            ground: self.ground.clone(),
            border: self.border.clone(),
            background: self.background.clone(),
        }
    }
}

impl<T> World<T> {
    pub fn load<'t, TL>(
        texture_manager: &mut TextureManager<'t, TL>,
        level: &data::Level,
        game: &data::Game,
    ) -> Result<Self>
    where
        TL: TextureLoader<'t, Texture = T>,
        T: Texture,
    {
        let background = Background::load(texture_manager, &game.background)?;
        let texture = game.ground.center.load(texture_manager)?;
        let dims = game.ground.out_size.into();

        let tile = obstacle::Tile { texture, dims };

        let ground = Obstacle {
            tile: tile.clone(),
            tl: glm::ivec2(0, 720 - dims.y as i32),
            count: glm::uvec2(60, 1),
        };

        let border = Obstacle {
            tile: tile,
            tl: glm::ivec2(0, 0),
            count: glm::uvec2(1, 720 / dims.y),
        };

        Ok(World {
            ground,
            border,
            background,
        })
    }

    pub fn force(&self, player: &Player<T>) -> glm::DVec2 {
        let gravity = glm::dvec2(0., 1.);
        let mut force = gravity;
        let mut body = player.body.nudge(gravity + player.velocity);

        if let Some(f) = self.border.mtv(&body) {
            force = force + f;
            body = body.nudge(f);
        }

        if let Some(f) = self.ground.mtv(&body) {
            force = force + f;
        }
        force
    }
}

impl<'t, R: Renderer<'t>> Scene<R> for World<R::Texture> {
    fn show(&self, renderer: &mut R) -> moho_errors::Result<()> {
        renderer.show(&self.background)?;
        renderer.show(&self.ground)?;
        renderer.show(&self.border)
    }
}
