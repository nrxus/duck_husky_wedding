use super::player::Player;

use glm;
use moho::errors as moho_errors;
use moho::input;
use moho::shape::{Rectangle, Shape};
use moho::renderer::{ColorRGBA, Font, FontTexturizer, Renderer, Scene, Texture};
use sdl2::mouse::MouseButton;

use std::rc::Rc;

pub struct Button<F, T> {
    font: Rc<F>,
    text: &'static str,
    is_hovering: bool,
    body: Rectangle,
    on_click: Box<FnMut(&mut Player<T>) -> ()>,
}

impl<F: Font, T> Button<F, T> {
    pub fn new(text: &'static str,
               font: Rc<F>,
               tl: glm::UVec2,
               on_click: Box<FnMut(&mut Player<T>)>)
               -> Self {
        let dims = font.measure(text).unwrap();
        let body = Rectangle {
            top_left: glm::to_dvec2(tl),
            dims: glm::to_dvec2(dims),
        };
        Button {
            font: font,
            text: text,
            is_hovering: false,
            body: body,
            on_click: on_click,
        }
    }

    pub fn update(&mut self, input_state: &input::State, player: &mut Player<T>) {
        let mouse = input_state.mouse_coords();
        self.is_hovering = self.body.contains(&glm::to_dvec2(mouse));
        if self.is_hovering && input_state.did_click_mouse(MouseButton::Left) {
            (self.on_click)(player);
        }
    }
}

impl<'f, F, T, R> Scene<R> for Button<F, T>
    where F: Font,
          T: Texture,
          R: FontTexturizer<'f, Font = F, Texture = T> + Renderer<Texture = T>
{
    fn show(&self, renderer: &mut R) -> moho_errors::Result<()> {
        let color = if self.is_hovering {
            ColorRGBA(255, 255, 0, 0)

        } else {
            ColorRGBA(255, 255, 255, 0)
        };
        let texture = renderer.texturize(&self.font, self.text, &color)?;
        let dst_rect = glm::to_ivec4(glm::dvec4(self.body.top_left.x,
                                                self.body.top_left.y,
                                                self.body.dims.x,
                                                self.body.dims.y));
        renderer.copy(&texture, Some(&dst_rect), None)
    }
}
