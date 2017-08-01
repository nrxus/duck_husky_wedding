use super::Button;
use errors::*;

use glm;
use moho::errors as moho_errors;
use moho::shape::Rectangle;
use moho::renderer::{options, ColorRGBA, Font, FontTexturizer, Renderer, Scene};

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
        let dims = glm::to_dvec2(font.measure(text)?);
        let body = Rectangle {
            top_left: glm::dvec2(center.x as f64 - dims.x / 2., center.y as f64 - dims.y / 2.),
            dims: dims,
        };

        Self::text_at(text, texturizer, font, body)
    }

    pub fn from_text<'t, F, FT>(
        text: &str,
        texturizer: &'t FT,
        font: &F,
        tl: glm::IVec2,
    ) -> Result<Self>
    where
        F: Font,
        FT: FontTexturizer<'t, F, Texture = T>,
    {
        let dims = font.measure(text)?;
        let body = Rectangle {
            top_left: glm::to_dvec2(tl),
            dims: glm::to_dvec2(dims),
        };

        Self::text_at(text, texturizer, font, body)
    }

    pub fn text_at<'t, F, FT>(
        text: &str,
        texturizer: &'t FT,
        font: &F,
        body: Rectangle,
    ) -> Result<Self>
    where
        FT: FontTexturizer<'t, F, Texture = T>,
    {
        let is_hovering = false;
        let idle = Rc::new(texturizer
            .texturize(font, text, &ColorRGBA(255, 255, 255, 255))?);
        let hover = Rc::new(texturizer
            .texturize(font, text, &ColorRGBA(255, 255, 0, 255))?);
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
