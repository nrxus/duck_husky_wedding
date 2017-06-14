use moho::shape::{Intersect, Rectangle};
use moho::renderer::{options, Renderer, Scene};
use moho::errors as moho_errors;

use glm;

use std::rc::Rc;

pub struct Tile<T> {
    pub texture: Rc<T>,
    pub dims: glm::UVec2,
}

pub struct Obstacle<T> {
    pub tile: Tile<T>,
    pub tl: glm::IVec2,
    pub count: glm::UVec2,
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
        for i in 0..self.count.x {
            for j in 0..self.count.y {
                let dims = &self.tile.dims;
                let dst = glm::ivec4(self.tl.x + (dims.x * i) as i32,
                                     self.tl.y + (dims.y * j) as i32,
                                     dims.x as i32,
                                     dims.y as i32);
                renderer.copy(&*self.tile.texture, options::at(&dst))?;
            }
        }
        Ok(())
    }
}
