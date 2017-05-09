use duck_husky_wedding::button::Button;
use errors::*;

use glm;
use moho::input;
use moho::errors as moho_errors;
use moho::renderer::{ColorRGBA, Font, FontDetails, FontLoader, FontManager, FontTexturizer,
                     Renderer, Scene, Show, Texture};

use std::rc::Rc;

pub struct HighScore<T, F> {
    title: T,
    back: Button<T>,
    score_font: Rc<F>,
}

impl<T, F: Font> HighScore<T, F> {
    pub fn load<'f, 't, FT, FL>(font_manager: &mut FontManager<'f, FL>,
                                texturizer: &'t FT)
                                -> Result<Self>
        where T: Texture,
              FL: FontLoader<'f, Font = F>,
              FT: FontTexturizer<'f, 't, Font = F, Texture = T>
    {
        let font_details = FontDetails {
            path: "media/fonts/kenpixel_mini.ttf",
            size: 64,
        };
        let font = font_manager.load(&font_details)?;
        let dims = font.measure("<")?;
        let top_left = glm::ivec2(10, 360 - dims.y as i32 / 2);
        let back = Button::from_text("<", texturizer, &*font, top_left)?;
        let title_color = ColorRGBA(255, 255, 0, 255);
        let title = texturizer
            .texturize(&*font, "HIGH SCORES", &title_color)?;
        let score_font_details = FontDetails {
            path: "media/fonts/kenpixel_mini.ttf",
            size: 32,
        };
        let score_font = font_manager.load(&score_font_details)?;
        Ok(HighScore {
               title: title,
               back: back,
               score_font: score_font,
           })
    }

    pub fn update(&mut self, input: &input::State) -> Option<super::Kind> {
        if self.back.update(input) {
            Some(super::Kind::Menu)
        } else {
            None
        }
    }
}

impl<'t, T, F, R> Scene<R> for HighScore<T, F>
    where T: Texture,
          R: Renderer<'t, Texture = T> + Show
{
    fn show(&self, renderer: &mut R) -> moho_errors::Result<()> {
        let title_dims = glm::to_ivec2(self.title.dims());
        let title_rectangle = glm::ivec4(640 - title_dims.x / 2, 0, title_dims.x, title_dims.y);
        renderer.show(&self.back)?;
        renderer.copy(&self.title, Some(&title_rectangle), None)
    }
}
