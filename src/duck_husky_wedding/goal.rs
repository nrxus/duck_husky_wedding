use data;
use errors::*;

use glm;
use moho::errors as moho_errors;
use moho::renderer::{options, Renderer, Scene, Texture, TextureLoader, TextureManager};

use std::rc::Rc;

pub struct Goal<T> {
    texture: Rc<T>,
    pub dst: glm::IVec4,
}

impl<T: Texture> Goal<T> {
    pub fn load<'t, TL>(
        bl: glm::IVec2,
        data: &data::Image,
        texture_manager: &mut TextureManager<'t, TL>,
    ) -> Result<Self>
    where
        TL: TextureLoader<'t, Texture = T>,
    {
        let texture = data.texture.load(texture_manager)?;
        let dims: glm::IVec2 = data.out_size.into();
        let top_left = glm::ivec2(bl.x, 720 - bl.y - dims.y);
        let dst = glm::ivec4(top_left.x, top_left.y, dims.x, dims.y);
        Ok(Goal { texture, dst })
    }
}

impl<T> Clone for Goal<T> {
    fn clone(&self) -> Self {
        Goal {
            texture: self.texture.clone(),
            dst: self.dst,
        }
    }
}

impl<'t, R: Renderer<'t>> Scene<R> for Goal<R::Texture> {
    fn show(&self, renderer: &mut R) -> moho_errors::Result<()> {
        renderer.copy(&*self.texture, options::at(self.dst))
    }
}
