use super::Button;
use errors::*;

use glm;
use moho::errors as moho_errors;
use moho::shape::Rectangle;
use moho::renderer::{options, ColorRGBA, Font, FontTexturizer, Renderer, Scene, Texture};

use std::rc::Rc;

pub struct Static<T> {
    idle: Rc<T>,
    hover: Rc<T>,
    is_hovering: bool,
    pub body: Rectangle,
}

impl<T> Clone for Static<T> {
    fn clone(&self) -> Static<T> {
        Static {
            idle: self.idle.clone(),
            hover: self.hover.clone(),
            body: self.body.clone(),
            is_hovering: false,
        }
    }
}

impl<T> Button for Static<T> {
    fn body(&self) -> &Rectangle {
        &self.body
    }

    fn on_hover(&mut self, hovers: bool) {
        self.is_hovering = hovers
    }
}

impl<T> Static<T> {
    pub fn from_text<'f, 't, R>(
        text: &str,
        texturizer: &'t R,
        font: &R::Font,
        tl: glm::IVec2,
    ) -> Result<Self>
    where
        T: Texture,
        R: FontTexturizer<'f, 't, Texture = T>,
    {
        let dims = font.measure(text)?;
        let body = Rectangle {
            top_left: glm::to_dvec2(tl),
            dims: glm::to_dvec2(dims),
        };

        Self::text_at(text, texturizer, font, body)
    }

    pub fn text_at<'f, 't, R>(
        text: &str,
        texturizer: &'t R,
        font: &R::Font,
        body: Rectangle,
    ) -> Result<Self>
    where
        T: Texture,
        R: FontTexturizer<'f, 't, Texture = T>,
    {
        let is_hovering = false;
        let idle = Rc::new(texturizer
            .texturize(font, text, &ColorRGBA(255, 255, 255, 255))?);
        let hover = Rc::new(texturizer
            .texturize(font, text, &ColorRGBA(255, 255, 0, 0))?);
        Ok(Static {
            idle,
            hover,
            is_hovering,
            body,
        })
    }
}

impl<'t, R: Renderer<'t>> Scene<R> for Static<R::Texture> {
    fn show(&self, renderer: &mut R) -> moho_errors::Result<()> {
        let dst_rect = glm::to_ivec4(glm::dvec4(
            self.body.top_left.x,
            self.body.top_left.y,
            self.body.dims.x,
            self.body.dims.y,
        ));

        let texture = if self.is_hovering {
            &*self.hover
        } else {
            &*self.idle
        };

        renderer.copy(texture, options::at(&dst_rect))
    }
}
