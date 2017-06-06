use duck_husky_wedding::button::{self, Button};
use errors::*;

use glm;
use serde_yaml;
use moho::input;
use moho::errors as moho_errors;
use moho::renderer::{options, ColorRGBA, Font, FontDetails, FontLoader, FontManager,
                     FontTexturizer, Renderer, Scene, Show, Texture};

use std::fs::File;

#[derive(Debug, Deserialize, Clone)]
struct ScoreEntry {
    score: u32,
    name: String,
}

pub struct HighScore<T> {
    title: T,
    back: button::Static<T>,
    scores: Vec<T>,
}

impl<T> HighScore<T> {
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
        let dims = font.measure("<")?;
        let top_left = glm::ivec2(10, 360 - dims.y as i32 / 2);
        let back = button::Static::from_text("<", texturizer, &*font, top_left)?;
        let title_color = ColorRGBA(255, 255, 0, 255);
        let title = texturizer.texturize(&*font, "High Scores", &title_color)?;
        Ok(HighScore {
               title: title,
               back: back,
               scores: vec![],
           })
    }

    pub fn load_scores<'f, 't, FT, FL>(&mut self,
                                       font_manager: &mut FontManager<'f, FL>,
                                       texturizer: &'t FT)
                                       -> Result<()>
        where T: Texture,
              FL: FontLoader<'f>,
              FT: FontTexturizer<'f, 't, Font = FL::Font, Texture = T>
    {
        let font_details = FontDetails {
            path: "media/fonts/joystix.monospace.ttf",
            size: 32,
        };
        let font = font_manager.load(&font_details)?;

        let path = "media/high_scores.yaml";
        let f = File::open(path)?;
        let color = ColorRGBA(255, 255, 255, 255);
        let scores: Vec<ScoreEntry> = serde_yaml::from_reader(&f)?;
        self.scores = scores
            .iter()
            .map(|s| {
                     let score = format!("{:04}{:5}{:>3}", s.score, "", s.name);
                     texturizer
                         .texturize(&*font, &score, &color)
                         .map_err(Into::into)
                 })
            .collect::<Result<Vec<_>>>()?;
        Ok(())
    }

    pub fn update(&mut self, input: &input::State) -> Option<super::Kind> {
        if self.back.update(input) {
            Some(super::Kind::Menu)
        } else {
            None
        }
    }
}

impl<'t, T, R> Scene<R> for HighScore<T>
    where T: Texture,
          R: Renderer<'t, Texture = T> + Show
{
    fn show(&self, renderer: &mut R) -> moho_errors::Result<()> {
        let title_dims = glm::to_ivec2(self.title.dims());
        let title_rectangle = glm::ivec4(640 - title_dims.x / 2, 0, title_dims.x, title_dims.y);
        renderer.show(&self.back)?;
        let mut height = 200;
        self.scores
            .iter()
            .map(|s| {
                     let dims = glm::to_ivec2(s.dims());
                     let dst = glm::ivec4(640 - dims.x / 2, height, dims.x, dims.y);
                     height += dims.y;
                     renderer.copy(s, options::at(&dst))
                 })
            .take_while(moho_errors::Result::is_ok)
            .last()
            .unwrap_or(Ok(()))?;
        renderer.copy(&self.title, options::at(&title_rectangle))
    }
}
