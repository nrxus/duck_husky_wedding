use duck_husky_wedding::edit_text::EditText;
use duck_husky_wedding::high_score::{self, ScoreEntry};

use errors::*;

use glm;
use moho;
use moho::renderer::{align, options, Canvas, ColorRGBA, Font, Scene, Texture};
use moho::input;
use sdl2::rect::Rect;
use sdl2::keyboard::Keycode;

use std::rc::Rc;
use std::time::Duration;

pub struct Data<F> {
    pub title_font: Rc<F>,
    pub detail_font: Rc<F>,
    pub view: glm::IVec4,
}

pub struct ScoreData<T, F> {
    previous: Vec<ScoreEntry>,
    current: u32,
    name: EditText<T, F>,
}

impl<T, F: Font<Texture = T>> ScoreData<T, F> {
    fn extract(&self) -> Option<Vec<ScoreEntry>> {
        let name = self.name.extract();
        if name.is_empty() {
            None
        } else {
            let mut updated = self.previous.clone();
            updated.push(ScoreEntry {
                name,
                score: self.current,
            });
            updated.sort_by(|a, b| a.score.cmp(&b.score).reverse());
            updated.truncate(10);
            Some(updated)
        }
    }
}

pub struct Finish<T, F> {
    instructions: T,
    view: glm::IVec4,
    title: T,
    score: T,
    time: T,
    total: T,
    score_entry: Option<ScoreData<T, F>>,
}

impl<T, F: Font<Texture = T>> Finish<T, F> {
    pub fn load(data: &Data<F>, score: u32, duration: Duration) -> Result<Self> {
        let yellow = ColorRGBA(255, 255, 0, 255);
        let white = ColorRGBA(255, 255, 255, 255);
        let view = data.view;
        let title = data.title_font.texturize("FINISHED!", &yellow)?;

        let instructions = data.title_font
            .texturize("<PRESS ENTER TO CONTINUE>", &yellow)?;

        let duration = 5 * duration.as_secs();
        let new_score = duration as u32 + score;
        let total = data.detail_font
            .texturize(&format!("     total: {:>06}", new_score), &white)?;

        let score = data.detail_font
            .texturize(&format!("     score: {:>06}", score), &white)?;

        let time = data.detail_font
            .texturize(&format!("time bonus: {:>06}", duration), &white)?;

        let name = EditText::load(
            "Enter Name: ",
            glm::ivec2(369, 400),
            Rc::clone(&data.detail_font),
        )?;

        let score_entry = {
            let previous = high_score::get();
            let min_score = if previous.len() < 10 {
                None
            } else {
                previous.iter().map(|s| s.score).min()
            };

            match min_score {
                Some(min_score) if min_score > new_score => None,
                _ => Some(ScoreData {
                    previous,
                    name,
                    current: new_score,
                }),
            }
        };

        Ok(Finish {
            title,
            instructions,
            view,
            score,
            time,
            total,
            score_entry,
        })
    }

    pub fn before_draw(&mut self) -> Result<()> {
        if let Some(ref mut s) = self.score_entry {
            s.name.before_draw()
        } else {
            Ok(())
        }
    }

    pub fn update(&mut self, elapsed: Duration, state: &input::State) -> Option<super::Kind> {
        if state.did_press_key(Keycode::Return) {
            match self.score_entry {
                None => Some(super::Kind::Menu),
                Some(ref s) => s.extract().map(|ref s| {
                    high_score::create(s).expect("could not write to high score file");
                    super::Kind::HighScore
                }),
            }
        } else {
            if let Some(ref mut s) = self.score_entry {
                s.name.update(elapsed, state);
            }
            None
        }
    }
}

impl<'t, R: Canvas<'t>, F> Scene<R> for Finish<R::Texture, F>
where
    R::Texture: Texture,
{
    fn show(&self, renderer: &mut R) -> moho::errors::Result<()> {
        renderer.set_draw_color(ColorRGBA(0, 0, 0, 255));
        renderer.fill_rects(&[
            Rect::new(
                self.view.x,
                self.view.y,
                self.view.z as u32,
                self.view.w as u32,
            ),
        ])?;
        renderer.set_draw_color(ColorRGBA(60, 0, 70, 255));
        renderer.fill_rects(&[
            Rect::new(
                self.view.x + 6,
                self.view.y + 6,
                self.view.z as u32 - 12,
                self.view.w as u32 - 12,
            ),
        ])?;

        let mut top = align::top(self.view.y);
        //title
        {
            let texture = &self.title;
            let options = options::at(top.center(640));
            top = top.nudge(5 + texture.dims().y as i32);
            renderer.copy(texture, options)
        }?;

        // score
        let left = 640 - self.score.dims().x as i32 / 2;
        let mut dst = top.left(left);
        {
            let texture = &self.score;
            let options = options::at(dst);
            dst = dst.nudge(glm::ivec2(0, 5 + texture.dims().y as i32));
            renderer.copy(texture, options)
        }?;

        // time
        {
            let texture = &self.time;
            let options = options::at(dst);
            dst = dst.nudge(glm::ivec2(0, 5 + texture.dims().y as i32));
            renderer.copy(texture, options)
        }?;

        // total
        renderer.copy(&self.total, options::at(dst))?;

        if let Some(ref s) = self.score_entry {
            renderer.show(&s.name)?;
        }

        // instructions
        renderer.copy(
            &self.instructions,
            options::at(align::bottom(self.view.y + self.view.w - 5).center(640)),
        )
    }
}
