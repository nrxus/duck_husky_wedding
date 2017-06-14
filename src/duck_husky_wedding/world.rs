use duck_husky_wedding::player::Player;
use duck_husky_wedding::obstacle::{self, Obstacle};

use glm;
use moho::shape::{Rectangle, Shape, Intersect};
use moho::renderer::{options, Scene, Renderer, Texture};
use moho::errors as moho_errors;

use std::rc::Rc;

pub struct Tile<T> {
    texture: Rc<T>,
    body: Rectangle,
}

pub struct Ground<T> {
    obstacle: Obstacle<T>,
}

impl<T> Ground<T> {
    fn new<'t>(tile: (Rc<T>, glm::UVec2)) -> Self
        where T: Texture
    {

        let (texture, dims) = tile;
        let tile = obstacle::Tile {
            texture: texture,
            dims: dims,
        };

        let obstacle = Obstacle {
            tile: tile,
            tl: glm::ivec2(0, 720 - dims.y as i32),
            count: glm::uvec2(60, 1),
        };
        Ground { obstacle }
    }

    fn mtv(&self, body: Rectangle) -> Option<glm::DVec2> {
        self.obstacle.mtv(&body)
    }
}

impl<'t, R> Scene<R> for Ground<R::Texture>
    where R: Renderer<'t>
{
    fn show(&self, renderer: &mut R) -> moho_errors::Result<()> {
        renderer.show(&self.obstacle)
    }
}

pub struct World<T> {
    ground: Ground<T>,
    border: Vec<Tile<T>>,
}

impl<T> World<T> {
    pub fn new<'t>(tile: (Rc<T>, glm::UVec2)) -> Self
        where T: Texture
    {
        let ground = Ground::new(tile.clone());
        let (texture, dims) = tile;
        let border = (1..9)
            .map(|i| {
                let top_left = glm::dvec2(0., 720. - (dims.y * i) as f64);
                let body = Rectangle {
                    top_left: top_left,
                    dims: glm::to_dvec2(dims),
                };
                Tile {
                    texture: texture.clone(),
                    body: body,
                }
            })
            .collect();
        World { ground, border }
    }

    pub fn force(&self, player: &Player<T>) -> glm::DVec2 {
        let gravity = glm::dvec2(0., 1.);
        let mut force = gravity;
        let mut body = player.body.nudge(gravity + player.velocity);

        {
            let mut mtv = None;
            for t in &self.border {
                if let Some(f) = body.mtv(&t.body) {
                    body = body.nudge(f);
                    mtv = match mtv {
                        Some(of) => Some(of + f),
                        None => Some(f),
                    }
                }
            }

            if let Some(f) = mtv {
                force = force + f;
            }
        }

        if let Some(f) = self.ground.mtv(body) {
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
        let results = self.border
            .iter()
            .map(|t| {
                     let tl = t.body.top_left;
                     let dims = t.body.dims;
                     let rect = glm::dvec4(tl.x, tl.y, dims.x, dims.y);
                     let dst_rect = glm::to_ivec4(rect);
                     renderer.copy(&*t.texture, options::at(&dst_rect))
                 });
        for r in results {
            r?
        }
        Ok(())
    }
}
