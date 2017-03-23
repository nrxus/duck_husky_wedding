pub mod game_data;

use errors::*;
use self::game_data::GameData;

use glm;
use moho::input_manager::InputManager;
use moho::resource_manager::{Renderer, ResourceLoader, TileSheet, FrameAnimator};
use moho::MohoEngine;
use moho::timer::Timer;

use std::time::Duration;

pub struct DuckHuskyWedding<E: MohoEngine> {
    input_manager: InputManager<E::EventPump>,
    renderer: E::Renderer,
    sheet: TileSheet,
    animator: FrameAnimator,
    dimensions: glm::UVec2,
    position: glm::IVec2,
}

impl<E: MohoEngine> DuckHuskyWedding<E> {
    pub fn new(renderer: E::Renderer,
               input_manager: InputManager<E::EventPump>,
               game_data: GameData)
               -> Self {
        let duck_data = game_data.duck;
        let duck_file = format!("media/sprites/{}", duck_data.file_name);
        let duck_texture = renderer.load_texture(&duck_file).unwrap();
        let sheet = TileSheet::new(duck_data.tiles.into(), duck_texture);
        let animator = FrameAnimator::new(duck_data.frames, Duration::from_millis(60), true);
        println!("{:?}", duck_texture.id);
        DuckHuskyWedding {
            input_manager: input_manager,
            sheet: sheet,
            animator: animator,
            renderer: renderer,
            dimensions: duck_data.out_size.into(),
            position: glm::ivec2(0, 300),
        }
    }

    pub fn run(&mut self) -> Result<()> {
        const GAME_SPEED: u32 = 60;
        const MAX_SKIP: u32 = 10;
        let update_duration = Duration::new(0, 1000000000 / GAME_SPEED);
        let mut timer = Timer::new();
        let mut delta: Duration = Default::default();
        self.animator.start();
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
            self.animator.animate(delta);
            let interpolation = delta.subsec_nanos() as f64 / update_duration.subsec_nanos() as f64;
            self.draw(interpolation)?;
        }
        Ok(())
    }

    fn update(&mut self) {
        self.position.x = (self.position.x + 5) % 1280;
    }

    fn draw(&mut self, interpolation: f64) -> Result<()> {
        self.renderer.clear();
        let tile = self.sheet.tile(self.animator.frame().unwrap());
        let dst_rect = glm::ivec4(self.position.x,
                                  self.position.y,
                                  self.dimensions.x as i32,
                                  self.dimensions.y as i32);
        self.renderer.render(&tile, dst_rect);
        self.renderer.present();
        Ok(())
    }

    fn game_quit(&self) -> bool {
        self.input_manager.game_quit()
    }
}
