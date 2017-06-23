use glm;
use moho::animation::{self, Animation};
use moho::errors as moho_errors;
use moho::input;
use moho::renderer::{options, Renderer, Scene, TextureFlip};
use moho::shape::{Rectangle, Shape};
use sdl2::keyboard::Keycode;

use std::ops::AddAssign;
use std::time::Duration;
use std::rc::Rc;

enum Action<T> {
    Moving(Animation<T>),
    Jumping(Rc<T>, u32),
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
        let up = input.is_key_down(Keycode::Up);

        if up {
            match self.action {
                Action::Jumping(_, ref mut held) => {
                    if *held < 15 {
                        held.add_assign(1);
                        self.velocity.y -= 5. / (*held as f64);
                    }
                }
                _ => self.velocity.y -= 5.,
            }
        } else {
            if let Action::Jumping(_, ref mut held) = self.action {
                held.add_assign(15);
            }
        }

        if left ^ right {
            self.backwards = left;
            self.velocity.x = if left { -5. } else { 5. };
        } else {
            self.velocity.x = 0.;
        }
    }

    pub fn update(&mut self, force: glm::DVec2, delta: Duration) {
        let next_action = match self.action {
            Action::Moving(ref mut a) => {
                if self.velocity.y.abs() > 0.0000001 || force.y.abs() > 0.0000001 {
                    Some(Action::Jumping(self.texture.clone(), 0))
                } else if self.velocity.x == 0. {
                    Some(Action::Standing(self.texture.clone()))
                } else {
                    a.animate(delta);
                    None
                }
            }
            Action::Standing(_) => {
                if self.velocity.y.abs() > 0.0000001 || force.y.abs() > 0.0000001 {
                    Some(Action::Jumping(self.texture.clone(), 0))
                } else if self.velocity.x == 0. {
                    None
                } else {
                    let animation = self.animation.clone().start();
                    Some(Action::Moving(animation))
                }
            }
            Action::Jumping(_, _) => {
                if self.velocity.y.abs() > 0.0000001 || force.y.abs() > 0.0000001 {
                    None
                } else if self.velocity.x == 0. {
                    Some(Action::Standing(self.texture.clone()))
                } else {
                    let animation = self.animation.clone().start();
                    Some(Action::Moving(animation))
                }
            }
        };

        if let Some(a) = next_action {
            self.action = a;
        }

        self.velocity = self.velocity + force;
        self.body = self.body.nudge(self.velocity);
    }
}

impl<'t, R: Renderer<'t>> Scene<R> for Player<R::Texture> {
    fn show(&self, renderer: &mut R) -> moho_errors::Result<()> {
        let dst_rect = glm::to_ivec4(glm::dvec4(
            self.body.top_left.x,
            self.body.top_left.y,
            self.body.dims.x,
            self.body.dims.y,
        ));
        let mut options = options::at(&dst_rect);
        if self.backwards {
            options = options.flip(TextureFlip::Horizontal);
        }
        match self.action {
            Action::Moving(ref a) => renderer.copy_asset(&a.tile(), options),
            Action::Standing(ref t) => renderer.copy(&*t, options),
            Action::Jumping(ref t, _) => renderer.copy(&*t, options),
        }
    }
}
