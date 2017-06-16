use errors::*;
use duck_husky_wedding::button::{self, Button};

use glm;
use moho::errors as moho_errors;
use moho::input;
use moho::renderer::{options, ColorRGBA, Font, FontDetails, FontLoader, FontManager,
                     FontTexturizer, Renderer, Scene, Texture};
use moho::shape::Rectangle;

use std::rc::Rc;

pub struct Menu<T> {
    title: Rc<T>,
    new_game: button::Static<T>,
    high_score: button::Static<T>,
}

pub struct Data<T> {
    title: Rc<T>,
    new_game: button::Static<T>,
    high_score: button::Static<T>,
}

impl<T> Data<T> {
    pub fn load<'f, 't, FT, FL>(
        font_manager: &mut FontManager<'f, FL>,
        texturizer: &'t FT,
    ) -> Result<Self>
    where
        T: Texture,
        FL: FontLoader<'f>,
        FT: FontTexturizer<'f, 't, Font = FL::Font, Texture = T>,
    {
        let font_details = FontDetails {
            path: "media/fonts/kenpixel_mini.ttf",
            size: 64,
        };
        let font = font_manager.load(&font_details)?;
        let dims = font.measure("New Game")?;
        let top_left = glm::ivec2(640 - dims.x as i32 / 2, 200);
        let body = Rectangle {
            top_left: glm::to_dvec2(top_left),
            dims: glm::to_dvec2(dims),
        };
        let new_game = button::Static::text_at("New Game", texturizer, &*font, body)?;
        let title_color = ColorRGBA(255, 255, 0, 255);
        let title = Rc::new(texturizer.texturize(
            &*font,
            "Husky Loves Ducky",
            &title_color,
        )?);

        let dims = font.measure("High Score")?;
        let top_left = glm::ivec2(640 - dims.x as i32 / 2, 400);
        let body = Rectangle {
            top_left: glm::to_dvec2(top_left),
            dims: glm::to_dvec2(dims),
        };
        let high_score = button::Static::text_at("High Score", texturizer, &*font, body)?;
        Ok(Data {
            title,
            new_game,
            high_score,
        })
    }

    pub fn activate(&self) -> Menu<T> {
        let title = self.title.clone();
        let new_game = self.new_game.clone();
        let high_score = self.high_score.clone();

        Menu {
            title,
            new_game,
            high_score,
        }
    }
}

impl<T> Menu<T> {
    pub fn update(&mut self, input: &input::State) -> Option<super::Kind> {
        if self.new_game.update(input) {
            Some(super::Kind::PlayerSelect)
        } else if self.high_score.update(input) {
            Some(super::Kind::HighScore)
        } else {
            None
        }
    }
}

impl<'t, T, R> Scene<R> for Menu<T>
where
    T: Texture,
    R: Renderer<'t, Texture = T>,
{
    fn show(&self, renderer: &mut R) -> moho_errors::Result<()> {
        let title_dims = glm::to_ivec2(self.title.dims());
        let title_rectangle = glm::ivec4(640 - title_dims.x / 2, 0, title_dims.x, title_dims.y);
        renderer.show(&self.new_game)?;
        renderer.show(&self.high_score)?;
        renderer.copy(&self.title, options::at(&title_rectangle))
    }
}
