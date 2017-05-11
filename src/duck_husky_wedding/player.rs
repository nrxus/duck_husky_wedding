use glm;
use moho::animation::{self, Animation};
use moho::errors as moho_errors;
use moho::input;
use moho::renderer::{options, Renderer, Scene, Show, Texture, TextureFlip};
use moho::shape::Rectangle;
use sdl2::keyboard::Keycode;

use std::time::Duration;
use std::rc::Rc;

enum Action<T> {
    Moving(Animation<T>),
    Standing(Rc<T>),
}

pub struct Player<T> {
    action: Action<T>,
    animation: animation::Data<T>,
    texture: Rc<T>,
    body: Rectangle,
    backwards: bool,
}

impl<T> Player<T> {
    pub fn new(animation: animation::Data<T>, texture: Rc<T>, body: Rectangle) -> Self
        where T: Texture
    {
        Player {
            action: Action::Standing(texture.clone()),
            animation: animation,
            texture: texture,
            body: body,
            backwards: false,
        }
    }

    pub fn update(&mut self, delta: Duration, input: &input::State) {
        let left = input.is_key_down(Keycode::Left);
        let right = input.is_key_down(Keycode::Right);

        if left ^ right {
            match self.action {
                Action::Moving(ref mut a) => {
                    a.animate(delta);
                }
                Action::Standing(_) => {
                    let animation = self.animation.clone().start();
                    self.action = Action::Moving(animation);
                }
            }
            let window = Rectangle {
                top_left: glm::dvec2(0., 0.),
                dims: glm::dvec2(1280., 720.),
            };
            self.backwards = left;
            let velocity = if left { -5. } else { 5. };
            self.body.top_left.x += velocity;
            self.body = Self::clamp(&self.body, &window);
        } else if let Action::Moving(_) = self.action {
            self.action = Action::Standing(self.texture.clone());
        }
    }

    fn clamp(shape: &Rectangle, window: &Rectangle) -> Rectangle {
        let tl = shape.top_left;

        let left = tl.x
            .max(window.top_left.x)
            .min(window.top_left.x + window.dims.x - shape.dims.x);
        let top = tl.y
            .max(window.top_left.y)
            .min(window.top_left.y + window.dims.y - shape.dims.y);

        Rectangle {
            top_left: glm::dvec2(left, top),
            dims: shape.dims,
        }
    }
}

impl<'t, T, R> Scene<R> for Player<T>
    where R: Renderer<'t, Texture = T> + Show
{
    fn show(&self, renderer: &mut R) -> moho_errors::Result<()> {
        let dst_rect = glm::to_ivec4(glm::dvec4(self.body.top_left.x,
                                                self.body.top_left.y,
                                                self.body.dims.x,
                                                self.body.dims.y));
        let mut options = options::at(&dst_rect);
        if self.backwards {
            options = options.flip(TextureFlip::Horizontal);
        }
        match self.action {
            Action::Moving(ref a) => renderer.copy_asset(&a.tile(), options),
            Action::Standing(ref t) => renderer.copy(&*t, options),
        }
    }
}
