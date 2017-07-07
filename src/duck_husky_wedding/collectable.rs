use data;
use errors::*;


use moho::animation::{self, Animation};
use moho::errors as moho_errors;
use moho::renderer::{options, Renderer, Scene, Texture, TextureLoader, TextureManager};
use moho::shape::Rectangle;

use glm;

use std::time::Duration;

#[derive(Debug)]
pub struct Collectable<T> {
    animation: Animation<T>,
    body: Rectangle,
}

#[derive(Debug)]
pub struct Data<T> {
    animation: animation::Data<T>,
    body: Rectangle,
}

impl<T: Texture> Data<T> {
    pub fn load<'t, TL>(
        bl: glm::IVec2,
        data: &data::Collectable,
        texture_manager: &mut TextureManager<'t, TL>,
    ) -> Result<Self>
    where
        TL: TextureLoader<'t, Texture = T>,
    {
        let animation = data.animation.load(texture_manager)?;
        let dims: glm::DVec2 = data.out_size.into();
        let top_left = glm::dvec2(bl.x as f64, 720. - bl.y as f64 - dims.y);
        let body = Rectangle { top_left, dims };
        Ok(Data { animation, body })
    }
}

impl<T> Collectable<T> {
    pub fn new(data: &Data<T>) -> Self {
        Collectable {
            animation: data.animation.clone().start(),
            body: data.body.clone(),
        }
    }

    pub fn animate(&mut self, duration: Duration) {
        self.animation.animate(duration);
    }
}

impl<'t, R: Renderer<'t>> Scene<R> for Collectable<R::Texture> {
    fn show(&self, renderer: &mut R) -> moho_errors::Result<()> {
        let dst = glm::ivec4(
            self.body.top_left.x as i32,
            self.body.top_left.y as i32,
            self.body.dims.x as i32,
            self.body.dims.y as i32,
        );

        renderer.copy_asset(&self.animation.tile(), options::at(&dst))
    }
}
