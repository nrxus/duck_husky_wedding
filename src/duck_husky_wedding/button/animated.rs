use moho::animation;
use moho::shape::Rectangle;

use std::rc::Rc;

pub struct Animated<T> {
    pub idle: Rc<T>,
    pub animation: animation::Data<T>,
    pub body: Rectangle,
}

impl<T> Clone for Animated<T> {
    fn clone(&self) -> Animated<T> {
        Animated::new(self.idle.clone(), self.animation.clone(), self.body.clone())
    }
}

impl<T> Animated<T> {
    pub fn new(idle: Rc<T>, animation: animation::Data<T>, body: Rectangle) -> Self {
        Animated {
            idle,
            animation,
            body,
        }
    }
}
