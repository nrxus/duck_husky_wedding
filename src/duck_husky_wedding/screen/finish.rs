use duck_husky_wedding::button::{self, Button};

use errors::*;

use glm;
use moho::errors as moho_errors;
use moho::renderer::{options, Canvas, ColorRGBA, Font, FontTexturizer, Scene, Texture};
use moho::input;
use sdl2::rect::Rect;

use std::rc::Rc;
use std::time::Duration;

pub struct Data<F> {
    pub title_font: Rc<F>,
    pub detail_font: Rc<F>,
    pub view: glm::IVec4,
}

pub struct Finish<T> {
    button: button::Static<T>,
    view: glm::IVec4,
    title: T,
    score: T,
    time: T,
    total: T,
}

impl<T> Finish<T> {
    pub fn load<'t, F, FT>(
        data: &Data<F>,
        texturizer: &'t FT,
        score: u32,
        duration: Duration,
    ) -> Result<Self>
    where
        F: Font,
        FT: FontTexturizer<'t, F, Texture = T>,
    {
        let view = data.view;
        let button = button::Static::center_text(
            "DONE",
            texturizer,
            &*data.title_font,
            glm::ivec2(640, view.y + view.w - 45),
        )?;
        let title = texturizer
            .texturize(&*data.title_font, "FINISHED!", &ColorRGBA(255, 255, 0, 255))?;

        let duration = duration.as_secs() as u64;
        let total = texturizer.texturize(
            &*data.detail_font,
            &format!("     total: {:>05}", duration as u32 + score),
            &ColorRGBA(255, 255, 0, 255),
        )?;

        let score = texturizer.texturize(
            &*data.detail_font,
            &format!("     score: {:>05}", score),
            &ColorRGBA(255, 255, 0, 255),
        )?;

        let time = texturizer.texturize(
            &*data.detail_font,
            &format!("time bonus: {:>05}", duration),
            &ColorRGBA(255, 255, 0, 255),
        )?;

        Ok(Finish {
            title,
            button,
            view,
            score,
            time,
            total,
        })
    }

    pub fn update(&mut self, state: &input::State) -> Option<super::Kind> {
        if self.button.update(state) {
            Some(super::Kind::HighScore)
        } else {
            None
        }
    }
}

impl<'t, R: Canvas<'t>> Scene<R> for Finish<R::Texture>
where
    R::Texture: Texture,
{
    fn show(&self, renderer: &mut R) -> moho_errors::Result<()> {
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

        let mut top = self.view.y;
        //title
        {
            let texture = &self.title;
            let dims = glm::to_ivec2(texture.dims());
            let dst = glm::ivec4(640 - dims.x / 2, top, dims.x, dims.y);
            top += 5 + dims.y;
            renderer.copy(texture, options::at(&dst))
        }?;

        // score
        let left = 640 - self.score.dims().x as i32 / 2;
        {
            let texture = &self.score;
            let dims = glm::to_ivec2(texture.dims());
            let dst = glm::ivec4(left, top, dims.x, dims.y);
            top += 5 + dims.y;
            renderer.copy(texture, options::at(&dst))
        }?;

        // time
        {
            let texture = &self.time;
            let dims = glm::to_ivec2(texture.dims());
            let dst = glm::ivec4(left, top, dims.x, dims.y);
            top += 5 + dims.y;
            renderer.copy(texture, options::at(&dst))
        }?;

        // total
        {
            let texture = &self.total;
            let dims = glm::to_ivec2(texture.dims());
            let dst = glm::ivec4(left, top, dims.x, dims.y);
            renderer.copy(texture, options::at(&dst))
        }?;

        renderer.show(&self.button)
    }
}
