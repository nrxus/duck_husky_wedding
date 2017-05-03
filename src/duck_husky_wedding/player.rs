use duck_husky_wedding::game_data::SpriteData;

use glm;
use moho::errors as moho_errors;
use moho::renderer::{Drawable, Renderer, Scene, Show, Texture};
use moho::animation::{Animation, AnimationData, AnimatorData, TileSheet};

use std::time::Duration;
use std::rc::Rc;

pub struct Player<T> {
    animation: Animation<T>,
    dimensions: glm::UVec2,
    position: glm::IVec2,
    velocity: i32,
}

impl<T> Player<T> {
    pub fn new(data: SpriteData, texture: Rc<T>) -> Self
        where T: Texture
    {
        let sheet = TileSheet::new(data.tiles.into(), texture);
        let animator = AnimatorData::new(data.frames, Duration::from_millis(50));
        let animation_data = AnimationData::new(animator, sheet);
        Player {
            animation: animation_data.start(),
            dimensions: data.out_size.into(),
            position: glm::ivec2(0, 300),
            velocity: 4,
        }
    }

    pub fn animate(&mut self, delta: Duration) {
        self.animation.animate(delta);
    }

    pub fn update(&mut self) {
        self.position.x = (self.position.x + self.velocity + 1280) % 1280;
    }

    pub fn flip(&mut self) {
        self.velocity *= -1;
    }
}

impl<'t, T, R> Scene<R> for Player<T>
    where R: Renderer<'t, Texture = T> + Show
{
    fn show(&self, renderer: &mut R) -> moho_errors::Result<()> {
        let dst_rect = glm::ivec4(self.position.x,
                                  self.position.y,
                                  self.dimensions.x as i32,
                                  self.dimensions.y as i32);
        self.animation.draw(&dst_rect, renderer)
    }
}
