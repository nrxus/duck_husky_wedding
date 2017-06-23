use game_data::GameData;
use errors::*;

use moho::input;
use moho::renderer::{Canvas, TextureLoader, TextureManager};

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
        R: Canvas<'t, Texture = TL::Texture>,
        E: input::EventPump,
    {
        let game_data = GameData::load("media/game_data.yaml")?;
        loop {
            let state = self.input_manager.update();
            if state.game_quit() {
                break;
            }
            self.renderer.clear();
            self.renderer.present();
        }
        Ok(())
    }
}
