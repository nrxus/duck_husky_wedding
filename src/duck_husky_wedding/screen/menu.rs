use errors::*;
use duck_husky_wedding::button::Button;

use glm;
use moho::errors as moho_errors;
use moho::input;
use moho::renderer::{ColorRGBA, Font, FontDetails, FontLoader, FontManager, FontTexturizer,
                     Renderer, Scene, Show, Texture};
use moho::shape::Rectangle;

pub struct Menu<T> {
    title: T,
    new_game: Button<T>,
}

impl<T> Menu<T> {
    pub fn load<'f, 't, FT, FL>(font_manager: &mut FontManager<'f, FL>,
                                texturizer: &'t FT)
                                -> Result<Self>
        where T: Texture,
              FL: FontLoader<'f>,
              FT: FontTexturizer<'f, 't, Font = FL::Font, Texture = T>
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
        let new_game = Button::text_at("New Game", texturizer, &*font, body)?;
        let title_color = ColorRGBA(255, 255, 0, 255);
        let title = texturizer
            .texturize(&*font, "Husky Loves Ducky", &title_color)?;
        Ok(Self::new(title, new_game))
    }

    pub fn new(title: T, new_game: Button<T>) -> Self {
        Menu {
            title: title,
            new_game: new_game,
        }
    }

    pub fn update(&mut self, input: &input::State) -> super::Kind {
        if self.new_game.update(input) {
            super::Kind::GamePlay
        } else {
            super::Kind::Menu
        }
    }
}

impl<'t, T, R> Scene<R> for Menu<T>
    where T: Texture,
          R: Renderer<'t, Texture = T> + Show
{
    fn show(&self, renderer: &mut R) -> moho_errors::Result<()> {
        let title_dims = glm::to_ivec2(self.title.dims());
        let title_rectangle = glm::ivec4(640 - title_dims.x / 2, 0, title_dims.x, title_dims.y);
        renderer.show(&self.new_game)?;
        renderer.copy(&self.title, Some(&title_rectangle), None)
    }
}
