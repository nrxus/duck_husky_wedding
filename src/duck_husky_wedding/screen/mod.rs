mod menu;

use self::menu::Menu;
use super::game_data::GameData;

use moho::errors as moho_errors;
use moho::renderer::{FontLoader, FontManager, FontTexturizer, Renderer, Scene, Show, Texture,
                     TextureLoader, TextureManager};

use errors::*;

pub enum Screen<'s, T: 's> {
    Menu(&'s Menu<T>),
}

impl<'s, 't, T, R> Scene<R> for Screen<'s, T>
    where T: Texture,
          R: Renderer<'t, Texture = T> + Show
{
    fn show(&self, renderer: &mut R) -> moho_errors::Result<()> {
        match *self {
            Screen::Menu(s) => renderer.show(s),
        }
    }
}

pub enum MutScreen<'s, T: 's> {
    Menu(&'s mut Menu<T>),
}

pub struct Manager<T> {
    menu: Menu<T>,
}

impl<T> Manager<T> {
    pub fn load<'f, 't, R, TL, FL>(font_manager: &mut FontManager<'f, FL>,
                                   texture_manager: &mut TextureManager<'t, TL>,
                                   texturizer: &'t R,
                                   data: GameData)
                                   -> Result<Self>
        where T: Texture,
              TL: TextureLoader<'t, Texture = T>,
              FL: FontLoader<'f>,
              R: FontTexturizer<'f, 't, Font = FL::Font, Texture = T>
    {
        let menu = Menu::load(font_manager, texture_manager, texturizer, data.duck)?;
        Ok(Manager { menu: menu })
    }

    pub fn mut_screen(&mut self) -> MutScreen<T> {
        MutScreen::Menu(&mut self.menu)
    }

    pub fn screen(&self) -> Screen<T> {
        Screen::Menu(&self.menu)
    }
}
