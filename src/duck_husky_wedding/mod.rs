pub mod game_data;
mod player;

use errors::*;
use self::player::Player;
use self::game_data::GameData;

use moho::input_manager::{InputManager, EventPump};
use moho::renderer::{Renderer, ResourceLoader, Texture};
use moho::timer::Timer;

use std::time::Duration;

pub struct DuckHuskyWedding<E: EventPump, R, T: Texture> {
    input_manager: InputManager<E>,
    renderer: R,
    player: Player<T>,
}

impl<E: EventPump, R, T: Texture> DuckHuskyWedding<E, R, T>
    where R: Renderer<T> + for<'a> ResourceLoader<'a, T>
{
    pub fn load(renderer: R, input_manager: InputManager<E>, game_data: GameData) -> Result<Self> {
        let player = Player::load(game_data.duck, &renderer)?;
        Ok(Self::new(renderer, input_manager, player))
    }

    pub fn new(renderer: R, input_manager: InputManager<E>, player: Player<T>) -> Self {
        DuckHuskyWedding {
            input_manager: input_manager,
            renderer: renderer,
            player: player,
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
            self.player.animate(game_time.since_update);
            let interpolation = delta.subsec_nanos() as f64 / update_duration.subsec_nanos() as f64;
            self.draw(interpolation)?;
        }
        Ok(())
    }

    fn update(&mut self) {
        self.player.update();
    }

    fn draw(&mut self, interpolation: f64) -> Result<()> {
        self.renderer.clear();
        self.renderer.show(&self.player)?;
        self.renderer.present();
        Ok(())
    }

    fn game_quit(&self) -> bool {
        self.input_manager.game_quit()
    }
}
