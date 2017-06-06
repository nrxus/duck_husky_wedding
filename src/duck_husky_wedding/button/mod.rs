use glm;
use moho::input;
use moho::shape::{Rectangle, Shape};
use sdl2::mouse::MouseButton;

mod b_static;

pub use self::b_static::Static;

pub trait Button {
    fn body(&self) -> &Rectangle;
    fn on_hover(&mut self, _hovers: bool) {}

    fn update(&mut self, input_state: &input::State) -> bool {
        let mouse = input_state.mouse_coords();
        if self.body().contains(&glm::to_dvec2(mouse)) {
            self.on_hover(true);
            input_state.did_click_mouse(MouseButton::Left)
        } else {
            self.on_hover(false);
            false
        }
    }
}
