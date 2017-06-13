use duck_husky_wedding::player::Player;

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
    tiles: Vec<Tile<T>>,
}

impl<T> Ground<T> {
    fn new<'t>(tile: (Rc<T>, glm::DVec2)) -> Self
        where T: Texture
    {
        let (texture, dims) = tile;
        let tiles = (0..60)
            .map(|i| {
                let top_left = glm::dvec2(dims.x * i as f64, 720. - dims.y);
                let body = Rectangle {
                    top_left: top_left,
                    dims: dims,
                };
                Tile {
                    texture: texture.clone(),
                    body: body,
                }
            })
            .collect();
        Ground { tiles }
    }

    fn mtv(&self, mut body: Rectangle) -> Option<glm::DVec2> {
        let mut mtv = None;
        for t in &self.tiles {
            if let Some(f) = body.mtv(&t.body) {
                body = body.nudge(f);
                mtv = match mtv {
                    Some(of) => Some(of + f),
                    None => Some(f),
                }
            }
        }
        mtv
    }
}

impl<'t, R> Scene<R> for Ground<R::Texture>
    where R: Renderer<'t>
{
    fn show(&self, renderer: &mut R) -> moho_errors::Result<()> {
        let results = self.tiles
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

pub struct World<T> {
    ground: Ground<T>,
    border: Vec<Tile<T>>,
}

impl<T> World<T> {
    pub fn new<'t>(tile: (Rc<T>, glm::DVec2)) -> Self
        where T: Texture
    {
        let ground = Ground::new(tile.clone());
        let (texture, dims) = tile;
        let border = (1..9)
            .map(|i| {
                let top_left = glm::dvec2(0., 720. - dims.y * i as f64);
                let body = Rectangle {
                    top_left: top_left,
                    dims: dims,
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
