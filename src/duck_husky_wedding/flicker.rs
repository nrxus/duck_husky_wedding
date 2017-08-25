use std::time::Duration;

pub enum State {
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

pub struct Flicker {
    duration: Duration,
    remaining: Duration,
    pub state: State,
}

impl Flicker {
    pub fn new(duration: Duration) -> Self {
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
}
