use duck_husky_wedding::game_data::SpriteData;

use glm;
use moho::animation::{self, animator, Animation, TileSheet};
use moho::errors as moho_errors;
use moho::input;
use moho::renderer::{Renderer, Scene, Show, Texture, TextureFlip};
use moho::shape::Rectangle;
use sdl2::keyboard::Keycode;

use std::time::Duration;
use std::rc::Rc;

pub struct Player<T> {
    animation: Animation<T>,
    body: Rectangle,
    backwards: bool,
}

impl<T> Player<T> {
    pub fn new(data: SpriteData, texture: Rc<T>) -> Self
        where T: Texture
    {
        let sheet = TileSheet::new(data.tiles.into(), texture);
        let animator = animator::Data::new(data.frames, Duration::from_millis(50));
        let animation_data = animation::Data::new(animator, sheet);
        let body = Rectangle {
            top_left: glm::dvec2(0., 300.),
            dims: glm::dvec2(data.out_size.x as f64, data.out_size.y as f64),
        };
        Player {
            animation: animation_data.start(),
            body: body,
            backwards: false,
        }
    }

    pub fn update(&mut self, delta: Duration, input: &input::State) {
        let left = input.is_key_down(Keycode::Left);
        let right = input.is_key_down(Keycode::Right);
        if left ^ right {
            self.animation.animate(delta);
            let window = Rectangle {
                top_left: glm::dvec2(0., 0.),
                dims: glm::dvec2(1280., 720.),
            };
            self.backwards = left;
            let velocity = if left { -5. } else { 5. };
            self.body.top_left.x += velocity;
            self.body = Self::clamp(&self.body, &window);
        } else {
            self.animation.animator.restart();
        }
    }

    fn clamp(shape: &Rectangle, window: &Rectangle) -> Rectangle {
        let tl = shape.top_left;

        let left = tl.x
            .max(window.top_left.x)
            .min(window.top_left.x + window.dims.x - shape.dims.x);
        let top = tl.y
            .max(window.top_left.y)
            .min(window.top_left.y + window.dims.y - shape.dims.y);

        Rectangle {
            top_left: glm::dvec2(left, top),
            dims: shape.dims,
        }
    }
}

impl<'t, T, R> Scene<R> for Player<T>
    where R: Renderer<'t, Texture = T> + Show
{
    fn show(&self, renderer: &mut R) -> moho_errors::Result<()> {
        let dst_rect = glm::to_ivec4(glm::dvec4(self.body.top_left.x,
                                                self.body.top_left.y,
                                                self.body.dims.x,
                                                self.body.dims.y));
        let tile = self.animation.tile();
        let mut partial = renderer.with_asset(&tile).at(&dst_rect);
        if self.backwards {
            partial = partial.flip(TextureFlip::Horizontal)
        }
        partial.copy()
    }
}
