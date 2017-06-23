use data;
use errors::*;
use duck_husky_wedding::background::Background;
use duck_husky_wedding::player::Player;
use duck_husky_wedding::obstacle::Obstacle;
use duck_husky_wedding::try::Try;

use glm;
use moho::shape::Shape;
use moho::renderer::{Scene, Renderer, Texture, TextureLoader, TextureManager};
use moho::errors as moho_errors;

pub struct World<T> {
    background: Background<T>,
    obstacles: Vec<Obstacle<T>>,
}

impl<T> Clone for World<T> {
    fn clone(&self) -> Self {
        World {
            obstacles: self.obstacles.clone(),
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
        let obstacles = level
            .obstacles
            .iter()
            .map(|o| Obstacle::load(texture_manager, &game.ground, o))
            .collect::<Result<Vec<_>>>()?;

        Ok(World {
            background,
            obstacles,
        })
    }

    pub fn force(&self, player: &Player<T>) -> glm::DVec2 {
        let gravity = glm::dvec2(0., 1.);
        let mut force = gravity;
        let mut body = player.body.nudge(gravity + player.velocity);

        for o in &self.obstacles {
            if let Some(f) = o.mtv(&body) {
                force = force + f;
                body = body.nudge(f);
            }
        }

        force
    }
}

impl<'t, R: Renderer<'t>> Scene<R> for World<R::Texture> {
    fn show(&self, renderer: &mut R) -> moho_errors::Result<()> {
        renderer.show(&self.background)?;
        self.obstacles.iter().map(|o| renderer.show(o)).try()
    }
}
