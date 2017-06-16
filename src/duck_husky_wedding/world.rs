use duck_husky_wedding::player::Player;
use duck_husky_wedding::obstacle::{self, Obstacle};

use glm;
use moho::shape::Shape;
use moho::renderer::{Scene, Renderer, Texture};
use moho::errors as moho_errors;

use std::rc::Rc;

pub struct World<T> {
    ground: Obstacle<T>,
    border: Obstacle<T>,
}

impl<T> Clone for World<T> {
    fn clone(&self) -> Self {
        World {
            ground: self.ground.clone(),
            border: self.border.clone(),
        }
    }
}

impl<T> World<T> {
    pub fn new<'t>(texture: Rc<T>, dims: glm::UVec2) -> Self
        where T: Texture
    {
        let tile = obstacle::Tile {
            texture: texture,
            dims: dims,
        };

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

        World { ground, border }
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

impl<'t, R> Scene<R> for World<R::Texture>
    where R: Renderer<'t>
{
    fn show(&self, renderer: &mut R) -> moho_errors::Result<()> {
        renderer.show(&self.ground)?;
        renderer.show(&self.border)
    }
}
