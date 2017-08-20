use errors::*;

use glm;
use moho::errors as moho_errors;
use moho::renderer::{options, ColorRGBA, Font, FontTexturizer, Renderer, Scene};

use std::rc::Rc;

pub struct Static<T> {
    idle: Rc<T>,
    hover: Rc<T>,
    dst: glm::IVec4,
    pub is_selected: bool,
}

impl<T> Clone for Static<T> {
    fn clone(&self) -> Static<T> {
        Static {
            idle: self.idle.clone(),
            hover: self.hover.clone(),
            dst: self.dst,
            is_selected: false,
        }
    }
}

impl<T> Static<T> {
    pub fn center_text<'t, F, FT>(
        text: &str,
        texturizer: &'t FT,
        font: &F,
        center: glm::IVec2,
    ) -> Result<Self>
    where
        F: Font,
        FT: FontTexturizer<'t, F, Texture = T>,
    {
        let dims = glm::to_ivec2(font.measure(text)?);
        let dst = glm::ivec4(center.x - dims.x / 2, center.y - dims.y / 2, dims.x, dims.y);

        Self::text_at(text, texturizer, font, dst)
    }

    pub fn text_at<'t, F, FT>(
        text: &str,
        texturizer: &'t FT,
        font: &F,
        dst: glm::IVec4,
    ) -> Result<Self>
    where
        FT: FontTexturizer<'t, F, Texture = T>,
    {
        let is_selected = false;
        let idle = Rc::new(texturizer
            .texturize(font, text, &ColorRGBA(255, 255, 255, 255))?);
        let hover = Rc::new(texturizer
            .texturize(font, text, &ColorRGBA(255, 255, 0, 255))?);
        Ok(Static {
            idle,
            hover,
            is_selected,
            dst,
        })
    }
}

impl<'t, R: Renderer<'t>> Scene<R> for Static<R::Texture> {
    fn show(&self, renderer: &mut R) -> moho_errors::Result<()> {
        let texture = if self.is_selected {
            &*self.hover
        } else {
            &*self.idle
        };

        renderer.copy(texture, options::at(&self.dst))
    }
}
