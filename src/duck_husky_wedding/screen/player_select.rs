use data;
use duck_husky_wedding::button;
use duck_husky_wedding::font;
use errors::*;

use glm;
use moho::animation::{self, Animation};
use moho::{self, input};
use moho::renderer::{align, options, ColorRGBA, FontTexturizer, Renderer, Scene, Texture,
                     TextureLoader, TextureManager};
use sdl2::keyboard::Keycode;

use std::rc::Rc;
use std::time::Duration;

struct AnimatedData<T> {
    animation: animation::Data<T>,
    dst: options::Destination,
}

struct Animated<T> {
    animation: Animation<T>,
    dst: options::Destination,
}

impl<T> AnimatedData<T> {
    fn start(&self) -> Animated<T> {
        Animated {
            dst: self.dst,
            animation: self.animation.clone().start(),
        }
    }
}

impl<'t, R: Renderer<'t>> Scene<R> for Animated<R::Texture> {
    fn show(&self, renderer: &mut R) -> moho::errors::Result<()> {
        renderer.copy_asset(&self.animation.tile(), options::at(self.dst))
    }
}

impl<T> Animated<T> {
    fn animate(&mut self, delta: Duration) {
        self.animation.animate(delta);
    }
}


pub struct PlayerSelect<T> {
    title: Rc<T>,
    collect_text: Rc<T>,
    avoid_text: Rc<T>,
    button_manager: ButtonManager<T>,
    instructions: Rc<T>,
    gem: Animated<T>,
    coin: Animated<T>,
    cat: Animated<T>,
}

pub struct Data<T> {
    title: Rc<T>,
    collect_text: Rc<T>,
    avoid_text: Rc<T>,
    button_manager: ButtonManager<T>,
    instructions: Rc<T>,
    gem: AnimatedData<T>,
    coin: AnimatedData<T>,
    cat: AnimatedData<T>,
}

impl<T> Data<T> {
    pub fn load<'f, 't, FT, FM, TL>(
        font_manager: &mut FM,
        texturizer: &'t FT,
        texture_manager: &mut TextureManager<'t, TL>,
        data: &data::Game,
        picker: Rc<T>,
    ) -> Result<Self>
    where
        T: Texture,
        TL: TextureLoader<'t, Texture = T>,
        FM: font::Manager,
        FT: FontTexturizer<'t, FM::Font, Texture = T>,
    {
        let font = font_manager.load(font::Kind::KenPixel, 64)?;
        let title_color = ColorRGBA(255, 255, 0, 255);
        let title = texturizer
            .texturize(&*font, "Select Player", &title_color)
            .map(Rc::new)?;
        let button_manager = ButtonManager::load(data, texture_manager, picker)?;
        let collect_text = texturizer
            .texturize(&*font, "Collect", &title_color)
            .map(Rc::new)?;
        let avoid_text = texturizer
            .texturize(&*font, "Avoid", &title_color)
            .map(Rc::new)?;
        let collect_distance = 50;
        let coin = {
            let data = &data.coin;
            let animation = data.animation.load(texture_manager)?;
            let dims: glm::DVec2 = data.out_size.into();
            let dst = align::right(320 - collect_distance / 2)
                .top(525)
                .dims(glm::to_uvec2(dims * 2.));
            AnimatedData { animation, dst }
        };
        let gem = {
            let data = &data.gem;
            let animation = data.animation.load(texture_manager)?;
            let dims: glm::DVec2 = data.out_size.into();
            let dst = align::left(320 + collect_distance / 2)
                .top(525)
                .dims(glm::to_uvec2(dims * 2.));
            AnimatedData { animation, dst }
        };
        let cat = {
            let data = &data.cat;
            let animation = data.idle.load(texture_manager)?;
            let dims: glm::DVec2 = data.out_size.into();
            let dst = align::center(960).top(500).dims(glm::to_uvec2(dims * 2.));
            AnimatedData { animation, dst }
        };
        let instructions = {
            let font = font_manager.load(font::Kind::KenPixel, 32)?;
            texturizer
                .texturize(
                    &*font,
                    "<Use Arrow Keys to choose player; then press Enter>",
                    &title_color,
                )
                .map(Rc::new)
        }?;

        Ok(Data {
            title,
            button_manager,
            collect_text,
            avoid_text,
            coin,
            gem,
            cat,
            instructions,
        })
    }

    pub fn activate(&self) -> PlayerSelect<T> {
        PlayerSelect {
            title: self.title.clone(),
            button_manager: self.button_manager.clone(),
            collect_text: self.collect_text.clone(),
            avoid_text: self.avoid_text.clone(),
            gem: self.gem.start(),
            coin: self.coin.start(),
            cat: self.cat.start(),
            instructions: self.instructions.clone(),
        }
    }
}

impl<T> PlayerSelect<T> {
    pub fn update(&mut self, delta: Duration, input: &input::State) -> Option<super::Kind> {
        let next = self.button_manager.update(delta, input);
        if next.is_none() {
            self.gem.animate(delta);
            self.coin.animate(delta);
            self.cat.animate(delta);
        }
        next.map(super::Kind::GamePlay)
    }
}

impl<'t, R: Renderer<'t>> Scene<R> for PlayerSelect<R::Texture>
where
    R::Texture: Texture,
{
    fn show(&self, renderer: &mut R) -> moho::errors::Result<()> {
        renderer.copy(&self.title, options::at(align::top(50).center(640)))?;
        renderer.show(&self.button_manager)?;
        renderer.copy(&self.avoid_text, options::at(align::top(400).center(960)))?;
        renderer.show(&self.cat)?;
        renderer.copy(&self.collect_text, options::at(align::top(400).center(320)))?;
        renderer.show(&self.coin)?;
        renderer.show(&self.gem)?;
        renderer.copy(
            &self.instructions,
            options::at(align::bottom(720 - self.instructions.dims().y as i32).center(640)),
        )
    }
}

