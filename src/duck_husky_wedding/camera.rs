use glm;
use moho::errors as moho_errors;
use moho::renderer::{Options, Renderer};
use sdl2::rect;

use std::cmp;

pub struct Camera<'c, R: 'c> {
    viewport: &'c ViewPort,
    renderer: &'c mut R,
}

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
        self.translation.x = cmp::min(self.dims.x / 2 - center.x, 0);
    }

    pub fn camera<'c, 't, R: Renderer<'t>>(&'c self, renderer: &'c mut R) -> Camera<'c, R> {
        Camera {
            viewport: self,
            renderer: renderer,
        }
    }
}

impl<'c, 't, R: Renderer<'t>> Renderer<'t> for Camera<'c, R> {
    type Texture = R::Texture;

    fn fill_rects(&mut self, rects: &[rect::Rect]) -> moho_errors::Result<()> {
        //TODO: move rectangles
        self.renderer.fill_rects(rects)
    }

    fn copy(&mut self, texture: &Self::Texture, options: Options) -> moho_errors::Result<()> {
        let mut dst = glm::ivec4(self.viewport.translation.x,
                                 self.viewport.translation.y,
                                 0,
                                 0);
        let dst = options
            .dst
            .map(|d| {
                     dst = dst + *d;
                     &dst
                 });
        let mut options = options;
        options.dst = dst;

        self.renderer.copy(texture, options)
    }
}
