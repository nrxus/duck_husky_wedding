mod menu;
mod game_play;
mod high_score;
mod player_select;

use self::menu::Menu;
use self::game_play::GamePlay;
use self::high_score::HighScore;
use self::player_select::PlayerSelect;
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
    HighScore,
    PlayerSelect,
}

pub enum RefScreen<'s, T: 's> {
    Menu(&'s Menu<T>),
    GamePlay(&'s GamePlay<T>),
    HighScore(&'s HighScore<T>),
    PlayerSelect(&'s PlayerSelect<T>),
}

impl<'s, 't, T, R> Scene<R> for RefScreen<'s, T>
    where T: Texture,
          R: Renderer<'t, Texture = T> + Show
{
    fn show(&self, renderer: &mut R) -> moho_errors::Result<()> {
        match *self {
            RefScreen::Menu(s) => renderer.show(s),
            RefScreen::GamePlay(s) => renderer.show(s),
            RefScreen::HighScore(s) => renderer.show(s),
            RefScreen::PlayerSelect(s) => renderer.show(s),
        }
    }
}

pub enum MutScreen<'s, T: 's> {
    Menu(&'s mut Menu<T>),
    GamePlay(&'s mut GamePlay<T>),
    HighScore(&'s mut HighScore<T>),
    PlayerSelect(&'s mut PlayerSelect<T>),
}

impl<'s, T> MutScreen<'s, T> {
    pub fn update(&mut self, delta: Duration, input: &input::State) -> Option<Kind> {
        match *self {
            MutScreen::Menu(ref mut s) => s.update(input),
            MutScreen::GamePlay(ref mut s) => s.update(delta, input),
            MutScreen::HighScore(ref mut s) => s.update(input),
            MutScreen::PlayerSelect(ref mut s) => s.update(delta, input),
        }
    }
}

pub struct Manager<T> {
    menu: Menu<T>,
    game_play: GamePlay<T>,
    high_score: HighScore<T>,
    player_select: PlayerSelect<T>,
    //kind of current screen
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
        let player_select = PlayerSelect::load(font_manager, texturizer, texture_manager, &data)?;
        let menu = Menu::load(font_manager, texturizer)?;
        let game_play = GamePlay::load(texture_manager, data)?;
        let high_score = HighScore::load(font_manager, texturizer)?;
        Ok(Manager {
               menu: menu,
               game_play: game_play,
               high_score: high_score,
               player_select: player_select,
               active: Kind::Menu,
           })
    }

    pub fn mut_screen(&mut self) -> MutScreen<T> {
        match self.active {
            Kind::Menu => MutScreen::Menu(&mut self.menu),
            Kind::GamePlay => MutScreen::GamePlay(&mut self.game_play),
            Kind::HighScore => MutScreen::HighScore(&mut self.high_score),
            Kind::PlayerSelect => MutScreen::PlayerSelect(&mut self.player_select),
        }
    }

    pub fn screen(&self) -> RefScreen<T> {
        match self.active {
            Kind::Menu => RefScreen::Menu(&self.menu),
            Kind::GamePlay => RefScreen::GamePlay(&self.game_play),
            Kind::HighScore => RefScreen::HighScore(&self.high_score),
            Kind::PlayerSelect => RefScreen::PlayerSelect(&self.player_select),
        }
    }

    pub fn select_screen<'f, 't, R, FL>(&mut self,
                                        screen: Kind,
                                        font_manager: &mut FontManager<'f, FL>,
                                        texturizer: &'t R)
        where T: Texture,
              FL: FontLoader<'f>,
              R: FontTexturizer<'f, 't, Font = FL::Font, Texture = T>
    {
        if let Kind::HighScore = screen {
            self.high_score
                .load_scores(font_manager, texturizer)
                .unwrap();
        }
        self.active = screen;
    }
}
