use glm;
use moho::errors as moho_errors;
use moho::renderer::{Options, Renderer};
use sdl2::rect;

struct Camera<'c, R: 'c> {
    viewport: &'c ViewPort,
    renderer: &'c mut R,
}

struct ViewPort {
    dims: glm::IVec2,
    translation: glm::IVec2,
}

impl ViewPort {
    fn new(dims: glm::IVec2) -> ViewPort {
        let translation = glm::ivec2(0, 0);
        ViewPort { dims, translation }
    }

    fn translate(&mut self, t: glm::IVec2) {
        self.translation = self.translation + t;
    }

    fn center(&mut self, center: glm::IVec2) {
        self.translation = self.dims / 2 + self.translation - center;
    }

    fn camera<'c, 't, R: Renderer<'t>>(&'c self, renderer: &'c mut R) -> Camera<'c, R> {
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

    fn copy(&mut self, texture: &Self::Texture, mut options: Options) -> moho_errors::Result<()> {
        //TODO: move dst
        options.dst = options.dst.map(|r| r);
        self.renderer.copy(texture, options)
    }
}
