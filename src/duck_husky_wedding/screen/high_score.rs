use duck_husky_wedding::font;
use duck_husky_wedding::high_score;
use utils::Try;
use errors::*;

use moho::{self, input};
use moho::renderer::{align, options, ColorRGBA, Font, Renderer, Scene, Texture};
use sdl2::keyboard::Keycode;

use std::rc::Rc;

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
    pub fn load<FM>(font_manager: &mut FM) -> Result<Self>
    where
        FM: font::Manager,
        FM::Font: Font<Texture = T>,
    {
        let color = ColorRGBA(255, 255, 0, 255);

        let title = {
            let text = "High Scores";
            let font = font_manager.load(font::Kind::KenPixel, 64)?;
            font.texturize(text, &color).map(Rc::new)
        }?;
        let instructions = {
            let text = "<PRESS ENTER TO GO TO MAIN MENU>";
            let font = font_manager.load(font::Kind::KenPixel, 32)?;
            font.texturize(text, &color).map(Rc::new)
        }?;

        Ok(Data {
            title,
            instructions,
        })
    }

    pub fn activate<FM>(&mut self, font_manager: &mut FM) -> Result<HighScore<T>>
    where
        FM: font::Manager,
        FM::Font: Font<Texture = T>,
    {
        let font = font_manager.load(font::Kind::Joystix, 32)?;
        let color = ColorRGBA(255, 255, 255, 255);

        let scores = high_score::get()
            .iter()
            .map(|s| {
                let score = format!("{:06}{:5}{:>6}", s.score, "", s.name);
                font.texturize(&score, &color).map_err(Into::into)
            })
            .collect::<Result<Vec<_>>>()?;
        Ok(HighScore {
            scores: scores,
            title: Rc::clone(&self.title),
            instructions: Rc::clone(&self.instructions),
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
    fn show(&self, renderer: &mut R) -> moho::errors::Result<()> {
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
