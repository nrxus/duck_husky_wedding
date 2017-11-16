use errors::*;

use glm;
use moho::renderer::{ColorRGBA, Font};

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
    pub fn with_text<F: Font<Texture = T>>(text: &str, font: &F) -> Result<Self> {
        let dims = font.measure(text)?;

        let idle = font.texturize(text, &ColorRGBA(255, 255, 255, 255))
            .map(Rc::new)?;
        let selected = font.texturize(text, &ColorRGBA(255, 255, 0, 255))
            .map(Rc::new)?;

        Ok(Static {
            idle,
            selected,
            dims,
        })
    }
}
