use duck_husky_wedding::button::{self, Button};
use duck_husky_wedding::try::Try;
use errors::*;

use glm;
use serde_yaml;
use moho::input;
use moho::errors as moho_errors;
use moho::renderer::{options, ColorRGBA, Font, FontDetails, FontLoader, FontManager,
                     FontTexturizer, Renderer, Scene, Texture};

use std::fs::File;
use std::rc::Rc;

#[derive(Debug, Deserialize, Clone)]
struct ScoreEntry {
    score: u32,
    name: String,
}

pub struct HighScore<T> {
    title: Rc<T>,
    back: button::Static<T>,
    scores: Vec<T>,
}

pub struct Data<T> {
    title: Rc<T>,
    back: button::Static<T>,
}

impl<T> Data<T> {
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
        let title = Rc::new(texturizer.texturize(&*font, "High Scores", &title_color)?);
        Ok(Data {
               title: title,
               back: back,
           })
    }

    pub fn activate<'f, 't, FT, FL>(&mut self,
                                    font_manager: &mut FontManager<'f, FL>,
                                    texturizer: &'t FT)
                                    -> Result<HighScore<T>>
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
        let scores = scores
            .iter()
            .map(|s| {
                     let score = format!("{:04}{:5}{:>3}", s.score, "", s.name);
                     texturizer
                         .texturize(&*font, &score, &color)
                         .map_err(Into::into)
                 })
            .collect::<Result<Vec<_>>>()?;
        Ok(HighScore {
               scores: scores,
               back: self.back.clone(),
               title: self.title.clone(),
           })
    }
}

impl<T> HighScore<T> {
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
          R: Renderer<'t, Texture = T>
{
    fn show(&self, renderer: &mut R) -> moho_errors::Result<()> {
        let title_dims = glm::to_ivec2(self.title.dims());
        let title_rectangle = glm::ivec4(640 - title_dims.x / 2, 0, title_dims.x, title_dims.y);
        renderer.show(&self.back)?;
        renderer.copy(&self.title, options::at(&title_rectangle))?;

        self.scores
            .iter()
            .enumerate()
            .map(|(i, s)| {
                     let dims = glm::to_ivec2(s.dims());
                     (s, glm::ivec4(640 - dims.x / 2, 200 * i as i32 + dims.y, dims.x, dims.y))
                 })
            .map(|(s, d)| renderer.copy(s, options::at(&d))).try()
    }
}
