use data;
use errors::*;
use duck_husky_wedding::background::Background;
use duck_husky_wedding::collectable::{self, Collectable};
use duck_husky_wedding::goal::Goal;
use duck_husky_wedding::npc::Npc;
use duck_husky_wedding::player::Player;
use duck_husky_wedding::obstacle::Obstacle;
use duck_husky_wedding::try::Try;

use glm;
use moho::shape::Shape;
use moho::renderer::{Scene, Renderer, Texture, TextureLoader, TextureManager};
use moho::errors as moho_errors;

use std::time::Duration;

pub struct Data<T> {
    background: Background<T>,
    obstacles: Vec<Obstacle<T>>,
    goal: Goal<T>,
    npc_pos: glm::UVec2,
    collectables: Vec<collectable::Data<T>>,
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
        let tile_size: glm::IVec2 = game.ground.out_size.into();
        let background = Background::load(texture_manager, &game.background)?;
        let obstacles: Vec<_> = level
            .obstacles
            .iter()
            .map(|o| Obstacle::load(texture_manager, &game.ground, o))
            .collect::<Result<_>>()?;
        let goal = {
            let bl: glm::IVec2 = level.goal.into();
            Goal::load(bl * tile_size, &game.goal, texture_manager)
        }?;
        let npc_pos = glm::uvec2(goal.dst.x as u32, 720 - game.ground.out_size.y);
        let mut collectables = level
            .coins
            .iter()
            .map(|c| {
                let bl: glm::IVec2 = (*c).into();
                collectable::Data::load(bl * tile_size, &game.coin, texture_manager)
            })
            .collect::<Result<Vec<_>>>()?;

        let mut gems = level
            .gems
            .iter()
            .map(|g| {
                let bl: glm::IVec2 = (*g).into();
                collectable::Data::load(bl * tile_size, &game.gem, texture_manager)
            })
            .collect::<Result<Vec<_>>>()?;

        collectables.append(&mut gems);

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
        let collectables = self.collectables.iter().map(Collectable::new).collect();
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
            c.animate(duration);
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

impl<'t, R: Renderer<'t>> Scene<R> for World<R::Texture> {
    fn show(&self, renderer: &mut R) -> moho_errors::Result<()> {
        renderer.show(&self.background)?;
        renderer.show(&self.goal)?;
        self.obstacles.iter().map(|o| renderer.show(o)).try()?;
        self.collectables.iter().map(|c| renderer.show(c)).try()?;
        renderer.show(&self.npc)
    }
}
