use errors::*;

use glm;
use moho::renderer::{ColorRGBA, Font, FontTexturizer};

use std::rc::Rc;

pub struct Static<T> {
    pub idle: Rc<T>,
    pub selected: Rc<T>,
    pub dims: glm::UVec2,
}

impl<T> Clone for Static<T> {
    fn clone(&self) -> Static<T> {
        Static {
            idle: Rc::clone(&self.idle),
            selected: Rc::clone(&self.selected),
            dims: self.dims,
        }
    }
}

impl<T> Static<T> {
    pub fn with_text<'t, F, FT>(text: &str, texturizer: &'t FT, font: &F) -> Result<Self>
    where
        F: Font,
        FT: FontTexturizer<'t, F, Texture = T>,
    {
        let dims = font.measure(text)?;

        let idle = texturizer
            .texturize(font, text, &ColorRGBA(255, 255, 255, 255))
            .map(Rc::new)?;
        let selected = texturizer
            .texturize(font, text, &ColorRGBA(255, 255, 0, 255))
            .map(Rc::new)?;

        Ok(Static {
            idle,
            selected,
            dims,
        })
    }
}
