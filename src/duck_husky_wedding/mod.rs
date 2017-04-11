pub mod game_data;
mod player;
mod button;

use errors::*;
use self::button::Button;
use self::player::Player;
use self::game_data::GameData;

use glm;
use moho::input::{self, EventPump};
use moho::renderer::{Font, ColorRGBA, FontDetails, FontTexturizer, FontLoader, Renderer,
                     ResourceLoader, ResourceManager, Show, Texture};
use moho::timer::Timer;

use std::time::Duration;

pub struct DuckHuskyWedding<E, R, T>
    where E: EventPump
{
    input_manager: input::Manager<E>,
    title: T,
    player: Player<T>,
    renderer: R,
    button: Button<T>,
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
        let title_color = ColorRGBA(255, 255, 0, 255);
        let title = renderer
            .texturize(&font, "Husky <3's Ducky", &title_color)?;
        let file_name: &str = &format!("media/sprites/{}", game_data.duck.file_name);
        let texture = texture_manager.load(file_name, &renderer)?;
        let player = Player::new(game_data.duck, texture);
        let button = Button::from_text("click me",
                                       &renderer,
                                       &*font,
                                       glm::uvec2(100, 100),
                                       Box::new(|p| p.flip()))?;
        let game = DuckHuskyWedding {
            input_manager: input_manager,
            title: title,
            renderer: renderer,
            player: player,
            button: button,
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
                self.player.update();
                self.button.update(state, &mut self.player);
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

    fn draw(&mut self, interpolation: f64) -> Result<()>
        where R: Show
    {
        let title_dims = self.title.dims();
        let title_rectangle = glm::ivec4(0, 0, title_dims.x as i32, title_dims.y as i32);
        self.renderer.clear();
        self.renderer.show(&self.player)?;
        self.renderer.show(&self.button)?;
        self.renderer
            .copy(&self.title, Some(&title_rectangle), None)?;
        self.renderer.present();
        Ok(())
    }

    fn game_quit(&self) -> bool {
        self.input_manager.current.game_quit()
    }
}
