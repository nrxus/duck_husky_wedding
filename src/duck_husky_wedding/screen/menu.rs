use errors::*;
use duck_husky_wedding::{button, font};

use glm;
use moho::errors as moho_errors;
use moho::input;
use moho::renderer::{align, options, ColorRGBA, Font, FontTexturizer, Renderer, Scene, Texture};

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
    pub fn load<'f, 't, FT, FM>(font_manager: &mut FM, texturizer: &'t FT) -> Result<Self>
    where
        FM: font::Manager,
        FM::Font: Font,
        FT: FontTexturizer<'t, FM::Font, Texture = T>,
    {
        let font = font_manager.load(font::Kind::KenPixel, 64)?;
        let title_color = ColorRGBA(255, 255, 0, 255);
        let title = texturizer
            .texturize(&*font, "Husky Loves Ducky", &title_color)
            .map(Rc::new)?;
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
        renderer.copy(&self.title, options::at(align::top(0).center(640)))?;
        renderer.show(&self.button_manager)
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum ButtonKind {
    NewGame,
    HighScore,
}

struct Button<T> {
    kind: ButtonKind,
    inner: button::Static<T>,
    center: glm::IVec2,
}

impl<T> Clone for Button<T> {
    fn clone(&self) -> Self {
        Button {
            kind: self.kind,
            center: self.center,
            inner: self.inner.clone(),
        }
    }
}

struct ButtonManager<T> {
    selected: ButtonKind,
    new_game: Button<T>,
    high_score: Button<T>,
    picker: Rc<T>,
}

impl<T> Clone for ButtonManager<T> {
    fn clone(&self) -> Self {
        ButtonManager {
            selected: self.selected,
            new_game: self.new_game.clone(),
            high_score: self.high_score.clone(),
            picker: self.picker.clone(),
        }
    }
}

impl<T> ButtonManager<T> {
    pub fn load<'t, FT, F>(texturizer: &'t FT, font: &F) -> Result<Self>
    where
        F: Font,
        FT: FontTexturizer<'t, F, Texture = T>,
    {
        let new_game = {
            let center = glm::ivec2(640, 250);
            let inner = button::Static::with_text("New Game", texturizer, font)?;
            Button {
                center,
                inner,
                kind: ButtonKind::NewGame,
            }
        };

        let high_score = {
            let center = glm::ivec2(640, 450);
            let inner = button::Static::with_text("High Scores", texturizer, font)?;
            Button {
                center,
                inner,
                kind: ButtonKind::HighScore,
            }
        };

        let picker = texturizer
            .texturize(font, "->", &ColorRGBA(255, 255, 0, 255))
            .map(Rc::new)?;

        Ok(ButtonManager {
            new_game,
            high_score,
            picker,
            selected: ButtonKind::NewGame,
        })
    }

    pub fn update(&mut self, input: &input::State) -> Option<ButtonKind> {
        if input.did_press_key(Keycode::Down) ^ input.did_press_key(Keycode::Up) {
            self.selected = match self.selected {
                ButtonKind::NewGame => ButtonKind::HighScore,
                ButtonKind::HighScore => ButtonKind::NewGame,
            }
        }

        if input.did_press_key(Keycode::Return) {
            Some(self.selected)
        } else {
            None
        }
    }
}

struct ButtonRenderer<'r, 't, R: 'r + Renderer<'t>>
where
    R::Texture: 'r,
{
    renderer: &'r mut R,
    selected: ButtonKind,
    picker: &'r R::Texture,
}

impl<'r, 't, R: Renderer<'t>> ButtonRenderer<'r, 't, R> {
    fn show(&mut self, button: &Button<R::Texture>) -> moho_errors::Result<()>
    where
        R::Texture: Texture,
    {
        let middle = align::middle(button.center.y);

        let texture = if button.kind == self.selected {
            let texture = &*button.inner.selected;
            let right = button.center.x - texture.dims().x as i32 / 2 - 5;
            self.renderer
                .copy(self.picker, options::at(middle.right(right)))?;
            texture
        } else {
            &*button.inner.idle
        };

        self.renderer
            .copy(texture, options::at(middle.center(button.center.x)))
    }
}

impl<'t, R: Renderer<'t>> Scene<R> for ButtonManager<R::Texture>
where
    R::Texture: Texture,
{
    fn show(&self, renderer: &mut R) -> moho_errors::Result<()> {
        let mut renderer = ButtonRenderer {
            renderer,
            selected: self.selected,
            picker: &*self.picker,
        };
        renderer.show(&self.new_game)?;
        renderer.show(&self.high_score)
    }
}
