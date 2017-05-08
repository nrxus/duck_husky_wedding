mod menu;
mod game_play;

use self::menu::Menu;
use self::game_play::GamePlay;
use super::game_data::GameData;

use moho::errors as moho_errors;
use moho::input;
use moho::renderer::{FontLoader, FontManager, FontTexturizer};
use moho::renderer::{Renderer, Scene, Show, Texture, TextureLoader, TextureManager};

use errors::*;

use std::time::Duration;

pub enum Kind {
    Menu,
    GamePlay,
}

pub enum Screen<'s, T: 's> {
    Menu(&'s Menu<T>),
    GamePlay(&'s GamePlay<T>),
}

impl<'s, 't, T, R> Scene<R> for Screen<'s, T>
    where T: Texture,
          R: Renderer<'t, Texture = T> + Show
{
    fn show(&self, renderer: &mut R) -> moho_errors::Result<()> {
        match *self {
            Screen::Menu(s) => renderer.show(s),
            Screen::GamePlay(s) => renderer.show(s),
        }
    }
}

pub enum MutScreen<'s, T: 's> {
    Menu(&'s mut Menu<T>),
    GamePlay(&'s mut GamePlay<T>),
}

impl<'s, T> MutScreen<'s, T> {
    pub fn update(&mut self, delta: Duration, input: &input::State) -> Kind {
        match *self {
            MutScreen::Menu(ref mut s) => s.update(input),
            MutScreen::GamePlay(ref mut s) => s.update(delta, input),
        }
    }
}

pub struct Manager<T> {
    menu: Menu<T>,
    game_play: GamePlay<T>,
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
        let menu = Menu::load(font_manager, texturizer)?;
        let game_play = GamePlay::load(texture_manager, data.duck)?;
        Ok(Manager {
               menu: menu,
               game_play: game_play,
               active: Kind::Menu,
           })
    }

    pub fn mut_screen(&mut self) -> MutScreen<T> {
        match self.active {
            Kind::Menu => MutScreen::Menu(&mut self.menu),
            Kind::GamePlay => MutScreen::GamePlay(&mut self.game_play),
        }
    }

    pub fn screen(&self) -> Screen<T> {
        match self.active {
            Kind::Menu => Screen::Menu(&self.menu),
            Kind::GamePlay => Screen::GamePlay(&self.game_play),
        }
    }

    pub fn select_screen(&mut self, screen: Kind) {
        self.active = screen;
    }
}
