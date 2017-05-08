mod menu;

use self::menu::Menu;
use super::game_data::GameData;

use moho::errors as moho_errors;
use moho::input;
use moho::renderer::{FontLoader, FontManager, FontTexturizer};
use moho::renderer::{Renderer, Scene, Show, Texture, TextureLoader, TextureManager};

use errors::*;

use std::time::Duration;

pub enum Kind {
    Menu,
}

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

impl<'s, T> MutScreen<'s, T> {
    pub fn update(&mut self, delta: Duration, input: &input::State) -> Kind {
        match *self {
            MutScreen::Menu(ref mut s) => s.update(delta, input),
        }
    }
}

pub struct Manager<T> {
    menu: Menu<T>,
    active: Kind,
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
        Ok(Manager {
               menu: menu,
               active: Kind::Menu,
           })
    }

    pub fn mut_screen(&mut self) -> MutScreen<T> {
        match self.active {
            Kind::Menu => MutScreen::Menu(&mut self.menu),
        }
    }

    pub fn screen(&self) -> Screen<T> {
        match self.active {
            Kind::Menu => Screen::Menu(&self.menu),
        }
    }

    pub fn select_screen(&mut self, screen: Kind) {
        self.active = screen;
    }
}
