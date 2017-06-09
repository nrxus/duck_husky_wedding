use errors::*;

use glm;
use moho::renderer::{Asset, Options, Renderer};

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

impl<'c, R: Renderer<'c>> Camera<'c, R> {
    fn display(&mut self, texture: &R::Texture, mut options: Options) -> Result<()> {
        options.dst = options.dst.map(|r| r);
        self.renderer.copy(texture, options).map_err(Into::into)
    }

    fn display_asset<A>(&mut self, drawable: &A, mut options: Options) -> Result<()>
        where A: Asset<R>
    {
        options.dst = options.dst.map(|r| r);
        self.renderer
            .copy_asset(drawable, options)
            .map_err(Into::into)
    }
}
