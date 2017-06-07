mod game_data;
mod player;
mod button;
mod screen;
mod world;

use errors::*;
use self::game_data::GameData;

use moho::input;
use moho::renderer::{ColorRGBA, FontTexturizer, FontLoader, Renderer, TextureLoader,
                     TextureManager, FontManager, Show};
use moho::timer::Timer;

use std::time::Duration;

pub struct DuckHuskyWedding<'f, 't, TL, FL, R, E>
    where TL: 't + TextureLoader<'t>,
          FL: 'f + FontLoader<'f>
{
    input_manager: input::Manager<E>,
    texture_manager: TextureManager<'t, TL>,
    font_manager: FontManager<'f, FL>,
    renderer: R,
    texture_loader: &'t TL,
}

impl<'f, 't, TL, FL, R, E> DuckHuskyWedding<'f, 't, TL, FL, R, E>
    where TL: TextureLoader<'t>,
          FL: FontLoader<'f>
{
    pub fn new(renderer: R,
               font_loader: &'f FL,
               texture_loader: &'t TL,
               input_manager: input::Manager<E>)
               -> Self {
        let texture_manager = TextureManager::new(texture_loader);
        let font_manager = FontManager::new(font_loader);
        DuckHuskyWedding {
            input_manager: input_manager,
            texture_manager: texture_manager,
            font_manager: font_manager,
            renderer: renderer,
            texture_loader: texture_loader,
        }
    }

    pub fn run(&mut self) -> Result<()>
        where TL: FontTexturizer<'f,
                                 't,
                                 Texture = <TL as TextureLoader<'t>>::Texture,
                                 Font = FL::Font>,
              R: Renderer<'t, Texture = <TL as TextureLoader<'t>>::Texture> + Show,
              E: input::EventPump
    {
        let game_data = GameData::load("media/game_data.yaml")?;
        let mut screen_manager = screen::Manager::load(&mut self.font_manager,
                                                       &mut self.texture_manager,
                                                       self.texture_loader,
                                                       game_data)?;

        const GAME_SPEED: u32 = 60;
        const MAX_SKIP: u32 = 10;
        let update_duration = Duration::new(0, 1000000000 / GAME_SPEED);
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
                    screen_manager.select_screen(s,
                                                 &mut self.font_manager,
                                                 &mut self.texture_manager,
                                                 self.texture_loader);
                }

                delta -= update_duration;
                loops += 1;
            }
            let interpolation = delta.subsec_nanos() as f64 / update_duration.subsec_nanos() as f64;
            self.renderer.set_draw_color(color);
            self.renderer.clear();
            self.renderer.show(screen_manager.screen())?;
            self.renderer.present();
        }
        Ok(())
    }
}
