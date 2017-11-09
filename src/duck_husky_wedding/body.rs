use data;
use utils::Try;

use glm;
use moho;
use moho::shape::{Circle, Intersect, Rectangle, Shape};
use moho::renderer::{Renderer, Scene};
use sdl2::rect;

#[derive(Debug)]
pub struct Body {
    pub rectangles: Vec<Rectangle>,
    pub circles: Vec<Circle>,
}

impl Body {
    pub fn new(rect: &glm::DVec4, body: &[data::Shape], flip: bool) -> Self {
        let mut rectangles = vec![];
        let mut circles = vec![];

        for s in body {
            match *s {
                data::Shape::Rectangle(mut tl, d) => {
                    if flip {
                        tl.x = 100 - tl.x - d.x;
                    }
                    let tl = glm::DVec2::from(tl) / 100.;
                    let d = glm::DVec2::from(d) / 100.;
                    let top_left = glm::dvec2(rect.x + rect.z * tl.x, rect.y + rect.w * tl.y);
                    let dims = glm::dvec2(rect.z * d.x, rect.w * d.y);
                    rectangles.push(Rectangle { top_left, dims })
                }
                data::Shape::Circle(mut c, r) => {
                    if flip {
                        c.x = 100 - c.x;
                    }
                    let c = glm::DVec2::from(c) / 100.;
                    let r = f64::from(r) / 100.;
                    let center = glm::dvec2(rect.x + rect.z * c.x, rect.y + rect.w * c.y);
                    let radius = rect.z.min(rect.w) * r;
                    circles.push(Circle { center, radius })
                }
            }
        }
        Body {
            rectangles,
            circles,
        }
    }

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
        c_mtvs
            .chain(r_mtvs)
            .filter_map(|m| m)
            .take(2)
            .max_by(|&a, &b| {
                glm::length(a).partial_cmp(&glm::length(b)).unwrap()
            })
    }

    pub fn collides(&self, other: &Body) -> bool {
        other.rectangles.iter().any(|r| self.intersects(r))
            || other.circles.iter().any(|c| self.intersects(c))
    }

    pub fn intersects<S>(&self, other: &S) -> bool
    where
        Rectangle: Intersect<S>,
        Circle: Intersect<S>,
    {
        self.rectangles.iter().any(|r| r.intersects(other))
            || self.circles.iter().any(|c| c.intersects(other))
    }
}

impl<'t, R: Renderer<'t>> Scene<R> for Body {
    fn show(&self, renderer: &mut R) -> moho::errors::Result<()> {
        return Ok(());
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
            .map(|r| renderer.draw_rects(&[r]))
            .try()
    }
}