trait ButtonLoader<T> {
    fn load(
        &mut self,
        player: &data::Player,
        alignment: align::Alignment<align::Horizontal>,
    ) -> Result<button::Animated<T>>;
}

impl<'t, TL> ButtonLoader<TL::Texture> for TextureManager<'t, TL>
where
    TL: TextureLoader<'t>,
    TL::Texture: Texture,
{
    fn load(
        &mut self,
        player: &data::Player,
        alignment: align::Alignment<align::Horizontal>,
    ) -> Result<button::Animated<TL::Texture>> {
        let idle = player.idle_texture.load(self)?;
        let animation = player.animation.load(self)?;
        let dims: glm::IVec2 = player.out_size.into();
        let dims = glm::to_uvec2(glm::to_dvec2(dims) * 2.);
        let dst = alignment.bottom(300).dims(dims);
        Ok(button::Animated {
            idle,
            animation,
            dst,
        })
    }
}

struct SelectedButton<T> {
    kind: super::PlayerKind,
    animation: Animation<T>,
}

struct ButtonManager<T> {
    selected: Option<SelectedButton<T>>,
    duck: Button<T>,
    husky: Button<T>,
    picker: Rc<T>,
}

impl<T> Clone for ButtonManager<T> {
    fn clone(&self) -> Self {
        ButtonManager {
            duck: self.duck.clone(),
            husky: self.husky.clone(),
            picker: self.picker.clone(),
            selected: None,
        }
    }
}

impl<T> ButtonManager<T> {
    fn load<L>(data: &data::Game, loader: &mut L, picker: Rc<T>) -> Result<Self>
    where
        L: ButtonLoader<T>,
    {
        let distance = 50;
        let husky = {
            let alignment = align::Alignment {
                pos: 640 - distance / 2,
                align: align::Horizontal::Right,
            };
            Button {
                inner: loader.load(&data.husky, alignment)?,
                kind: super::PlayerKind::Husky,
            }
        };
        let duck = {
            let alignment = align::Alignment {
                pos: 640 + distance / 2,
                align: align::Horizontal::Left,
            };
            Button {
                inner: loader.load(&data.duck, alignment)?,
                kind: super::PlayerKind::Duck,
            }
        };
        Ok(ButtonManager {
            husky,
            duck,
            picker,
            selected: None,
        })
    }

    fn update(&mut self, elapsed: Duration, input: &input::State) -> Option<super::PlayerKind> {
        if let Some(ref mut s) = self.selected {
            s.animation.animate(elapsed);
        }

        let left = input.did_press_key(Keycode::Left);
        let right = input.did_press_key(Keycode::Right);

        if left && !right {
            match self.selected {
                None |
                Some(SelectedButton {
                    kind: super::PlayerKind::Duck,
                    ..
                }) => {
                    self.selected = Some(SelectedButton {
                        kind: super::PlayerKind::Husky,
                        animation: self.husky.animation(),
                    })
                }
                _ => {}
            }
        } else if right && !left {
            match self.selected {
                None |
                Some(SelectedButton {
                    kind: super::PlayerKind::Husky,
                    ..
                }) => {
                    self.selected = Some(SelectedButton {
                        kind: super::PlayerKind::Duck,
                        animation: self.duck.animation(),
                    })
                }
                _ => {}
            }
        }

        if input.did_press_key(Keycode::Return) {
            self.selected.as_ref().map(|s| s.kind)
        } else {
            None
        }
    }
}

struct Button<T> {
    inner: button::Animated<T>,
    kind: super::PlayerKind,
}

impl<T> Clone for Button<T> {
    fn clone(&self) -> Self {
        Button {
            inner: self.inner.clone(),
            kind: self.kind,
        }
    }
}

impl<T> Button<T> {
    fn animation(&self) -> Animation<T> {
        self.inner.animation.clone().start()
    }
}

struct ButtonRenderer<'b, 't, R: 'b + Renderer<'t>>
where
    R::Texture: 'b,
{
    renderer: &'b mut R,
    selected: &'b Option<SelectedButton<R::Texture>>,
    picker: &'b R::Texture,
}

impl<'b, 't, R: Renderer<'t>> ButtonRenderer<'b, 't, R> {
    fn show(&mut self, button: &Button<R::Texture>) -> moho::errors::Result<()> {
        let options = options::at(button.inner.dst);
        match *self.selected {
            Some(ref b) if b.kind == button.kind => {
                let rect = button.inner.dst.rect(|| unreachable!());
                let dst = align::top(rect.y + rect.w + 10).center(rect.x + rect.z / 2);
                self.renderer.copy_asset(&b.animation.tile(), options)?;
                self.renderer.copy(self.picker, options::at(dst))
            }
            _ => self.renderer.copy(&*button.inner.idle, options),
        }
    }
}

impl<'t, R: Renderer<'t>> Scene<R> for ButtonManager<R::Texture>
where
    R::Texture: Texture,
{
    fn show(&self, renderer: &mut R) -> moho::errors::Result<()> {
        let mut renderer = ButtonRenderer {
            renderer: renderer,
            selected: &self.selected,
            picker: &*self.picker,
        };
        renderer.show(&self.husky)?;
        renderer.show(&self.duck)
    }
}
