use data;
use errors::*;
use duck_husky_wedding::background::Background;
use duck_husky_wedding::npc::Npc;
use duck_husky_wedding::player::Player;
use duck_husky_wedding::obstacle::Obstacle;
use duck_husky_wedding::try::Try;

use glm;
use moho::animation::{self, Animation};
use moho::shape::{Rectangle, Shape};
use moho::renderer::{options, Scene, Renderer, Texture, TextureLoader, TextureManager};
use moho::errors as moho_errors;

use std::rc::Rc;
use std::time::Duration;

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

#[derive(Debug)]
pub struct Collectable<T> {
    animation: Animation<T>,
    body: Rectangle,
}

#[derive(Debug)]
pub struct CollectableData<T> {
    animation: animation::Data<T>,
    body: Rectangle,
}

pub struct Data<T> {
    background: Background<T>,
    obstacles: Vec<Obstacle<T>>,
    goal: Goal<T>,
    npc_pos: glm::UVec2,
    collectables: Vec<CollectableData<T>>,
}

pub struct World<T> {
    background: Background<T>,
    obstacles: Vec<Obstacle<T>>,
    goal: Goal<T>,
    collectables: Vec<Collectable<T>>,
    pub npc: Npc<T>,
}

impl<T> Data<T> {
    pub fn load<'t, TL>(
        texture_manager: &mut TextureManager<'t, TL>,
        level: &data::Level,
        game: &data::Game,
    ) -> Result<Self>
    where
        T: Texture,
        TL: TextureLoader<'t, Texture = T>,
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
        let npc_pos = glm::uvec2(goal.body.top_left.x as u32, 720 - game.ground.out_size.y);
        let coin = CollectableData {
            animation: game.coin.animation.load(texture_manager)?,
            body: Rectangle {
                top_left: glm::dvec2(43., 43.),
                dims: game.coin.out_size.into(),
            },
        };

        let gem = CollectableData {
            animation: game.gem.animation.load(texture_manager)?,
            body: Rectangle {
                top_left: glm::dvec2(143., 43.),
                dims: game.coin.out_size.into(),
            },
        };

        let collectables = vec![coin, gem];

        Ok(Data {
            background,
            obstacles,
            goal,
            npc_pos,
            collectables,
        })
    }

    pub fn activate<'t, TL>(
        &self,
        npc: &data::Player,
        texture_manager: &mut TextureManager<'t, TL>,
    ) -> Result<World<T>>
    where
        TL: TextureLoader<'t, Texture = T>,
    {
        let mut tl = self.npc_pos;
        tl.y -= npc.out_size.y;
        let npc = Npc::load(npc, tl, texture_manager)?;
        let collectables = self.collectables
            .iter()
            .map(|c| {
                Collectable {
                    animation: c.animation.clone().start(),
                    body: c.body.clone(),
                }
            })
            .collect();
        Ok(World {
            npc,
            background: self.background.clone(),
            obstacles: self.obstacles.clone(),
            goal: self.goal.clone(),
            collectables,
        })
    }
}

impl<T> World<T> {
    pub fn update(&mut self, duration: Duration) {
        for mut c in &mut self.collectables {
            c.animation.animate(duration);
        }
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

impl<'t, R: Renderer<'t>> Scene<R> for Collectable<R::Texture> {
    fn show(&self, renderer: &mut R) -> moho_errors::Result<()> {
        let dst = glm::ivec4(
            self.body.top_left.x as i32,
            self.body.top_left.y as i32,
            self.body.dims.x as i32,
            self.body.dims.y as i32,
        );

        renderer.copy_asset(&self.animation.tile(), options::at(&dst))
    }
}

impl<'t, R: Renderer<'t>> Scene<R> for World<R::Texture> {
    fn show(&self, renderer: &mut R) -> moho_errors::Result<()> {
        renderer.show(&self.background)?;
        renderer.show(&self.goal)?;
        self.obstacles.iter().map(|o| renderer.show(o)).try()?;
        self.collectables.iter().map(|c| renderer.show(c)).try()?;
        renderer.show(&self.npc)
    }
}
