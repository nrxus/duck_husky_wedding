use errors::*;
use duck_husky_wedding::button;

use glm;
use moho::errors as moho_errors;
use moho::input;
use moho::renderer::{options, ColorRGBA, Font, FontDetails, FontLoader, FontManager,
                     FontTexturizer, Renderer, Scene, Texture};

use std::rc::Rc;
use sdl2::keyboard::Keycode;

pub struct Menu<T> {
    title: Rc<T>,
    button_manager: ButtonManager<T>,
}

pub struct Data<T> {
    title: Rc<T>,
    button_manager: ButtonManager<T>,
}

impl<T> Data<T> {
    pub fn load<'f, 't, FT, FL>(
        font_manager: &mut FontManager<'f, FL>,
        texturizer: &'t FT,
    ) -> Result<Self>
    where
        FL: FontLoader<'f>,
        FL::Font: Font,
        FT: FontTexturizer<'t, FL::Font, Texture = T>,
    {
        let font_details = FontDetails {
            path: "media/fonts/kenpixel_mini.ttf",
            size: 64,
        };
        let font = font_manager.load(&font_details)?;
        let title_color = ColorRGBA(255, 255, 0, 255);
        let title = Rc::new(texturizer
            .texturize(&*font, "Husky Loves Ducky", &title_color)?);
        let button_manager = ButtonManager::load(texturizer, &*font)?;
        Ok(Data {
            title,
            button_manager,
        })
    }

    pub fn activate(&self) -> Menu<T> {
        let title = self.title.clone();
        let button_manager = self.button_manager.clone();

        Menu {
            title,
            button_manager,
        }
    }
}

impl<T> Menu<T> {
    pub fn update(&mut self, input: &input::State) -> Option<super::Kind> {
        self.button_manager.update(input).map(|b| match b {
            ButtonKind::HighScore => super::Kind::HighScore,
            ButtonKind::NewGame => super::Kind::PlayerSelect,
        })
    }
}

impl<'t, R: Renderer<'t>> Scene<R> for Menu<R::Texture>
where
    R::Texture: Texture,
{
    fn show(&self, renderer: &mut R) -> moho_errors::Result<()> {
        renderer.show(&self.button_manager)?;
        let title_dims = glm::to_ivec2(self.title.dims());
        let title_rectangle = glm::ivec4(640 - title_dims.x / 2, 0, title_dims.x, title_dims.y);
        renderer.copy(&self.title, options::at(&title_rectangle))
    }
}

#[derive(Clone, Copy)]
enum ButtonKind {
    NewGame,
    HighScore,
}

struct ButtonManager<T> {
    selected: ButtonKind,
    new_game: button::Static<T>,
    high_score: button::Static<T>,
}

impl<T> Clone for ButtonManager<T> {
    fn clone(&self) -> Self {
        let mut new_game = self.new_game.clone();
        new_game.is_selected = true;

        ButtonManager {
            new_game: new_game,
            selected: self.selected,
            high_score: self.high_score.clone(),
        }
    }
}

impl<T> ButtonManager<T> {
    pub fn load<'t, FT, F>(texturizer: &'t FT, font: &F) -> Result<Self>
    where
        F: Font,
        FT: FontTexturizer<'t, F, Texture = T>,
    {
        let mut new_game =
            button::Static::center_text("New Game", texturizer, font, glm::ivec2(640, 250))?;
        new_game.is_selected = true;

        let high_score =
            button::Static::center_text("High Scores", texturizer, font, glm::ivec2(640, 450))?;
        Ok(ButtonManager {
            new_game,
            high_score,
            selected: ButtonKind::NewGame,
        })
    }

    pub fn update(&mut self, input: &input::State) -> Option<ButtonKind> {
        if input.did_press_key(Keycode::Down) ^ input.did_press_key(Keycode::Up) {
            self.selected = match self.selected {
                ButtonKind::NewGame => {
                    self.new_game.is_selected = false;
                    self.high_score.is_selected = true;
                    ButtonKind::HighScore
                }
                ButtonKind::HighScore => {
                    self.high_score.is_selected = false;
                    self.new_game.is_selected = true;
                    ButtonKind::NewGame
                }
            }
        }

        if input.did_press_key(Keycode::Return) {
            Some(self.selected)
        } else {
            None
        }
    }
}

impl<'t, R: Renderer<'t>> Scene<R> for ButtonManager<R::Texture>
where
    R::Texture: Texture,
{
    fn show(&self, renderer: &mut R) -> moho_errors::Result<()> {
        renderer.show(&self.new_game)?;
        renderer.show(&self.high_score)
    }
}
