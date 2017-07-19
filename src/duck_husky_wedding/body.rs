use duck_husky_wedding::try::Try;

use glm;
use moho::errors as moho_errors;
use moho::shape::{Circle, Shape, Rectangle, Intersect};
use moho::renderer::{Renderer, Scene};
use sdl2::rect;

pub struct Body {
    pub rectangles: Vec<Rectangle>,
    pub circles: Vec<Circle>,
}

impl Body {
    pub fn nudge(self, delta: glm::DVec2) -> Body {
        let rectangles = self.rectangles
            .into_iter()
            .map(|r| r.nudge(delta))
            .collect();
        let circles = self.circles.into_iter().map(|c| c.nudge(delta)).collect();
        Body {
            rectangles,
            circles,
        }
    }

    pub fn mtv<S>(&self, fixed: &S) -> Option<glm::DVec2>
    where
        Rectangle: Intersect<S>,
        Circle: Intersect<S>,
    {
        let c_mtvs = self.circles.iter().map(|c| c.mtv(fixed));
        let r_mtvs = self.rectangles.iter().map(|r| r.mtv(fixed));
        c_mtvs.chain(r_mtvs).filter_map(|m| m).nth(0)
    }
}

impl<'t, R: Renderer<'t>> Scene<R> for Body {
    fn show(&self, renderer: &mut R) -> moho_errors::Result<()> {
        let circles = self.circles.iter().map(|c| {
            let hdims = glm::dvec2(c.radius, c.radius);
            glm::dvec4(
                c.center.x - hdims.x,
                c.center.y - hdims.y,
                hdims.x * 2.,
                hdims.y * 2.,
            )
        });

        let rects = self.rectangles.iter().map(|r| {
            glm::dvec4(r.top_left.x, r.top_left.y, r.dims.x, r.dims.y)
        });

        circles
            .chain(rects)
            .map(|d| {
                rect::Rect::new(d.x as i32, d.y as i32, d.z as u32, d.w as u32)
            })
            .map(|r| renderer.fill_rects(&[r]))
            .try()
    }
}
