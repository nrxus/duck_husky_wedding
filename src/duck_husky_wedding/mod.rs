pub mod game_data;
mod player;

use errors::*;
use self::player::Player;
use self::game_data::GameData;

use glm;
use moho::shape::Rectangle;
use moho::input_manager::{InputManager, EventPump};
use moho::renderer::{Font, ColorRGBA, FontDetails, FontTexturizer, FontLoader, Renderer,
                     ResourceLoader, ResourceManager, Texture};
use moho::timer::Timer;

use std::time::Duration;

pub struct DuckHuskyWedding<E: EventPump, R, T: Texture> {
    input_manager: InputManager<E>,
    title: T,
    player: Player<T>,
    renderer: R,
}

impl<E: EventPump, R, T: Texture> DuckHuskyWedding<E, R, T>
    where R: Renderer<T>
{
    pub fn load<'ttf, F: Font, FL>(renderer: R,
                                   font_loader: &'ttf FL,
                                   input_manager: InputManager<E>,
                                   game_data: GameData)
                                   -> Result<Self>
        where FL: FontLoader<'ttf, F>,
              R: FontTexturizer<'ttf, F, Texture = T> + for<'a> ResourceLoader<'a, T>
    {
        let mut texture_manager: ResourceManager<T, String> = ResourceManager::new();
        let mut font_manager: ResourceManager<F, FontDetails> = ResourceManager::new();
        let font_details = FontDetails {
            path: "media/fonts/kenpixel_mini.ttf",
            size: 64,
        };
        let font = font_manager.load(&font_details, font_loader)?;
        let title_color = ColorRGBA(255, 255, 0, 255);
        let title = renderer.texturize(&font, "Husky <3's Ducky", title_color)?;
        let file_name: &str = &format!("media/sprites/{}", game_data.duck.file_name);
        let texture = texture_manager.load(file_name, &renderer)?;
        let player = Player::new(game_data.duck, texture);
        let game = DuckHuskyWedding {
            input_manager: input_manager,
            title: title,
            renderer: renderer,
            player: player,
        };
        Ok(game)
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
        let title_dims = self.title.dims();
        let title_rectangle = glm::ivec4(0, 0, title_dims.x as i32, title_dims.y as i32);
        self.renderer.clear();
        self.renderer.show(&self.player)?;
        self.renderer.copy(&self.title, Some(title_rectangle), None)?;
        self.renderer.present();
        Ok(())
    }

    fn game_quit(&self) -> bool {
        self.input_manager.game_quit()
    }
}
