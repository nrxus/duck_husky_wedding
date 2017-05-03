pub mod game_data;
mod player;
mod menu_screen;
mod button;

use errors::*;
use self::game_data::GameData;
use self::menu_screen::MenuScreen;

use moho::input;
use moho::renderer::{Font, FontDetails, FontTexturizer, FontLoader, Renderer, TextureLoader,
                     TextureManager, FontManager, Show, Texture};
use moho::timer::Timer;

use std::time::Duration;

pub struct DuckHuskyWedding<'f, 't, TL: 't, FL: 'f, R, T, F, E> {
    input_manager: input::Manager<E>,
    texture_manager: TextureManager<'t, T, TL>,
    font_manager: FontManager<'f, F, FL>,
    renderer: R,
    texture_loader: &'t TL,
}

impl<'f, 't, TL, FL, R, T, F, E> DuckHuskyWedding<'f, 't, TL, FL, R, T, F, E> {
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

    pub fn run(&mut self, game_data: GameData) -> Result<()>
        where TL: TextureLoader<'t, Texture = T> + FontTexturizer<'f, 't, Texture = T, Font = F>,
              FL: FontLoader<'f, Font = F>,
              R: Renderer<'t, Texture = T> + Show,
              E: input::EventPump,
              T: Texture,
              F: Font
    {
        let font_details = FontDetails {
            path: "media/fonts/kenpixel_mini.ttf",
            size: 64,
        };
        let font = self.font_manager.load(&font_details)?;
        let file_name: &str = &format!("media/sprites/{}", game_data.duck.file_name);
        let texture = self.texture_manager.load(file_name)?;
        let mut menu_screen =
            MenuScreen::load(&*font, self.texture_loader, game_data.duck, texture)?;

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
                let state = self.input_manager.update();
                if state.game_quit() {
                    break;
                }
                menu_screen.update(state);
                delta -= update_duration;
                loops += 1;
            }
            if self.game_quit() {
                break;
            }
            menu_screen.animate(game_time.since_update);
            let interpolation = delta.subsec_nanos() as f64 / update_duration.subsec_nanos() as f64;
            self.renderer.clear();
            self.renderer.show(&menu_screen)?;
            self.renderer.present();
        }
        Ok(())
    }

    fn game_quit(&self) -> bool {
        self.input_manager.current.game_quit()
    }
}
