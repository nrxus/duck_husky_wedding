use duck_husky_wedding::game_data::SpriteData;

use glm;
use moho::animation::{Animation, AnimationData, AnimatorData, TileSheet};
use moho::errors as moho_errors;
use moho::input;
use moho::renderer::{Renderer, Scene, Show, Texture};
use moho::shape::Rectangle;
use sdl2::keyboard::Keycode;

use std::time::Duration;
use std::rc::Rc;

pub struct Player<T> {
    animation: Animation<T>,
    body: Rectangle,
    velocity: f64,
}

impl<T> Player<T> {
    pub fn new(data: SpriteData, texture: Rc<T>) -> Self
        where T: Texture
    {
        let sheet = TileSheet::new(data.tiles.into(), texture);
        let animator = AnimatorData::new(data.frames, Duration::from_millis(50));
        let animation_data = AnimationData::new(animator, sheet);
        let body = Rectangle {
            top_left: glm::dvec2(0., 300.),
            dims: glm::dvec2(data.out_size.x as f64, data.out_size.y as f64),
        };
        Player {
            animation: animation_data.start(),
            body: body,
            velocity: 0.,
        }
    }

    pub fn update(&mut self, delta: Duration, input: &input::State) {
        self.animation.animate(delta);
        let mut velocity = 0.;
        if input.is_key_down(Keycode::Left) {
            velocity -= 5.;
        }
        if input.is_key_down(Keycode::Right) {
            velocity += 5.;
        }
        self.velocity = velocity;
        self.body.top_left.x += self.velocity;
        let window = Rectangle {
            top_left: glm::dvec2(0., 0.),
            dims: glm::dvec2(1280., 720.),
        };
        self.body = Self::clamp(&self.body, &window);
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
        renderer
            .with_asset(&self.animation.tile())
            .at(&dst_rect)
            .copy()
    }
}
