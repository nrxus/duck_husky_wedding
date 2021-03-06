pub mod camera;
pub mod world;
mod background;
mod high_score;
mod body;
mod button;
mod cat;
mod collectable;
mod edit_text;
mod flicker;
mod font;
mod goal;
mod hud;
mod npc;
mod obstacle;
mod player;
mod screen;

use errors::*;
use data;

use moho::input;
use moho::renderer::{Canvas, ColorRGBA, Font, FontLoader, FontManager, Texture, TextureLoader,
                     TextureManager};
use moho::timer::Timer;

use std::time::Duration;

pub struct DuckHuskyWedding<'f, 't, TL, FL, R, E>
where
    TL: 't + TextureLoader<'t>,
    FL: 'f + FontLoader<'f>,
{
    input_manager: input::Manager<E>,
    texture_manager: TextureManager<'t, TL>,
    font_manager: FontManager<'f, FL>,
    renderer: R,
}

impl<'f, 't, TL, FL, R, E> DuckHuskyWedding<'f, 't, TL, FL, R, E>
where
    TL: TextureLoader<'t>,
    TL::Texture: Texture,
    FL: FontLoader<'f>,
    FL::Font: Font<Texture = TL::Texture>,
{
    pub fn new(
        renderer: R,
        font_loader: &'f FL,
        texture_loader: &'t TL,
        input_manager: input::Manager<E>,
    ) -> Self {
        let texture_manager = TextureManager::new(texture_loader);
        let font_manager = FontManager::new(font_loader);
        DuckHuskyWedding {
            input_manager,
            texture_manager,
            font_manager,
            renderer,
        }
    }

    pub fn run(&mut self) -> Result<()>
    where
        R: Canvas<'t, Texture = <TL as TextureLoader<'t>>::Texture>,
        E: input::EventPump,
    {
        let game_data = data::Game::load("media/game_data.yaml")?;
        let level_data = data::Level::load("media/level.yaml")?;
        let mut screen_manager = screen::Manager::load(
            &mut self.font_manager,
            &mut self.texture_manager,
            &level_data,
            game_data,
        )?;

        const GAME_SPEED: u32 = 60;
        const MAX_SKIP: u32 = 10;
        let update_duration = Duration::new(0, 1_000_000_000 / GAME_SPEED);
        let mut timer = Timer::new();
        let mut delta = Duration::default();
        let color = ColorRGBA(60, 0, 70, 255);
        'game_loop: loop {
            let game_time = timer.update();
            delta += game_time.since_update;
            let mut loops: u32 = 0;
            while delta >= update_duration && loops < MAX_SKIP {
                let state = self.input_manager.update();
                if state.game_quit() {
                    break 'game_loop;
                }

                let next_screen = screen_manager.mut_screen().update(update_duration, state);
                if let Some(s) = next_screen {
                    screen_manager.select_screen(
                        s,
                        &mut self.font_manager,
                        &mut self.texture_manager,
                    );
                }

                delta -= update_duration;
                loops += 1;
            }
            // println!("fps: {:?}", game_time.fps());
            // let interpolation = delta.subsec_nanos() as f64
            // / update_duration.subsec_nanos() as f64;
            self.renderer.set_draw_color(color);
            self.renderer.clear();
            screen_manager.mut_screen().before_draw()?;
            self.renderer.show(screen_manager.screen())?;
            self.renderer.present();
        }
        Ok(())
    }
}
