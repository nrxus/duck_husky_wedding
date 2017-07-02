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
use moho::renderer::{Font, FontLoader, FontManager, FontTexturizer};
use moho::renderer::{Renderer, Scene, Texture, TextureLoader, TextureManager};

use errors::*;

use std::time::Duration;

pub enum Kind {
    Menu,
    GamePlay(PlayerKind),
    HighScore,
    PlayerSelect,
}

pub enum Screen<T, F> {
    Menu(Menu<T>),
    GamePlay(GamePlay<T, F>),
    HighScore(HighScore<T>),
    PlayerSelect(PlayerSelect<T>),
}

impl<'t, R: Renderer<'t>, F> Scene<R> for Screen<R::Texture, F>
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

impl<T, F> Screen<T, F> {
    pub fn update(&mut self, delta: Duration, input: &input::State) -> Option<Kind> {
        match *self {
            Screen::Menu(ref mut s) => s.update(input),
            Screen::GamePlay(ref mut s) => s.update(delta, input),
            Screen::HighScore(ref mut s) => s.update(input),
            Screen::PlayerSelect(ref mut s) => s.update(delta, input),
        }
    }

    pub fn before_draw<'t, FT>(&mut self, texturizer: &'t FT) -> Result<()>
    where
        T: Texture,
        FT: FontTexturizer<'t, F, Texture = T>,
    {
        if let Screen::GamePlay(ref mut s) = *self {
            s.before_draw(texturizer)
        } else {
            Ok(())
        }
    }
}

pub struct Manager<T, F> {
    menu: menu::Data<T>,
    game_play: game_play::Data<T>,
    high_score: high_score::Data<T>,
    player_select: player_select::Data<T>,
    //kind of current screen
    active: Screen<T, F>,
}

impl<T, F> Manager<T, F> {
    pub fn load<'f, 't, R, TL, FL>(
        font_manager: &mut FontManager<'f, FL>,
        texture_manager: &mut TextureManager<'t, TL>,
        texturizer: &'t R,
        level: &data::Level,
        game: data::Game,
    ) -> Result<Self>
    where
        T: Texture,
        F: Font,
        TL: TextureLoader<'t, Texture = T>,
        FL: FontLoader<'f, Font = F>,
        R: FontTexturizer<'t, F, Texture = T>,
    {
        let player_select =
            player_select::Data::load(font_manager, texturizer, texture_manager, &game)?;
        let menu = menu::Data::load(font_manager, texturizer)?;
        let active = Screen::Menu(menu.activate());
        let game_play = game_play::Data::load(texture_manager, level, game)?;
        let high_score = high_score::Data::load(font_manager, texturizer)?;
        Ok(Manager {
            menu: menu,
            game_play: game_play,
            high_score: high_score,
            player_select: player_select,
            active: active,
        })
    }

    pub fn mut_screen(&mut self) -> &mut Screen<T, F> {
        &mut self.active
    }

    pub fn screen(&self) -> &Screen<T, F> {
        &self.active
    }

    pub fn select_screen<'f, 't, FT, FL, TL>(
        &mut self,
        screen: Kind,
        font_manager: &mut FontManager<'f, FL>,
        texture_manager: &mut TextureManager<'t, TL>,
        texturizer: &'t FT,
    ) where
        T: Texture,
        F: Font,
        FL: FontLoader<'f, Font = F>,
        TL: TextureLoader<'t, Texture = T>,
        FT: FontTexturizer<'t, F, Texture = T>,
    {
        self.active = match screen {
            Kind::Menu => Screen::Menu(self.menu.activate()),
            Kind::PlayerSelect => Screen::PlayerSelect(self.player_select.activate()),
            Kind::GamePlay(k) => {
                Screen::GamePlay(
                    self.game_play
                        .activate(texture_manager, font_manager, texturizer, k)
                        .unwrap(),
                )
            }
            Kind::HighScore => {
                Screen::HighScore(self.high_score.activate(font_manager, texturizer).unwrap())
            }
        }
    }
}
