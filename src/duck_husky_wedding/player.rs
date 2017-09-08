use duck_husky_wedding::body::Body;
use duck_husky_wedding::flicker::Flicker;
use data;
use errors::*;

use glm;
use moho::animation::{self, Animation};
use moho::errors as moho_errors;
use moho::input;
use moho::renderer::{options, Renderer, Scene, Texture, TextureLoader, TextureManager};
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
    pub invincibility: Option<Invincibility>,
    body: Vec<data::Shape>,
    legs: Vec<data::Shape>,
    action: Action<T>,
    animation: animation::Data<T>,
    texture: Rc<T>,
    backwards: bool,
}

#[derive(Debug, Clone, Copy)]
pub struct Invincibility {
    duration: Duration,
    flicker: Flicker,
}

impl Invincibility {
    fn new() -> Self {
        let flicker = Flicker::new(Duration::from_millis(100));
        let duration = Duration::from_secs(1);
        Invincibility { flicker, duration }
    }

    fn update(mut self, delta: Duration) -> Option<Self> {
        self.duration.checked_sub(delta).map(|d| {
            self.duration = d;
            self.flicker.update(delta);
            self
        })
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
            let dims = glm::DVec2::from(data.out_size);
            let tl = glm::to_dvec2(tl);
            glm::dvec4(tl.x, tl.y, dims.x, dims.y)
        };
        let animation = data.animation.load(texture_manager)?;
        let texture = data.idle_texture.load(texture_manager)?;
        Ok(Player::new(
            animation,
            texture,
            dst_rect,
            data.body.clone(),
            data.legs.clone(),
        ))
    }

    pub fn new(
        animation: animation::Data<T>,
        texture: Rc<T>,
        dst_rect: glm::DVec4,
        body: Vec<data::Shape>,
        legs: Vec<data::Shape>,
    ) -> Self {
        Player {
            action: Action::Standing(texture.clone()),
            delta_pos: glm::dvec2(0., 0.),
            backwards: false,
            invincibility: None,
            animation,
            texture,
            dst_rect,
            body,
            legs,
        }
    }

    pub fn body(&self) -> Body {
        Body::new(&self.dst_rect, &self.body, self.backwards)
    }

    pub fn legs(&self) -> Body {
        Body::new(&self.dst_rect, &self.legs, self.backwards)
    }

    pub fn process(&mut self, input: &input::State) {
        let left = input.is_key_down(Keycode::Left);
        let right = input.is_key_down(Keycode::Right);
        let up = input.is_key_down(Keycode::Up);

        if up {
            match self.action {
                Action::Jumping(_, ref mut held) => if *held < 10 {
                    held.add_assign(1);
                    self.delta_pos.y -= 3.9 / f64::from(*held)
                },
                _ => {
                    self.delta_pos.y = -3.9;
                }
            }
        } else if let Action::Jumping(_, ref mut held) = self.action {
            held.add_assign(15);
        }

        if left ^ right {
            self.backwards = left;
            self.delta_pos.x = if left { -4.5 } else { 4.5 };
        } else {
            self.delta_pos.x = 0.;
        }

        self.delta_pos.y += 0.75;
        self.delta_pos.y = self.delta_pos.y.min(25.);
    }

    pub fn update(&mut self, (force, on_floor): (glm::DVec2, bool), delta: Duration) {
        if let Some(i) = self.invincibility {
            self.invincibility = i.update(delta);
        }

        let same_y = self.delta_pos.y.signum() == force.y.signum();

        let next_action = match self.action {
            Action::Moving(ref mut a) => if !on_floor {
                Some(Action::Jumping(self.texture.clone(), 0))
            } else if self.delta_pos.x == 0. {
                Some(Action::Standing(self.texture.clone()))
            } else {
                a.animate(delta);
                None
            },
            Action::Standing(_) => if !on_floor {
                Some(Action::Jumping(self.texture.clone(), 0))
            } else if self.delta_pos.x == 0. {
                None
            } else {
                let animation = self.animation.clone().start();
                Some(Action::Moving(animation))
            },
            Action::Jumping(_, ref mut held) => if !on_floor || (on_floor && self.delta_pos.y < 0.)
            {
                if self.delta_pos.y.abs() > 0. && force.y.abs() > 0. && !same_y {
                    held.add_assign(100);
                }
                None
            } else if self.delta_pos.x == 0. {
                Some(Action::Standing(self.texture.clone()))
            } else {
                let animation = self.animation.clone().start();
                Some(Action::Moving(animation))
            },
        };

        if let Some(a) = next_action {
            self.action = a;
        }

        self.dst_rect.x += self.delta_pos.x + force.x;
        self.dst_rect.y += self.delta_pos.y + force.y;

        if self.delta_pos.y.abs() > 0. && force.y.abs() > 0. && !same_y {
            self.delta_pos.y = 0.
        }

        let max_y = 720. - 17. - self.dst_rect.w;
        self.dst_rect.y = self.dst_rect.y.min(max_y);
    }

    pub fn invincible(&mut self) {
        self.invincibility = Some(Invincibility::new());
    }
}

impl<'t, R: Renderer<'t>> Scene<R> for Player<R::Texture> {
    fn show(&self, renderer: &mut R) -> moho_errors::Result<()> {
        if self.invincibility
            .map(|i| i.flicker.is_shown())
            .unwrap_or(true)
        {
            let dst = glm::to_ivec4(self.dst_rect);
            let mut options = options::at(dst);
            if self.backwards {
                options = options.flip(options::Flip::Horizontal);
            }
            match self.action {
                Action::Moving(ref a) => renderer.copy_asset(&a.tile(), options),
                Action::Standing(ref t) | Action::Jumping(ref t, _) => renderer.copy(&*t, options),
            }?;
            renderer.show(&self.body())?;
            renderer.show(&self.legs())
        } else {
            Ok(())
        }
    }
}
