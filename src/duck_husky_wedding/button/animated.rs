use moho::animation;
use moho::renderer::options;

use std::rc::Rc;

pub struct Animated<T> {
    pub idle: Rc<T>,
    pub animation: animation::Data<T>,
    pub dst: options::Destination,
}

impl<T> Clone for Animated<T> {
    fn clone(&self) -> Animated<T> {
        Animated {
            idle: Rc::clone(&self.idle),
            animation: self.animation.clone(),
            dst: self.dst,
        }
    }
}
