use glm;
use moho::errors as moho_errors;
use moho::renderer::{Options, Renderer, Texture};
use sdl2::rect;

use std::cmp;

pub struct Camera<'c, R: 'c> {
    viewport: &'c ViewPort,
    renderer: &'c mut R,
}

#[derive(Debug)]
pub struct ViewPort {
    dims: glm::IVec2,
    translation: glm::IVec2,
}

impl ViewPort {
    pub fn new(dims: glm::IVec2) -> ViewPort {
        let translation = glm::ivec2(0, 0);
        ViewPort { dims, translation }
    }

    pub fn translate(&mut self, t: glm::IVec2) {
        self.translation = self.translation + t;
    }

    pub fn center(&mut self, center: glm::IVec2) {
        self.translation.x = cmp::max(center.x - self.dims.x / 2, 0);
    }

    pub fn contains(&self, rect: &glm::IVec4) -> bool {
        !(self.translation.x > rect.x + rect.z) && !(self.translation.x + self.dims.x < rect.x) &&
            !(self.translation.y > rect.y + rect.w) &&
            !(self.translation.y + self.dims.y < rect.y)
    }

    pub fn camera<'c, 't, R: Renderer<'t>>(&'c self, renderer: &'c mut R) -> Camera<'c, R> {
        Camera {
            viewport: self,
            renderer: renderer,
        }
    }
}

impl<'c, 't, R: Renderer<'t>> Renderer<'t> for Camera<'c, R>
where
    R::Texture: Texture,
{
    type Texture = R::Texture;

    fn draw_rects(&mut self, rects: &[rect::Rect]) -> moho_errors::Result<()> {
        let rects: Vec<_> = rects
            .iter()
            .map(|r| {
                rect::Rect::new(
                    r.x - self.viewport.translation.x,
                    r.y - self.viewport.translation.y,
                    r.width(),
                    r.height(),
                )
            })
            .collect();
        self.renderer.draw_rects(&rects)
    }

    fn fill_rects(&mut self, rects: &[rect::Rect]) -> moho_errors::Result<()> {
        let rects: Vec<_> = rects
            .iter()
            .map(|r| {
                rect::Rect::new(
                    r.x - self.viewport.translation.x,
                    r.y - self.viewport.translation.y,
                    r.width(),
                    r.height(),
                )
            })
            .collect();
        self.renderer.fill_rects(&rects)
    }

    fn copy(&mut self, texture: &Self::Texture, options: Options) -> moho_errors::Result<()> {
        match options.dst {
            Some(d) if self.viewport.contains(&d.rect(|| texture.dims())) => {
                let dst = d.nudge(-self.viewport.translation);
                self.renderer.copy(texture, options.at(dst))
            }
            Some(_) => Ok(()),
            None => self.renderer.copy(texture, options),
        }
    }
}
