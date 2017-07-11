use data;
use errors::*;

use glm;
use moho::animation::{self, Animation};
use moho::errors as moho_errors;
use moho::shape::Rectangle;
use moho::renderer::{options, Renderer, Scene, Texture, TextureLoader, TextureManager};

use std::time::Duration;

enum Action<T> {
    Moving(Animation<T>),
    Standing(Animation<T>),
}

pub struct Data<T> {
    body: Rectangle,
    moving: animation::Data<T>,
    standing: animation::Data<T>,
}

impl<T> Clone for Data<T> {
    fn clone(&self) -> Self {
        Data {
            body: self.body.clone(),
            moving: self.moving.clone(),
            standing: self.standing.clone(),
        }
    }
}

impl<T> Data<T> {
    pub fn load<'t, TL>(
        data: &data::Cat,
        tl: glm::UVec2,
        texture_manager: &mut TextureManager<'t, TL>,
    ) -> Result<Self>
    where
        T: Texture,
        TL: TextureLoader<'t, Texture = T>,
    {
        let body = {
            let top_left = glm::to_dvec2(tl);
            let dims = data.out_size.into();
            Rectangle { top_left, dims }
        };
        let standing = data.idle.load(texture_manager)?;
        let moving = data.walking.load(texture_manager)?;
        Ok(Data {
            body,
            standing,
            moving,
        })
    }
}

pub struct Cat<T> {
    pub body: Rectangle,
    action: Action<T>,
    moving: animation::Data<T>,
}

impl<T> Cat<T> {
    pub fn new(data: &Data<T>) -> Self {
        Cat {
            body: data.body.clone(),
            moving: data.moving.clone(),
            action: Action::Standing(data.standing.clone().start()),
        }
    }

    pub fn start_walk(&mut self) {
        self.action = Action::Moving(self.moving.clone().start());
    }

    pub fn update(&mut self, duration: Duration) {
        match self.action {
            Action::Moving(ref mut animation) => animation.animate(duration),
            Action::Standing(ref mut animation) => animation.animate(duration),
        };
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
        let tile = match self.action {
            Action::Moving(ref animation) => animation.tile(),
            Action::Standing(ref animation) => animation.tile(),
        };
        renderer.copy_asset(&tile, options::at(&dst_rect))
    }
}
