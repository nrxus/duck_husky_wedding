use errors::*;
use duck_husky_wedding::button::Button;

use glm;
use moho::errors as moho_errors;
use moho::input;
use moho::renderer::{ColorRGBA, FontDetails, FontLoader, FontManager, FontTexturizer, Renderer,
                     Scene, Show, Texture};

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
        let title_color = ColorRGBA(255, 255, 0, 255);
        let title = texturizer
            .texturize(&*font, "Husky <3 Ducky", &title_color)?;
        let new_game = Button::from_text("New Game", texturizer, &*font, glm::ivec2(200, 200))?;
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
        let title_dims = self.title.dims();
        let title_rectangle = glm::ivec4(0, 0, title_dims.x as i32, title_dims.y as i32);
        renderer.show(&self.new_game)?;
        renderer.copy(&self.title, Some(&title_rectangle), None)
    }
}
