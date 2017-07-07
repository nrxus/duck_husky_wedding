use data;
use duck_husky_wedding::button::{self, Button};
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
    collect_text: Rc<T>,
    avoid_text: Rc<T>,
    husky: button::Animated<T>,
    duck: button::Animated<T>,
}

pub struct Data<T> {
    title: Rc<T>,
    collect_text: Rc<T>,
    avoid_text: Rc<T>,
    husky: button::Animated<T>,
    duck: button::Animated<T>,
}

impl<T> Data<T> {
    pub fn load<'f, 't, FT, FL, TL>(
        font_manager: &mut FontManager<'f, FL>,
        texturizer: &'t FT,
        texture_manager: &mut TextureManager<'t, TL>,
        data: &data::Game,
    ) -> Result<Self>
    where
        T: Texture,
        TL: TextureLoader<'t, Texture = T>,
        FL: FontLoader<'f>,
        FT: FontTexturizer<'t, FL::Font, Texture = T>,
    {
        let font_details = FontDetails {
            path: "media/fonts/kenpixel_mini.ttf",
            size: 64,
        };
        let font = font_manager.load(&font_details)?;
        let title_color = ColorRGBA(255, 255, 0, 255);
        let title = Rc::new(texturizer.texturize(&*font, "Select Player", &title_color)?);
        let button_distance = 50.;
        
        let husky = {
            let data = &data.husky;
            let idle = data.idle_texture.load(texture_manager)?;
            let animation = data.animation.load(texture_manager)?;
            let dims: glm::DVec2 = data.out_size.into();
            let dims = dims * 1.5;
            let top_left = glm::dvec2(640. - dims.x - button_distance, 250. - dims.y);
            let body = Rectangle { top_left, dims };
            button::Animated::new(idle, animation, body)
        };
        let duck = {
            let data = &data.duck;
            let idle = data.idle_texture.load(texture_manager)?;
            let animation = data.animation.load(texture_manager)?;
            let dims: glm::DVec2 = data.out_size.into();
            let dims = dims * 1.5;
            let top_left = glm::dvec2(640. + button_distance, 250. - dims.y);
            let body = Rectangle { top_left, dims };
            button::Animated::new(idle, animation, body)
        };
        let collect_text = Rc::new(texturizer.texturize(&*font, "Collect", &title_color)?);
        let avoid_text = Rc::new(texturizer.texturize(&*font, "Avoid", &title_color)?);

        Ok(Data {
            title,
            husky,
            duck,
            collect_text,
            avoid_text,
        })
    }

    pub fn activate(&self) -> PlayerSelect<T> {
        PlayerSelect {
            title: self.title.clone(),
            duck: self.duck.clone(),
            husky: self.husky.clone(),
            collect_text: self.collect_text.clone(),
            avoid_text: self.avoid_text.clone(),
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

impl<'t, R: Renderer<'t>> Scene<R> for PlayerSelect<R::Texture>
where
    R::Texture: Texture,
{
    fn show(&self, renderer: &mut R) -> moho_errors::Result<()> {
        let title_rectangle = {
            let dims = glm::to_ivec2(self.title.dims());
            glm::ivec4(640 - dims.x / 2, 0, dims.x, dims.y)
        };
        let collect_rectangle = {
            let dims = glm::to_ivec2(self.collect_text.dims());
            glm::ivec4(320 - dims.x / 2, 350, dims.x, dims.y)
        };
        let avoid_rectangle = {
            let dims = glm::to_ivec2(self.avoid_text.dims());
            glm::ivec4(960 - dims.x / 2, 350, dims.x, dims.y)
        };

        renderer.show(&self.husky)?;
        renderer.show(&self.duck)?;
        renderer.copy(&self.title, options::at(&title_rectangle))?;
        renderer.copy(&self.collect_text, options::at(&collect_rectangle))?;
        renderer.copy(&self.avoid_text, options::at(&avoid_rectangle))
    }
}
