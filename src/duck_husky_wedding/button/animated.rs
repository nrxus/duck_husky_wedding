use moho::animation;
use glm;

use std::rc::Rc;

pub struct Animated<T> {
    pub idle: Rc<T>,
    pub animation: animation::Data<T>,
    pub dst: glm::IVec4,
}

impl<T> Clone for Animated<T> {
    fn clone(&self) -> Animated<T> {
        Animated {
            idle: self.idle.clone(),
            animation: self.animation.clone(),
            dst: self.dst,
        }
    }
}
