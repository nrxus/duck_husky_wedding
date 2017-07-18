use glm;
use moho::shape::{Circle, Shape, Rectangle, Intersect};

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
