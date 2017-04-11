pub mod game_data;
mod player;
mod menu_screen;
mod button;

use errors::*;
use self::game_data::GameData;
use self::menu_screen::MenuScreen;

use moho::input::{self, EventPump};
use moho::renderer::{Font, FontDetails, FontTexturizer, FontLoader, Renderer, ResourceLoader,
                     ResourceManager, Show, Texture};
use moho::timer::Timer;

use std::time::Duration;

pub struct DuckHuskyWedding<E, R, T>
    where E: EventPump
{
    input_manager: input::Manager<E>,
    menu_screen: MenuScreen<T>,
    renderer: R,
}

impl<'f, E, R, T> DuckHuskyWedding<E, R, T>
    where E: EventPump,
          T: Texture,
          R: Renderer<Texture = T>
{
    pub fn load<F, FL>(renderer: R,
                       font_loader: &'f FL,
                       input_manager: input::Manager<E>,
                       game_data: GameData)
                       -> Result<Self>
        where F: Font,
              FL: FontLoader<'f, Font = F>,
              R: for<'a> ResourceLoader<Texture = T> + FontTexturizer<'f, Font = F, Texture = T>
    {
        let mut texture_manager: ResourceManager<String, T> = ResourceManager::new();
        let mut font_manager: ResourceManager<FontDetails, F> = ResourceManager::new();
        let font_details = FontDetails {
            path: "media/fonts/kenpixel_mini.ttf",
            size: 64,
        };
        let font = font_manager.load(&font_details, font_loader)?;
        let file_name: &str = &format!("media/sprites/{}", game_data.duck.file_name);
        let texture = texture_manager.load(file_name, &renderer)?;
        let menu_screen = MenuScreen::load(&*font, &renderer, game_data.duck, texture)?;
        let game = DuckHuskyWedding {
            menu_screen: menu_screen,
            input_manager: input_manager,
            renderer: renderer,
        };
        Ok(game)
    }

    pub fn run(&mut self) -> Result<()>
        where R: Show
    {
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
                self.menu_screen.update(state);
                delta -= update_duration;
                loops += 1;
            }
            if self.game_quit() {
                break;
            }
            self.menu_screen.animate(game_time.since_update);
            let interpolation = delta.subsec_nanos() as f64 / update_duration.subsec_nanos() as f64;
            self.draw(interpolation)?;
        }
        Ok(())
    }

    fn draw(&mut self, interpolation: f64) -> Result<()>
        where R: Show
    {
        self.renderer.clear();
        self.renderer.show(&self.menu_screen)?;
        self.renderer.present();
        Ok(())
    }

    fn game_quit(&self) -> bool {
        self.input_manager.current.game_quit()
    }
}
