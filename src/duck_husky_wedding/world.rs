use data;
use errors::*;
use duck_husky_wedding::background::Background;
use duck_husky_wedding::cat::{self, Cat};
use duck_husky_wedding::collectable::{self, Collectable};
use duck_husky_wedding::goal::Goal;
use duck_husky_wedding::npc::Npc;
use duck_husky_wedding::player::Player;
use duck_husky_wedding::obstacle::Obstacle;
use utils::Try;

use glm;
use moho;
use moho::shape::Rectangle;
use moho::renderer::{options, Renderer, Scene, Texture, TextureLoader, TextureManager};

use std::rc::Rc;
use std::time::Duration;

pub struct Spike<T> {
    count: u32,
    texture: Rc<T>,
    dims: glm::UVec2,
    top_left: glm::IVec2,
    body: Rectangle,
    expand_left: Option<Rc<T>>,
    expand_right: Option<Rc<T>>,
    expand_bottom: Option<Rc<T>>,
}

impl<'t, R: Renderer<'t>> Scene<R> for Spike<R::Texture> {
    fn show(&self, renderer: &mut R) -> moho::errors::Result<()> {
        if let Some(ref g) = self.expand_left {
            let d = glm::ivec4(
                self.top_left.x - self.dims.x as i32,
                self.top_left.y,
                self.dims.x as i32,
                self.dims.y as i32,
            );
            renderer.copy(g.as_ref(), options::at(d))?;
            if let Some(ref g) = self.expand_bottom {
                let d = glm::ivec4(d.x, d.y + self.dims.y as i32, d.z, d.w);
                renderer.copy(g.as_ref(), options::at(d))?;
            }
        }

        (0..self.count)
            .map(|i| {
                glm::ivec4(
                    self.top_left.x + (self.dims.x * i) as i32,
                    self.top_left.y,
                    self.dims.x as i32,
                    self.dims.y as i32,
                )
            })
            .map(|d| {
                if let Some(ref g) = self.expand_bottom {
                    let rect = glm::ivec4(d.x, d.y + self.dims.y as i32, d.z, d.w);
                    renderer.copy(g.as_ref(), options::at(rect))?;
                }
                renderer.copy(&*self.texture, options::at(d))
            })
            .try()?;

        if let Some(ref g) = self.expand_right {
            let d = glm::ivec4(
                self.top_left.x + (self.dims.x * self.count) as i32,
                self.top_left.y,
                self.dims.x as i32,
                self.dims.y as i32,
            );
            renderer.copy(g.as_ref(), options::at(d))?;
            if let Some(ref g) = self.expand_bottom {
                let d = glm::ivec4(d.x, d.y + self.dims.y as i32, d.z, d.w);
                renderer.copy(g.as_ref(), options::at(d))?;
            }
        }

        Ok(())
    }
}

impl<T> Clone for Spike<T> {
    fn clone(&self) -> Self {
        Spike {
            top_left: self.top_left,
            count: self.count,
            dims: self.dims,
            expand_left: self.expand_left.clone(),
            expand_right: self.expand_right.clone(),
            expand_bottom: self.expand_bottom.clone(),
            texture: Rc::clone(&self.texture),
            body: self.body.clone(),
        }
    }
}

pub struct Data<T> {
    background: Background<T>,
    obstacles: Vec<Obstacle<T>>,
    spikes: Vec<Spike<T>>,
    goal: Goal<T>,
    npc_pos: glm::UVec2,
    collectables: Vec<collectable::Data<T>>,
    enemies: Vec<cat::Data<T>>,
}

