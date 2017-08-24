use data;
use errors::*;
use duck_husky_wedding::body::Body;

use glm;
use moho::animation::{self, Animation};
use moho::errors as moho_errors;
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
    dst: glm::DVec4,
    body: Vec<data::Shape>,
    animation: animation::Data<T>,
    kind: Kind,
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
        let dst = {
            let dims: glm::DVec2 = data.out_size.into();
            glm::dvec4(bl.x as f64, bl.y as f64 - dims.y, dims.x, dims.y)
        };
        let animation = match kind {
            Kind::Idle => &data.idle,
            Kind::Moving { .. } => &data.walking,
        };
        let animation = animation.load(texture_manager)?;
        let body = data.body.clone();
        Ok(Data {
            dst,
            body,
            animation,
            kind,
        })
    }
}

pub struct Cat<T> {
    pub dst: glm::DVec4,
    body: Vec<data::Shape>,
    animation: Animation<T>,
    kind: Kind,
}

impl<T> Cat<T> {
    pub fn new(data: &Data<T>) -> Self {
        Cat {
            body: data.body.clone(),
            dst: data.dst,
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
            self.dst.x += updated as f64 - current as f64;
            current = updated;

            self.kind = Kind::Moving {
                total,
                current,
                left,
            };
        }
    }

    pub fn body(&self) -> Body {
        let backwards = match self.kind {
            Kind::Idle | Kind::Moving { left: true, .. } => true,
            _ => false,
        };
        Body::new(&self.dst, &self.body, backwards)
    }
}

impl<'t, R: Renderer<'t>> Scene<R> for Cat<R::Texture> {
    fn show(&self, renderer: &mut R) -> moho_errors::Result<()> {
        let dst_rect = glm::to_ivec4(self.dst);
        let mut options = options::at(dst_rect);

        match self.kind {
            Kind::Idle | Kind::Moving { left: true, .. } => {
                options = options.flip(TextureFlip::Horizontal)
            }
            _ => {}
        }

        renderer.copy_asset(&self.animation.tile(), options)?;
        renderer.show(&self.body())
    }
}
