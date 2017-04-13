use errors::*;

use glm;
use moho::errors as moho_errors;
use moho::input;
use moho::shape::{Rectangle, Shape};
use moho::renderer::{ColorRGBA, Font, FontTexturizer, Renderer, Scene, Texture};
use sdl2::mouse::MouseButton;

pub struct Button<T> {
    idle_texture: T,
    hover_texture: T,
    is_hovering: bool,
    body: Rectangle,
}

impl<T> Button<T> {
    pub fn from_text<'a, R, F>(text: &str, texturizer: &R, font: &F, tl: glm::UVec2) -> Result<Self>
        where T: Texture,
              F: Font,
              R: FontTexturizer<'a, Texture = T, Font = F>
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

    pub fn new(idle_texture: T, hover_texture: T, body: Rectangle) -> Self {
        Button {
            idle_texture: idle_texture,
            hover_texture: hover_texture,
            is_hovering: false,
            body: body,
        }
    }

    pub fn update(&mut self, input_state: &input::State) -> bool {
        let mouse = input_state.mouse_coords();
        self.is_hovering = self.body.contains(&glm::to_dvec2(mouse));
        self.is_hovering && input_state.did_click_mouse(MouseButton::Left)
    }
}

impl<T, R> Scene<R> for Button<T>
    where T: Texture,
          R: Renderer<Texture = T>
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
        renderer.copy(texture, Some(&dst_rect), None)
    }
}
