use data;
use duck_husky_wedding::camera::ViewPort;
use duck_husky_wedding::world;
use errors::*;

use glm;
use moho::input;
use moho::renderer::{Canvas, Renderer, Texture, TextureLoader, TextureManager};
use moho::timer::Timer;
use sdl2::keyboard::Keycode;

pub struct LevelViewer<'t, TL, R, E>
where
    TL: 't + TextureLoader<'t>,
{
    input_manager: input::Manager<E>,
    texture_manager: TextureManager<'t, TL>,
    renderer: R,
}

impl<'t, TL, R, E> LevelViewer<'t, TL, R, E>
where
    TL: TextureLoader<'t>,
{
    pub fn new(renderer: R, texture_loader: &'t TL, input_manager: input::Manager<E>) -> Self {
        let texture_manager = TextureManager::new(texture_loader);
        LevelViewer {
            input_manager: input_manager,
            texture_manager: texture_manager,
            renderer: renderer,
        }
    }

    pub fn run(&mut self) -> Result<()>
    where
        TL::Texture: Texture,
        R: Canvas<'t, Texture = TL::Texture>,
        E: input::EventPump,
    {
        let game_data = data::Game::load("media/game_data.yaml")?;
        let level_data = data::Level::load("media/level.yaml")?;
        let mut world = world::Data::load(&mut self.texture_manager, &level_data, &game_data)?
            .activate(&game_data.duck, &mut self.texture_manager)?;
        let mut viewport = ViewPort::new(glm::ivec2(1280, 720));
        let mut timer = Timer::new();
        loop {
            let game_time = timer.update();
            let input = self.input_manager.update();
            if input.game_quit() {
                break;
            };
            world.update(game_time.since_update);

            let mut t = glm::ivec2(0, 0);
            if input.is_key_down(Keycode::Left) {
                t.x -= 5;
            }
            if input.is_key_down(Keycode::Right) {
                t.x += 5;
            }
            if input.is_key_down(Keycode::Up) {
                t.y -= 5;
            }
            if input.is_key_down(Keycode::Down) {
                t.y += 5;
            }
            if input.is_key_down(Keycode::R) {
                let texture_manager = &mut self.texture_manager;
                let result = data::Level::load("media/level.yaml")
                    .and_then(|l| world::Data::load(texture_manager, &l, &game_data))
                    .and_then(|w| w.activate(&game_data.duck, texture_manager));
                match result {
                    Ok(w) => world = w,
                    Err(err) => println!("error reloading: {:?}", err),
                }
            }

            viewport.translate(t);

            //draw
            self.renderer.clear();
            {
                let mut camera = viewport.camera(&mut self.renderer);
                camera.show(&world)?;
            }
            self.renderer.present();
        }
        Ok(())
    }
}
