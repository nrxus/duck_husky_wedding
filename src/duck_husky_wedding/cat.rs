use data;
use errors::*;

use glm;
use moho::animation::{self, Animation};
use moho::errors as moho_errors;
use moho::shape::Rectangle;
use moho::renderer::{options, Renderer, Scene, Texture, TextureFlip, TextureLoader, TextureManager};

use std::time::Duration;

#[derive(Clone, Copy)]
pub enum Kind {
    Idle,
    Moving {
        total: u32,
        current: u32,
        left: bool,
    },
}

pub struct Data<T> {
    body: Rectangle,
    animation: animation::Data<T>,
    kind: Kind,
}

impl<T> Clone for Data<T> {
    fn clone(&self) -> Self {
        Data {
            body: self.body.clone(),
            animation: self.animation.clone(),
            kind: self.kind,
        }
    }
}

impl<T> Data<T> {
    pub fn load<'t, TL>(
        bl: glm::IVec2,
        kind: Kind,
        data: &data::Cat,
        texture_manager: &mut TextureManager<'t, TL>,
    ) -> Result<Self>
    where
        T: Texture,
        TL: TextureLoader<'t, Texture = T>,
    {
        let body = {
            let dims: glm::DVec2 = data.out_size.into();
            let top_left = glm::dvec2(bl.x as f64, bl.y as f64 - dims.y);
            Rectangle { top_left, dims }
        };
        let animation = match kind {
            Kind::Idle => &data.idle,
            Kind::Moving { .. } => &data.walking,
        };
        let animation = animation.load(texture_manager)?;
        Ok(Data {
            body,
            animation,
            kind,
        })
    }
}

pub struct Cat<T> {
    pub body: Rectangle,
    animation: Animation<T>,
    kind: Kind,
}

impl<T> Cat<T> {
    pub fn new(data: &Data<T>) -> Self {
        Cat {
            body: data.body.clone(),
            animation: data.animation.clone().start(),
            kind: data.kind,
        }
    }

    pub fn update(&mut self, duration: Duration) {
        self.animation.animate(duration);
        if let Kind::Moving {
            total,
            mut current,
            mut left,
        } = self.kind
        {
            let step = 2;
            let updated = if left {
                if current < step {
                    left = false;
                    step - current
                } else {
                    current - step
                }
            } else if current + step > total {
                left = true;
                2 * total - current - step
            } else {
                current + step
            };
            self.body.top_left.x += updated as f64 - current as f64;
            current = updated;

            self.kind = Kind::Moving {
                total,
                current,
                left,
            };
        }
    }
}

impl<'t, R: Renderer<'t>> Scene<R> for Cat<R::Texture> {
    fn show(&self, renderer: &mut R) -> moho_errors::Result<()> {
        let dst_rect = glm::ivec4(
            self.body.top_left.x as i32,
            self.body.top_left.y as i32,
            self.body.dims.x as i32,
            self.body.dims.y as i32,
        );
        let mut options = options::at(&dst_rect);

        match self.kind {
            Kind::Idle | Kind::Moving { left: true, .. } => {
                options = options.flip(TextureFlip::Horizontal)
            }
            _ => {}
        }

        renderer.copy_asset(&self.animation.tile(), options)
    }
}
