use duck_husky_wedding::body::Body;
use duck_husky_wedding::cat::Cat;
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
    pub delta_pos: glm::DVec2,
    pub dst_rect: glm::DVec4,
    body: Vec<data::Shape>,
    action: Action<T>,
    animation: animation::Data<T>,
    texture: Rc<T>,
    backwards: bool,
    invincible: Invincible,
}

enum Invincible {
    None,
    Show(Duration),
    Hide(Duration),
}

impl Invincible {
    fn active(&self) -> bool {
        if let Invincible::None = *self {
            false
        } else {
            true
        }
    }

    fn activate(&mut self) {
        if let Invincible::None = *self {
            *self = Invincible::Hide(Duration::from_secs(1));
        }
    }

    fn update(&mut self, delta: Duration) {
        match *self {
            Invincible::None => {}
            Invincible::Show(d) => match d.checked_sub(delta) {
                None => *self = Invincible::None,
                Some(d) => *self = Invincible::Hide(d),
            },
            Invincible::Hide(d) => match d.checked_sub(delta) {
                None => *self = Invincible::None,
                Some(d) => *self = Invincible::Show(d),
            },
        }
    }
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
            delta_pos: glm::dvec2(0., 0.),
            backwards: false,
            invincible: Invincible::None,
            animation,
            texture,
            dst_rect,
            body,
        }
    }

    pub fn start_invincibility(&mut self) {
        self.invincible.activate();
    }

    pub fn is_invincible(&self) -> bool {
        self.invincible.active()
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
                Action::Jumping(_, ref mut held) => if *held < 15 {
                    held.add_assign(1);
                    self.delta_pos.y -= 6. / (*held as f64);
                },
                _ => self.delta_pos.y -= 6.,
            }
        } else if let Action::Jumping(_, ref mut held) = self.action {
            held.add_assign(15);
        }

        if left ^ right {
            self.backwards = left;
            self.delta_pos.x = if left { -6. } else { 6. };
        } else {
            self.delta_pos.x = 0.;
        }
    }

    pub fn update(&mut self, mut force: glm::DVec2, delta: Duration) {
        self.invincible.update(delta);
        if force.y.abs() < 0.0000001 {
            force.y = 0.
        }

        let next_action = match self.action {
            Action::Moving(ref mut a) => {
                if self.delta_pos.y.abs() > 0.0000001 || force.y.abs() > 0.0000001 {
                    Some(Action::Jumping(self.texture.clone(), 0))
                } else if self.delta_pos.x == 0. {
                    Some(Action::Standing(self.texture.clone()))
                } else {
                    a.animate(delta);
                    None
                }
            }
            Action::Standing(_) => {
                if self.delta_pos.y.abs() > 0.0000001 || force.y.abs() > 0.0000001 {
                    Some(Action::Jumping(self.texture.clone(), 0))
                } else if self.delta_pos.x == 0. {
                    None
                } else {
                    let animation = self.animation.clone().start();
                    Some(Action::Moving(animation))
                }
            }
            Action::Jumping(_, _) => {
                if self.delta_pos.y.abs() > 0.0000001 || force.y.abs() > 0.0000001 {
                    None
                } else if self.delta_pos.x == 0. {
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

        self.delta_pos = self.delta_pos + force;
        self.dst_rect.x += self.delta_pos.x;
        self.dst_rect.y += self.delta_pos.y;
    }
}

impl<'t, R: Renderer<'t>> Scene<R> for Player<R::Texture> {
    fn show(&self, renderer: &mut R) -> moho_errors::Result<()> {
        if let Invincible::Hide(_) = self.invincible {
            Ok(())
        } else {
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
}
