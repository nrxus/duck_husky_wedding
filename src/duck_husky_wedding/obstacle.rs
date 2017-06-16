use moho::shape::{Intersect, Rectangle};
use moho::renderer::{options, Renderer, Scene};
use moho::errors as moho_errors;

use glm;

use std::rc::Rc;

pub struct Tile<T> {
    pub texture: Rc<T>,
    pub dims: glm::UVec2,
}

impl<T> Clone for Tile<T> {
    fn clone(&self) -> Self {
        Tile {
            texture: self.texture.clone(),
            dims: self.dims,
        }
    }
}

pub struct Obstacle<T> {
    pub tile: Tile<T>,
    pub tl: glm::IVec2,
    pub count: glm::UVec2,
}

impl<T> Clone for Obstacle<T> {
    fn clone(&self) -> Self {
        Obstacle {
            tile: self.tile.clone(),
            tl: self.tl,
            count: self.count,
        }
    }
}

impl<T> Obstacle<T> {
    pub fn mtv(&self, object: &Rectangle) -> Option<glm::DVec2> {
        let obstacle = Rectangle {
            top_left: glm::to_dvec2(self.tl),
            dims: glm::to_dvec2(self.tile.dims * self.count),
        };
        object.mtv(&obstacle)
    }
}

impl<'t, R: Renderer<'t>> Scene<R> for Obstacle<R::Texture> {
    fn show(&self, renderer: &mut R) -> moho_errors::Result<()> {
        let results = (0..self.count.x)
            .flat_map(|i| (0..self.count.y).map(move |j| (i, j)))
            .map(|(i, j)| {
                     glm::ivec4(self.tl.x + (self.tile.dims.x * i) as i32,
                                self.tl.y + (self.tile.dims.y * j) as i32,
                                self.tile.dims.x as i32,
                                self.tile.dims.y as i32)
                 })
            .map(|d| renderer.copy(&*self.tile.texture, options::at(&d)));
        for r in results {
            r?;
        }
        Ok(())
    }
}
