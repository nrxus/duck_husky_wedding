use data;
use errors::*;
use duck_husky_wedding::{button, font};

use glm;
use moho::{self, input};
use moho::renderer::{align, options, ColorRGBA, Font, Renderer, Scene, Texture, TextureLoader,
                     TextureManager};

use std::rc::Rc;
use sdl2::keyboard::Keycode;

pub struct Image<T> {
    texture: Rc<T>,
    dst: options::Destination,
}

impl<T> Clone for Image<T> {
    fn clone(&self) -> Self {
        Image {
            texture: Rc::clone(&self.texture),
            dst: self.dst,
        }
    }
}

pub struct Menu<T> {
    husky: Image<T>,
    duck: Image<T>,
    heart: Image<T>,
    button_manager: ButtonManager<T>,
    instructions: Rc<T>,
}

impl<T> Clone for Menu<T> {
    fn clone(&self) -> Self {
        Menu {
            button_manager: self.button_manager.clone(),
            duck: self.duck.clone(),
            husky: self.husky.clone(),
            heart: self.heart.clone(),
            instructions: Rc::clone(&self.instructions),
        }
    }
}

impl<T> Menu<T> {
    pub fn load<'f, 't, FM, TL>(
        font_manager: &mut FM,
        texture_manager: &mut TextureManager<'t, TL>,
        data: &data::Game,
        picker: Rc<T>,
    ) -> Result<Self>
    where
        T: Texture,
        TL: TextureLoader<'t, Texture = T>,
        FM: font::Manager,
        FM::Font: Font<Texture = T>,
    {
        let button_manager = {
            font_manager
                .load(font::Kind::KenPixel, 64)
                .and_then(|f| ButtonManager::load(f.as_ref(), picker))
        }?;

        let scale = 2;

        let husky = {
            let texture = data.husky.idle_texture.load(texture_manager)?;
            let dims: glm::UVec2 = data.husky.out_size.into();
            let dims = dims * scale;
            let dst = align::right(640 - 32 - 30).middle(125).dims(dims);
            Image { texture, dst }
        };

        let heart = {
            let texture = data.heart.texture.load(texture_manager)?;
            let dims: glm::UVec2 = data.heart.out_size.into();
            let dims = dims * scale;
            let dst = align::center(640).middle(125).dims(dims);
            Image { texture, dst }
        };

        let duck = {
            let texture = data.duck.idle_texture.load(texture_manager)?;
            let dims: glm::UVec2 = data.duck.out_size.into();
            let dims = dims * scale;
            let dst = align::left(640 + 32 + 30).middle(125).dims(dims);
            Image { texture, dst }
        };

        let instructions = {
            let font = font_manager.load(font::Kind::KenPixel, 32)?;
            let color = ColorRGBA(255, 255, 0, 255);
            let text = "<Use Arrow Keys to select option; then press Enter>";
            font.texturize(text, &color).map(Rc::new)
        }?;

        Ok(Menu {
            husky,
            duck,
            heart,
            button_manager,
            instructions,
        })
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
    fn show(&self, renderer: &mut R) -> moho::errors::Result<()> {
        renderer.copy(&*self.husky.texture, options::at(self.husky.dst))?;
        renderer.copy(&*self.heart.texture, options::at(self.heart.dst))?;
        renderer.copy(
            &*self.duck.texture,
            options::at(self.duck.dst).flip(options::Flip::Horizontal),
        )?;
        renderer.copy(
            &self.instructions,
            options::at(align::bottom(720 - self.instructions.dims().y as i32).center(640)),
        )?;
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
            picker: Rc::clone(&self.picker),
        }
    }
}

impl<T> ButtonManager<T> {
    pub fn load<F>(font: &F, picker: Rc<T>) -> Result<Self>
    where
        F: Font<Texture = T>,
    {
        let new_game = {
            let center = glm::ivec2(640, 325);
            let inner = button::Static::with_text("New Game", font)?;
            Button {
                center,
                inner,
                kind: ButtonKind::NewGame,
            }
        };

        let high_score = {
            let center = glm::ivec2(640, 500);
            let inner = button::Static::with_text("High Scores", font)?;
            Button {
                center,
                inner,
                kind: ButtonKind::HighScore,
            }
        };

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
    fn show(&mut self, button: &Button<R::Texture>) -> moho::errors::Result<()>
    where
        R::Texture: Texture,
    {
        let middle = align::middle(button.center.y);

        let texture = if button.kind == self.selected {
            let texture = &*button.inner.selected;
            let right = button.center.x - texture.dims().x as i32 / 2 - 10;
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
    fn show(&self, renderer: &mut R) -> moho::errors::Result<()> {
        let mut renderer = ButtonRenderer {
            renderer,
            selected: self.selected,
            picker: &*self.picker,
        };
        renderer.show(&self.new_game)?;
        renderer.show(&self.high_score)
    }
}