pub struct World<T> {
    background: Background<T>,
    obstacles: Vec<Obstacle<T>>,
    goal: Goal<T>,
    pub spikes: Vec<Spike<T>>,
    pub collectables: Vec<Collectable<T>>,
    pub npc: Npc<T>,
    pub enemies: Vec<Cat<T>>,
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
                let mut bl: glm::IVec2 = (*c).into();
                bl = bl * tile_size;
                bl.y = 720 - bl.y;
                collectable::Data::load(bl, &game.coin, texture_manager)
            })
            .collect::<Result<Vec<_>>>()?;

        let mut gems = level
            .gems
            .iter()
            .map(|g| {
                let mut bl: glm::IVec2 = (*g).into();
                bl = bl * tile_size;
                bl.y = 720 - bl.y;
                collectable::Data::load(bl, &game.gem, texture_manager)
            })
            .collect::<Result<Vec<_>>>()?;

        collectables.append(&mut gems);
        let enemies = level
            .cats
            .iter()
            .map(|c| {
                let mut bl: glm::IVec2 = c.bottom_left.into();
                bl = bl * tile_size;
                bl.y = 720 - bl.y;
                let kind = match c.kind {
                    data::CatKind::Idle => cat::Kind::Idle,
                    data::CatKind::Moving(t) => cat::Kind::Moving {
                        total: t * tile_size.x as u32,
                        left: false,
                        current: 0,
                    },
                };
                cat::Data::load(bl, kind, &game.cat, texture_manager)
            })
            .collect::<Result<Vec<_>>>()?;

        let spikes = {
            let texture = game.spike.texture.load(texture_manager)?;
            let dims: glm::UVec2 = game.spike.out_size.into();

            level
                .spikes
                .iter()
                .map(|s| {
                    let mut bl: glm::IVec2 = s.bottom_left.into();
                    bl = bl * tile_size;
                    bl.y = 720 - bl.y;
                    let texture = Rc::clone(&texture);
                    let top_left = glm::ivec2(bl.x, bl.y - dims.y as i32);
                    let expand_left = s.left
                        .map(|l| l.load(&game.ground, texture_manager).unwrap());
                    let expand_right = s.right
                        .map(|l| l.load(&game.ground, texture_manager).unwrap());
                    let expand_bottom = s.bottom
                        .map(|l| l.load(&game.ground, texture_manager).unwrap());

                    Spike {
                        count: s.count,
                        dims,
                        texture,
                        top_left,
                        expand_left,
                        expand_right,
                        expand_bottom,
                        body: Rectangle {
                            top_left: glm::dvec2(top_left.x.into(), (top_left.y + 9).into()),
                            dims: glm::dvec2((dims.x * s.count).into(), (dims.y - 9).into()),
                        },
                    }
                })
                .collect()
        };

        Ok(Data {
            background,
            obstacles,
            goal,
            npc_pos,
            collectables,
            enemies,
            spikes,
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
        let enemies = self.enemies.iter().map(Cat::new).collect();
        Ok(World {
            npc,
            background: self.background.clone(),
            obstacles: self.obstacles.clone(),
            spikes: self.spikes.clone(),
            goal: self.goal.clone(),
            collectables,
            enemies,
        })
    }
}

impl<T> World<T> {
    pub fn update(&mut self, duration: Duration) {
        for c in &mut self.collectables {
            c.animate(duration);
        }
        for e in &mut self.enemies {
            e.update(duration);
        }
    }

    pub fn force(&self, player: &Player<T>) -> (glm::DVec2, bool, bool) {
        let mut force = glm::dvec2(0., 0.);
        let mut legs = player.legs().nudge(player.delta_pos);
        let mut body = player.body().nudge(player.delta_pos);
        let mut touch_legs = false;
        let mut touch_spikes = false;

        for o in &self.obstacles {
            if let Some(f) = o.mtv(&legs) {
                force = force + f;
                legs = legs.nudge(f);
                body = body.nudge(f);
                touch_legs = true;
            }
        }

        for o in &self.obstacles {
            if let Some(f) = o.mtv(&body) {
                force = force + f;
                legs = legs.nudge(f);
                body = body.nudge(f);
            }
        }

        for o in &self.obstacles {
            if let Some(f) = o.mtv(&body) {
                force = force + f;
                legs = legs.nudge(f);
                body = body.nudge(f);
            }
        }

        for s in &self.spikes {
            if let Some(f) = legs.mtv(&s.body) {
                force = force + f;
                legs = legs.nudge(f);
                body = body.nudge(f);
                touch_legs = true;
                touch_spikes = true;
            }
        }

        for s in &self.spikes {
            if let Some(f) = body.mtv(&s.body) {
                force = force + f;
                body = body.nudge(f);
            }
        }

        for s in &self.spikes {
            if let Some(f) = body.mtv(&s.body) {
                force = force + f;
                body = body.nudge(f);
            }
        }

        (force, touch_legs, touch_spikes)
    }
}

impl<'t, R: Renderer<'t>> Scene<R> for World<R::Texture> {
    fn show(&self, renderer: &mut R) -> moho::errors::Result<()> {
        renderer.show(&self.background)?;
        renderer.show(&self.goal)?;
        self.obstacles.iter().map(|o| renderer.show(o)).try()?;
        self.collectables.iter().map(|c| renderer.show(c)).try()?;
        self.enemies.iter().map(|c| renderer.show(c)).try()?;
        self.spikes.iter().map(|s| renderer.show(s)).try()?;
        renderer.show(&self.npc)
    }
}
