use data;
use errors::*;

use glm;
use moho::errors as moho_errors;
use moho::renderer::{options, Renderer, Scene, Texture, TextureLoader, TextureManager};
use moho::shape::Rectangle;

use std::rc::Rc;

pub struct Goal<T> {
    texture: Rc<T>,
    pub body: Rectangle,
}

impl<T: Texture> Goal<T> {
    pub fn load<'t, TL>(
        bl: glm::DVec2,
        data: &data::Image,
        texture_manager: &mut TextureManager<'t, TL>,
    ) -> Result<Self>
    where
        TL: TextureLoader<'t, Texture = T>,
    {
        let texture = data.texture.load(texture_manager)?;
        let dims: glm::DVec2 = data.out_size.into();
        let top_left = glm::dvec2(bl.x, 720. - bl.y - dims.y);
        let body = Rectangle { top_left, dims };
        Ok(Goal { texture, body })
    }
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
