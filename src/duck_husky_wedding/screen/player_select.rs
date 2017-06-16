use duck_husky_wedding::button::{self, Button};
use duck_husky_wedding::game_data::GameData;
use errors::*;

use glm;
use moho::errors as moho_errors;
use moho::input;
use moho::renderer::{options, ColorRGBA, FontDetails, FontManager, FontLoader, FontTexturizer,
                     Renderer, Scene, Texture, TextureManager, TextureLoader};
use moho::shape::Rectangle;

use std::rc::Rc;
use std::time::Duration;

pub struct PlayerSelect<T> {
    title: Rc<T>,
    husky: button::Animated<T>,
    duck: button::Animated<T>,
}

pub struct Data<T> {
    title: Rc<T>,
    husky: button::Animated<T>,
    duck: button::Animated<T>,
}

impl<T> Data<T> {
    pub fn load<'f, 't, FT, FL, TL>(font_manager: &mut FontManager<'f, FL>,
                                    texturizer: &'t FT,
                                    texture_manager: &mut TextureManager<'t, TL>,
                                    data: &GameData)
                                    -> Result<Self>
        where T: Texture,
              TL: TextureLoader<'t, Texture = T>,
              FL: FontLoader<'f>,
              FT: FontTexturizer<'f, 't, Font = FL::Font, Texture = T>
    {
        let font_details = FontDetails {
            path: "media/fonts/kenpixel_mini.ttf",
            size: 64,
        };
        let font = font_manager.load(&font_details)?;
        let title_color = ColorRGBA(255, 255, 0, 255);
        let title = Rc::new(texturizer.texturize(&*font, "Select Player", &title_color)?);

        let husky = &data.husky;
        let file_name: &str = &format!("media/sprites/{}", husky.texture.file_name);
        let idle = texture_manager.load(file_name)?;
        let body = Rectangle {
            top_left: glm::dvec2(300., 300.),
            dims: glm::dvec2(husky.out_size.x as f64 * 2., husky.out_size.y as f64 * 2.),
        };
        let animation = husky.animation.load(texture_manager)?;
        let husky = button::Animated::new(idle, animation, body);

        let duck = &data.duck;
        let file_name: &str = &format!("media/sprites/{}", duck.texture.file_name);
        let idle = texture_manager.load(file_name)?;
        let body = Rectangle {
            top_left: glm::dvec2(750., 350.),
            dims: glm::dvec2(duck.out_size.x as f64 * 2., duck.out_size.y as f64 * 2.),
        };
        let animation = duck.animation.load(texture_manager)?;
        let duck = button::Animated::new(idle, animation, body);

        Ok(Data { title, husky, duck })
    }

    pub fn activate(&self) -> PlayerSelect<T> {
        PlayerSelect {
            title: self.title.clone(),
            duck: self.duck.clone(),
            husky: self.husky.clone(),
        }
    }
}

impl<T> PlayerSelect<T> {
    pub fn update(&mut self, delta: Duration, input: &input::State) -> Option<super::Kind> {
        if self.husky.update(input) {
            Some(super::Kind::GamePlay(super::PlayerKind::Husky))
        } else if self.duck.update(input) {
            Some(super::Kind::GamePlay(super::PlayerKind::Duck))
        } else {
            self.husky.animate(delta);
            self.duck.animate(delta);
            None
        }
    }
}

impl<'t, T, R> Scene<R> for PlayerSelect<T>
    where T: Texture,
          R: Renderer<'t, Texture = T>
{
    fn show(&self, renderer: &mut R) -> moho_errors::Result<()> {
        let title_dims = glm::to_ivec2(self.title.dims());
        let title_rectangle = glm::ivec4(640 - title_dims.x / 2, 0, title_dims.x, title_dims.y);
        renderer.show(&self.husky)?;
        renderer.show(&self.duck)?;
        renderer.copy(&self.title, options::at(&title_rectangle))
    }
}
