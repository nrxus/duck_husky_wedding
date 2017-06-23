use duck_husky_wedding::try::Try;
use errors::*;
use data;

use glm;
use moho::errors as moho_errors;
use moho::renderer::{options, Renderer, Scene, Texture, TextureLoader, TextureManager};

use std::rc::Rc;

pub struct Background<T> {
    texture: Rc<T>,
    dimensions: glm::UVec2,
}

impl<T> Clone for Background<T> {
    fn clone(&self) -> Self {
        Background {
            texture: self.texture.clone(),
            dimensions: self.dimensions,
        }
    }
}

impl<T: Texture> Background<T> {
    pub fn load<'t, TL>(
        texture_manager: &mut TextureManager<'t, TL>,
        data: &data::Image,
    ) -> Result<Self>
    where
        TL: TextureLoader<'t, Texture = T>,
    {
        let texture = data.texture.load(texture_manager)?;
        let dimensions = data.out_size.into();
        Ok(Background {
            texture,
            dimensions,
        })
    }
}

impl<'t, R: Renderer<'t>> Scene<R> for Background<R::Texture> {
    fn show(&self, renderer: &mut R) -> moho_errors::Result<()> {
        (0..4)
            .map(|i| {
                glm::ivec4(
                    self.dimensions.x as i32 * i,
                    0,
                    self.dimensions.x as i32,
                    self.dimensions.y as i32,
                )
            })
            .map(|d| renderer.copy(&*self.texture, options::at(&d)))
            .try()
    }
}
