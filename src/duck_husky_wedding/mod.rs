use errors::*;

use moho::input_manager::InputManager;
use moho::resource_manager::Renderer;
use moho::MohoEngine;
use moho::timer::Timer;

use std::time::Duration;

pub struct DuckHuskyWedding<E: MohoEngine> {
    input_manager: InputManager<E::EventPump>,
    renderer: E::Renderer,
}

impl<E: MohoEngine> DuckHuskyWedding<E> {
    pub fn new(renderer: E::Renderer, input_manager: InputManager<E::EventPump>) -> Self {
        DuckHuskyWedding {
            input_manager: input_manager,
            renderer: renderer,
        }
    }

    pub fn run(&mut self) -> Result<()> {
        const GAME_SPEED: u32 = 60;
        const MAX_SKIP: u32 = 10;
        let update_duration = Duration::new(0, 1000000000 / GAME_SPEED);
        let mut timer = Timer::new();
        let mut delta: Duration = Default::default();
        while !self.game_quit() {
            let game_time = timer.update();
            delta += game_time.since_update;
            let mut loops: u32 = 0;
            while delta >= update_duration && loops < MAX_SKIP {
                self.input_manager.update();
                if self.game_quit() {
                    break;
                }
                self.update();
                delta -= update_duration;
                loops += 1;
            }
            if self.game_quit() {
                break;
            }
            let interpolation = delta.subsec_nanos() as f64 / update_duration.subsec_nanos() as f64;
            self.draw(interpolation)?;
        }
        Ok(())
    }

    fn update(&mut self) {}

    fn draw(&mut self, interpolation: f64) -> Result<()> {
        self.renderer.clear();
        self.renderer.present();
        Ok(())
    }

    fn game_quit(&self) -> bool {
        self.input_manager.game_quit()
    }
}
