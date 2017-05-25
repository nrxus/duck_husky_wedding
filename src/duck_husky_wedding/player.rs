use glm;
use moho::animation::{self, Animation};
use moho::errors as moho_errors;
use moho::input;
use moho::renderer::{options, Renderer, Scene, Show, TextureFlip};
use moho::shape::{Rectangle, Shape};
use sdl2::keyboard::Keycode;

use std::time::Duration;
use std::rc::Rc;

enum Action<T> {
    Moving(Animation<T>),
    Standing(Rc<T>),
}

pub struct Player<T> {
    pub body: Rectangle,
    pub velocity: glm::DVec2,
    action: Action<T>,
    animation: animation::Data<T>,
    texture: Rc<T>,
    backwards: bool,
}

impl<T> Player<T> {
    pub fn new(animation: animation::Data<T>, texture: Rc<T>, body: Rectangle) -> Self {
        Player {
            action: Action::Standing(texture.clone()),
            velocity: glm::dvec2(0., 0.),
            animation: animation,
            texture: texture,
            body: body,
            backwards: false,
        }
    }

    pub fn process(&mut self, input: &input::State) {
        let left = input.is_key_down(Keycode::Left);
        let right = input.is_key_down(Keycode::Right);
        let up = input.did_press_key(Keycode::Up);
        if up {
            self.velocity.y -= 12.;
        }

        if left ^ right {
            self.backwards = left;
            self.velocity.x = if left { -5. } else { 5. };
        } else {
            self.velocity.x = 0.;
        }
    }

    pub fn update(&mut self, force: glm::DVec2, delta: Duration) {
        self.velocity = self.velocity + force;
        if self.velocity.x != 0. {
            match self.action {
                Action::Moving(ref mut a) => {
                    a.animate(delta);
                }
                Action::Standing(_) => {
                    let animation = self.animation.clone().start();
                    self.action = Action::Moving(animation);
                }
            }
        } else if let Action::Moving(_) = self.action {
            self.action = Action::Standing(self.texture.clone());
        }

        let window = Rectangle {
            top_left: glm::dvec2(0., 0.),
            dims: glm::dvec2(1280., 720.),
        };
        self.body = Self::clamp(&self.body.nudge(self.velocity), &window);
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

impl<'t, R> Scene<R> for Player<R::Texture>
    where R: Renderer<'t> + Show
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
