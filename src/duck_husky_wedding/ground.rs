use std::rc::Rc;

use glm;
use moho::errors as moho_errors;
use moho::renderer::{options, Renderer, Scene, Show};
use moho::shape::Rectangle;

pub struct Ground<T> {
    texture: Rc<T>,
    body: Rectangle,
}

impl<T> Ground<T> {
    pub fn new(texture: Rc<T>, body: Rectangle) -> Self {
        Ground {
            texture: texture,
            body: body,
        }
    }
}

impl<'t, R> Scene<R> for Ground<R::Texture>
    where R: Renderer<'t> + Show
{
    fn show(&self, renderer: &mut R) -> moho_errors::Result<()> {
        let dst_rect = glm::to_ivec4(glm::dvec4(self.body.top_left.x,
                                                self.body.top_left.y,
                                                self.body.dims.x,
                                                self.body.dims.y));
        renderer.copy(&*self.texture, options::at(&dst_rect))
    }
}
