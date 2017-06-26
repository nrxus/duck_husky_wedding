use data;
use errors::*;
use duck_husky_wedding::background::Background;
use duck_husky_wedding::player::Player;
use duck_husky_wedding::obstacle::Obstacle;
use duck_husky_wedding::try::Try;

use glm;
use moho::shape::{Rectangle, Shape};
use moho::renderer::{options, Scene, Renderer, Texture, TextureLoader, TextureManager};
use moho::errors as moho_errors;

use std::rc::Rc;

pub struct Goal<T> {
    texture: Rc<T>,
    body: Rectangle,
}

impl<T> Clone for Goal<T> {
    fn clone(&self) -> Self {
        Goal {
            texture: self.texture.clone(),
            body: self.body.clone(),
        }
    }
}

impl<'t, R: Renderer<'t>> Scene<R> for Goal<R::Texture> {
    fn show(&self, renderer: &mut R) -> moho_errors::Result<()> {
        let dst = glm::ivec4(
            self.body.top_left.x as i32,
            self.body.top_left.y as i32,
            self.body.dims.x as i32,
            self.body.dims.y as i32,
        );
        renderer.copy(&*self.texture, options::at(&dst))
    }
}

pub struct World<T> {
    background: Background<T>,
    obstacles: Vec<Obstacle<T>>,
    goal: Goal<T>,
}

impl<T> Clone for World<T> {
    fn clone(&self) -> Self {
        World {
            obstacles: self.obstacles.clone(),
            background: self.background.clone(),
            goal: self.goal.clone(),
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
        let obstacles: Vec<_> = level
            .obstacles
            .iter()
            .map(|o| Obstacle::load(texture_manager, &game.ground, o))
            .collect::<Result<_>>()?;
        let goal = {
            let texture = game.goal.texture.load(texture_manager)?;
            let tile_size: glm::DVec2 = game.ground.out_size.into();
            let goal_tl: glm::DVec2 = level.goal.into();
            let body = Rectangle {
                top_left: goal_tl * tile_size,
                dims: game.goal.out_size.into(),
            };
            Goal { texture, body }
        };

        Ok(World {
            background,
            obstacles,
            goal,
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
        renderer.show(&self.goal)?;
        self.obstacles.iter().map(|o| renderer.show(o)).try()
    }
}
