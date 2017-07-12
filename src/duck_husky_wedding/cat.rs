use data;
use errors::*;

use glm;
use moho::animation::{self, Animation};
use moho::errors as moho_errors;
use moho::shape::Rectangle;
use moho::renderer::{options, Renderer, Scene, Texture, TextureLoader, TextureManager};

use std::time::Duration;

pub struct Data<T> {
    body: Rectangle,
    animation: animation::Data<T>,
}

impl<T> Clone for Data<T> {
    fn clone(&self) -> Self {
        Data {
            body: self.body.clone(),
            animation: self.animation.clone(),
        }
    }
}

impl<T> Data<T> {
    pub fn load<'t, TL>(
        bl: glm::IVec2,
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
        let animation = data.idle.load(texture_manager)?;
        Ok(Data { body, animation })
    }
}

pub struct Cat<T> {
    pub body: Rectangle,
    animation: Animation<T>,
}

impl<T> Cat<T> {
    pub fn new(data: &Data<T>) -> Self {
        Cat {
            body: data.body.clone(),
            animation: data.animation.clone().start(),
        }
    }

    pub fn update(&mut self, duration: Duration) {
        self.animation.animate(duration);
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
        renderer.copy_asset(&self.animation.tile(), options::at(&dst_rect))
    }
}
