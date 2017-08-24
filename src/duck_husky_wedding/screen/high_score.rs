use duck_husky_wedding::font;
use utils::Try;
use errors::*;

use serde_yaml;
use moho::input;
use moho::errors as moho_errors;
use moho::renderer::{align, options, ColorRGBA, FontTexturizer, Renderer, Scene, Texture};
use sdl2::keyboard::Keycode;

use std::fs::File;
use std::rc::Rc;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ScoreEntry {
    pub score: u32,
    pub name: String,
}

pub struct HighScore<T> {
    title: Rc<T>,
    instructions: Rc<T>,
    scores: Vec<T>,
}

pub struct Data<T> {
    title: Rc<T>,
    instructions: Rc<T>,
}

impl<T> Data<T> {
    pub fn load<'f, 't, FT, FM>(font_manager: &mut FM, texturizer: &'t FT) -> Result<Self>
    where
        FM: font::Manager,
        FT: FontTexturizer<'t, FM::Font, Texture = T>,
    {
        let color = ColorRGBA(255, 255, 0, 255);

        let title = {
            let text = "High Scores";
            let font = font_manager.load(font::Kind::KenPixel, 64)?;
            texturizer.texturize(&*font, text, &color).map(Rc::new)
        }?;
        let instructions = {
            let text = "<PRESS ENTER TO GO TO MAIN MENU>";
            let font = font_manager.load(font::Kind::KenPixel, 32)?;
            texturizer.texturize(&*font, text, &color).map(Rc::new)
        }?;

        Ok(Data {
            title,
            instructions,
        })
    }

    pub fn activate<'f, 't, FT, FM>(
        &mut self,
        font_manager: &mut FM,
        texturizer: &'t FT,
    ) -> Result<HighScore<T>>
    where
        FM: font::Manager,
        FT: FontTexturizer<'t, FM::Font, Texture = T>,
    {
        let font = font_manager.load(font::Kind::Joystix, 32)?;

        let path = "media/high_scores.yaml";
        let f = File::open(path)?;
        let color = ColorRGBA(255, 255, 255, 255);
        let scores: Vec<ScoreEntry> = serde_yaml::from_reader(&f).unwrap_or_default();
        let scores = scores
            .iter()
            .map(|s| {
                let score = format!("{:06}{:5}{:>6}", s.score, "", s.name);
                texturizer
                    .texturize(&*font, &score, &color)
                    .map_err(Into::into)
            })
            .collect::<Result<Vec<_>>>()?;
        Ok(HighScore {
            scores: scores,
            title: self.title.clone(),
            instructions: self.instructions.clone(),
        })
    }
}

impl<T> HighScore<T> {
    pub fn update(&mut self, input: &input::State) -> Option<super::Kind> {
        if input.did_press_key(Keycode::Return) {
            Some(super::Kind::Menu)
        } else {
            None
        }
    }
}

impl<'t, R: Renderer<'t>> Scene<R> for HighScore<R::Texture>
where
    R::Texture: Texture,
{
    fn show(&self, renderer: &mut R) -> moho_errors::Result<()> {
        let center = align::center(640);

        renderer.copy(&*self.title, options::at(center.top(0)))?;

        {
            let texture = &*self.instructions;
            let dst = center.bottom(720 - texture.dims().y as i32);
            renderer.copy(texture, options::at(dst))
        }?;

        self.scores
            .iter()
            .enumerate()
            .map(|(i, s)| ((s.dims().y * i as u32) as i32, s))
            .map(|(d, s)| renderer.copy(s, options::at(center.top(150 + d))))
            .try()
    }
}
