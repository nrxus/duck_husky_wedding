use duck_husky_wedding::body::Body;
use data;
use errors::*;

use glm;
use moho::animation::{self, Animation};
use moho::errors as moho_errors;
use moho::input;
use moho::renderer::{options, Renderer, Scene, Texture, TextureFlip, TextureLoader, TextureManager};
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
    pub velocity: glm::DVec2,
    pub dst_rect: glm::DVec4,
    body: Vec<data::Shape>,
    action: Action<T>,
    animation: animation::Data<T>,
    texture: Rc<T>,
    backwards: bool,
}

impl<T> Player<T> {
    pub fn load<'t, TL>(
        data: &data::Player,
        tl: glm::UVec2,
        texture_manager: &mut TextureManager<'t, TL>,
    ) -> Result<Self>
    where
        T: Texture,
        TL: TextureLoader<'t, Texture = T>,
    {
        let dst_rect = {
            let dims = data.out_size;
            glm::dvec4(tl.x as f64, tl.y as f64, dims.x as f64, dims.y as f64)
        };
        let animation = data.animation.load(texture_manager)?;
        let texture = data.idle_texture.load(texture_manager)?;
        Ok(Player::new(animation, texture, dst_rect, data.body.clone()))
    }

    pub fn new(
        animation: animation::Data<T>,
        texture: Rc<T>,
        dst_rect: glm::DVec4,
        body: Vec<data::Shape>,
    ) -> Self {
        Player {
            action: Action::Standing(texture.clone()),
            velocity: glm::dvec2(0., 0.),
            backwards: false,
            animation,
            texture,
            dst_rect,
            body,
        }
    }

    pub fn body(&self) -> Body {
        Body::new(&self.dst_rect, &self.body, self.backwards)
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
                        self.velocity.y -= 6. / (*held as f64);
                    }
                }
                _ => self.velocity.y -= 6.,
            }
        } else if let Action::Jumping(_, ref mut held) = self.action {
            held.add_assign(15);
        }

        if left ^ right {
            self.backwards = left;
            self.velocity.x = if left { -6. } else { 6. };
        } else {
            self.velocity.x = 0.;
        }
    }

    pub fn update(&mut self, mut force: glm::DVec2, delta: Duration) {
        if force.y.abs() < 0.0000001 {
            force.y = 0.
        }

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
        self.dst_rect.x += self.velocity.x;
        self.dst_rect.y += self.velocity.y;
    }
}

impl<'t, R: Renderer<'t>> Scene<R> for Player<R::Texture> {
    fn show(&self, renderer: &mut R) -> moho_errors::Result<()> {
        let dst = glm::to_ivec4(self.dst_rect);
        let mut options = options::at(&dst);
        if self.backwards {
            options = options.flip(TextureFlip::Horizontal);
        }
        match self.action {
            Action::Moving(ref a) => renderer.copy_asset(&a.tile(), options),
            Action::Standing(ref t) | Action::Jumping(ref t, _) => renderer.copy(&*t, options),
        }?;
        renderer.show(&self.body())
    }
}
