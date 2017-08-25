use std::time::Duration;

#[derive(Debug, Clone, Copy)]
pub struct Flicker {
    duration: Duration,
    remaining: Duration,
    state: State,
}

impl Flicker {
    pub fn new(duration: Duration) -> Flicker {
        Flicker {
            duration,
            state: State::Show,
            remaining: duration,
        }
    }

    pub fn update(&mut self, delta: Duration) {
        match self.remaining.checked_sub(delta) {
            None => {
                self.state.toggle();
                self.remaining = self.duration;
            }
            Some(d) => self.remaining = d,
        }
    }

    pub fn is_shown(&self) -> bool {
        match self.state {
            State::Hide => false,
            State::Show => true,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum State {
    Hide,
    Show,
}

impl State {
    pub fn toggle(&mut self) {
        *self = match *self {
            State::Hide => State::Show,
            State::Show => State::Hide,
        }
    }
}
