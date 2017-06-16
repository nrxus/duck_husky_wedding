use super::Button;

use glm;
use moho::animation::{self, Animation};
use moho::errors as moho_errors;
use moho::renderer::{options, Renderer, Scene, Texture};
use moho::shape::Rectangle;

use std::rc::Rc;
use std::time::Duration;

enum State<T> {
    Idle(Rc<T>),
    Hovering(Animation<T>),
}

pub struct Animated<T> {
    idle: Rc<T>,
    animation: animation::Data<T>,
    state: State<T>,
    body: Rectangle,
    focus_changed: bool,
}

impl<T> Clone for Animated<T> {
    fn clone(&self) -> Animated<T> {
        Animated::new(self.idle.clone(), self.animation.clone(), self.body.clone())
    }
}

impl<T> Button for Animated<T> {
    fn body(&self) -> &Rectangle {
        &self.body
    }

    fn on_hover(&mut self, hovers: bool) {
        self.focus_changed = true;

        match self.state {
            State::Idle(_) if hovers => {
                self.state = State::Hovering(self.animation.clone().start())
            }
            State::Hovering(_) if !hovers => self.state = State::Idle(self.idle.clone()),
            _ => self.focus_changed = false,
        }
    }
}

impl<T> Animated<T> {
    pub fn new(idle: Rc<T>, animation: animation::Data<T>, body: Rectangle) -> Self {
        let state = State::Idle(idle.clone());
        let focus_changed = false;

        Animated {
            idle,
            animation,
            body,
            state,
            focus_changed,
        }
    }

    pub fn animate(&mut self, delta: Duration) {
        if !self.focus_changed {
            if let State::Hovering(ref mut a) = self.state {
                a.animate(delta);
            }
        }
    }
}

impl<'t, T, R> Scene<R> for Animated<T>
where
    T: Texture,
    R: Renderer<'t, Texture = T>,
{
    fn show(&self, renderer: &mut R) -> moho_errors::Result<()> {
        let dst_rect = glm::to_ivec4(glm::dvec4(
            self.body.top_left.x,
            self.body.top_left.y,
            self.body.dims.x,
            self.body.dims.y,
        ));
        match self.state {
            State::Idle(ref t) => renderer.copy(&*t, options::at(&dst_rect)),
            State::Hovering(ref a) => renderer.copy_asset(&a.tile(), options::at(&dst_rect)),
        }
    }
}
