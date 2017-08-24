use moho::animation;
use moho::renderer::Destination;

use std::rc::Rc;

pub struct Animated<T> {
    pub idle: Rc<T>,
    pub animation: animation::Data<T>,
    pub dst: Destination,
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
