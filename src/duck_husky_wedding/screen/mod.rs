mod menu;
mod game_play;
mod high_score;
mod player_select;

use data;
use self::menu::Menu;
use self::game_play::{GamePlay, PlayerKind};
use self::high_score::HighScore;
use self::player_select::PlayerSelect;

use moho::errors as moho_errors;
use moho::input;
use moho::renderer::{FontLoader, FontManager, FontTexturizer};
use moho::renderer::{Renderer, Scene, Texture, TextureLoader, TextureManager};

use errors::*;

use std::time::Duration;

pub enum Kind {
    Menu,
    GamePlay(PlayerKind),
    HighScore,
    PlayerSelect,
}

pub enum Screen<T> {
    Menu(Menu<T>),
    GamePlay(GamePlay<T>),
    HighScore(HighScore<T>),
    PlayerSelect(PlayerSelect<T>),
}

impl<'t, R: Renderer<'t>> Scene<R> for Screen<R::Texture>
where
    R::Texture: Texture,
{
    fn show(&self, renderer: &mut R) -> moho_errors::Result<()> {
        match *self {
            Screen::Menu(ref s) => renderer.show(s),
            Screen::GamePlay(ref s) => renderer.show(s),
            Screen::HighScore(ref s) => renderer.show(s),
            Screen::PlayerSelect(ref s) => renderer.show(s),
        }
    }
}

impl<T> Screen<T> {
    pub fn update(&mut self, delta: Duration, input: &input::State) -> Option<Kind> {
        match *self {
            Screen::Menu(ref mut s) => s.update(input),
            Screen::GamePlay(ref mut s) => s.update(delta, input),
            Screen::HighScore(ref mut s) => s.update(input),
            Screen::PlayerSelect(ref mut s) => s.update(delta, input),
        }
    }
}

pub struct Manager<T> {
    menu: menu::Data<T>,
    game_play: game_play::Data<T>,
    high_score: high_score::Data<T>,
    player_select: player_select::Data<T>,
    //kind of current screen
    active: Screen<T>,
}

impl<T: Texture> Manager<T> {
    pub fn load<'f, 't, R, TL, FL>(
        font_manager: &mut FontManager<'f, FL>,
        texture_manager: &mut TextureManager<'t, TL>,
        texturizer: &'t R,
        data: data::Game,
    ) -> Result<Self>
    where
        TL: TextureLoader<'t, Texture = T>,
        FL: FontLoader<'f>,
        R: FontTexturizer<'f, 't, Font = FL::Font, Texture = T>,
    {
        let player_select =
            player_select::Data::load(font_manager, texturizer, texture_manager, &data)?;
        let menu = menu::Data::load(font_manager, texturizer)?;
        let active = Screen::Menu(menu.activate());
        let game_play = game_play::Data::load(texture_manager, data)?;
        let high_score = high_score::Data::load(font_manager, texturizer)?;
        Ok(Manager {
            menu: menu,
            game_play: game_play,
            high_score: high_score,
            player_select: player_select,
            active: active,
        })
    }

    pub fn mut_screen(&mut self) -> &mut Screen<T> {
        &mut self.active
    }

    pub fn screen(&self) -> &Screen<T> {
        &self.active
    }

    pub fn select_screen<'f, 't, R, FL, TL>(
        &mut self,
        screen: Kind,
        font_manager: &mut FontManager<'f, FL>,
        texture_manager: &mut TextureManager<'t, TL>,
        texturizer: &'t R,
    ) where
        FL: FontLoader<'f>,
        TL: TextureLoader<'t, Texture = T>,
        R: FontTexturizer<'f, 't, Font = FL::Font, Texture = T>,
    {
        self.active = match screen {
            Kind::Menu => Screen::Menu(self.menu.activate()),
            Kind::PlayerSelect => Screen::PlayerSelect(self.player_select.activate()),
            Kind::GamePlay(k) => {
                Screen::GamePlay(self.game_play.activate(texture_manager, k).unwrap())
            }
            Kind::HighScore => {
                Screen::HighScore(self.high_score.activate(font_manager, texturizer).unwrap())
            }
        }
    }
}
