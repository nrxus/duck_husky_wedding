use data;
use errors::*;

use moho::errors as moho_errors;
use moho::renderer::{options, Renderer, Scene, TextureFlip, TextureLoader, TextureManager};

use glm;
use std::rc::Rc;

pub struct Npc<T> {
    texture: Rc<T>,
    dst: glm::IVec4,
}

impl<T> Clone for Npc<T> {
    fn clone(&self) -> Self {
        Npc {
            texture: self.texture.clone(),
            dst: self.dst,
        }
    }
}

impl<T> Npc<T> {
    pub fn load<'t, TL>(
        data: &data::Player,
        tl: glm::UVec2,
        texture_manager: &mut TextureManager<'t, TL>,
    ) -> Result<Self>
    where
        TL: TextureLoader<'t, Texture = T>,
    {
        let dims: glm::UVec2 = data.out_size.into();
        let dst = glm::ivec4(tl.x as i32, tl.y as i32, dims.x as i32, dims.y as i32);
        let texture = data.idle_texture.load(texture_manager)?;
        Ok(Npc { texture, dst })
    }

    pub fn x(&self) -> i32 {
        self.dst.x
    }
}

impl<'t, R: Renderer<'t>> Scene<R> for Npc<R::Texture> {
    fn show(&self, renderer: &mut R) -> moho_errors::Result<()> {
        renderer.copy(
            &*self.texture,
            options::at(&self.dst).flip(TextureFlip::Horizontal),
        )
    }
}
