mod menu;
mod finish;
mod game_play;
mod high_score;
mod player_select;

use data;
use self::menu::Menu;
use self::game_play::{GamePlay, PlayerKind};
use self::high_score::HighScore;
use self::player_select::PlayerSelect;

use moho::{self, input};
use moho::renderer::{Canvas, Font, FontLoader, FontManager, Scene, Texture, TextureLoader,
                     TextureManager};

use errors::*;

use std::rc::Rc;
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

impl<'t, R: Canvas<'t>, F> Scene<R> for Screen<R::Texture, F>
where
    R::Texture: Texture,
{
    fn show(&self, renderer: &mut R) -> moho::errors::Result<()> {
        match *self {
            Screen::Menu(ref s) => renderer.show(s),
            Screen::GamePlay(ref s) => renderer.show(s),
            Screen::HighScore(ref s) => renderer.show(s),
            Screen::PlayerSelect(ref s) => renderer.show(s),
        }
    }
}

impl<T: Texture, F: Font<Texture = T>> Screen<T, F> {
    pub fn update(&mut self, delta: Duration, input: &input::State) -> Option<Kind> {
        match *self {
            Screen::Menu(ref mut s) => s.update(input),
            Screen::GamePlay(ref mut s) => s.update(delta, input),
            Screen::HighScore(ref mut s) => s.update(input),
            Screen::PlayerSelect(ref mut s) => s.update(delta, input),
        }
    }

    pub fn before_draw(&mut self) -> Result<()> {
        if let Screen::GamePlay(ref mut s) = *self {
            s.before_draw()
        } else {
            Ok(())
        }
    }
}

pub struct Manager<T, F> {
    menu: Menu<T>,
    game_play: game_play::Data<T>,
    high_score: high_score::Data<T>,
    player_select: player_select::Data<T>,
    //kind of current screen
    active: Screen<T, F>,
}

impl<T, F: Font<Texture = T>> Manager<T, F> {
    pub fn load<'f, 't, TL, FL>(
        font_manager: &mut FontManager<'f, FL>,
        texture_manager: &mut TextureManager<'t, TL>,
        level: &data::Level,
        game: data::Game,
    ) -> Result<Self>
    where
        T: Texture,
        TL: TextureLoader<'t, Texture = T>,
        FL: FontLoader<'f, Font = F>,
    {
        let picker = game.heart.texture.load(texture_manager)?;
        let player_select =
            player_select::Data::load(font_manager, texture_manager, &game, Rc::clone(&picker))?;
        let menu = Menu::load(font_manager, texture_manager, &game, picker)?;
        let active = Screen::Menu(menu.clone());
        let game_play = game_play::Data::load(texture_manager, level, game)?;
        let high_score = high_score::Data::load(font_manager)?;
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

    pub fn select_screen<'f, 't, FL, TL>(
        &mut self,
        screen: Kind,
        font_manager: &mut FontManager<'f, FL>,
        texture_manager: &mut TextureManager<'t, TL>,
    ) where
        T: Texture,
        FL: FontLoader<'f, Font = F>,
        TL: TextureLoader<'t, Texture = T>,
    {
        self.active = match screen {
            Kind::Menu => Screen::Menu(self.menu.clone()),
            Kind::PlayerSelect => Screen::PlayerSelect(self.player_select.activate()),
            Kind::GamePlay(k) => Screen::GamePlay(
                self.game_play
                    .activate(texture_manager, font_manager, k)
                    .unwrap(),
            ),
            Kind::HighScore => Screen::HighScore(self.high_score.activate(font_manager).unwrap()),
        }
    }
}
