use super::Button;
use errors::*;

use glm;
use moho::errors as moho_errors;
use moho::shape::Rectangle;
use moho::renderer::{options, ColorRGBA, FontTexturizer, Renderer, Scene, Texture};

pub struct Static<T> {
    idle_texture: T,
    hover_texture: T,
    is_hovering: bool,
    pub body: Rectangle,
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
    pub fn from_text<'f, 't, R>(text: &str,
                                texturizer: &'t R,
                                font: &R::Font,
                                tl: glm::IVec2)
                                -> Result<Self>
        where T: Texture,
              R: FontTexturizer<'f, 't, Texture = T>
    {
        let idle_color = ColorRGBA(255, 255, 255, 255);
        let hover_color = ColorRGBA(255, 255, 0, 0);
        let idle_texture = texturizer.texturize(font, text, &idle_color)?;
        let hover_texture = texturizer.texturize(font, text, &hover_color)?;
        let dims = idle_texture.dims();
        let body = Rectangle {
            top_left: glm::to_dvec2(tl),
            dims: glm::to_dvec2(dims),
        };

        Ok(Self::new(idle_texture, hover_texture, body))
    }

    pub fn text_at<'f, 't, R>(text: &str,
                              texturizer: &'t R,
                              font: &R::Font,
                              body: Rectangle)
                              -> Result<Self>
        where T: Texture,
              R: FontTexturizer<'f, 't, Texture = T>
    {
        let idle_color = ColorRGBA(255, 255, 255, 255);
        let hover_color = ColorRGBA(255, 255, 0, 0);
        let idle_texture = texturizer.texturize(font, text, &idle_color)?;
        let hover_texture = texturizer.texturize(font, text, &hover_color)?;
        Ok(Self::new(idle_texture, hover_texture, body))
    }

    pub fn new(idle_texture: T, hover_texture: T, body: Rectangle) -> Self {
        Static {
            idle_texture: idle_texture,
            hover_texture: hover_texture,
            is_hovering: false,
            body: body,
        }
    }
}

impl<'t, T, R> Scene<R> for Static<T>
    where T: Texture,
          R: Renderer<'t, Texture = T>
{
    fn show(&self, renderer: &mut R) -> moho_errors::Result<()> {
        let texture = if self.is_hovering {
            &self.hover_texture
        } else {
            &self.idle_texture
        };
        let dst_rect = glm::to_ivec4(glm::dvec4(self.body.top_left.x,
                                                self.body.top_left.y,
                                                self.body.dims.x,
                                                self.body.dims.y));
        renderer.copy(texture, options::at(&dst_rect))
    }
}
