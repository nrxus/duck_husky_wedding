use data;
use duck_husky_wedding::button;
use duck_husky_wedding::collectable::{self, Collectable};
use errors::*;

use glm;
use moho::animation::{self, Animation};
use moho::errors as moho_errors;
use moho::input;
use moho::renderer::{options, ColorRGBA, FontDetails, FontLoader, FontManager, FontTexturizer,
                     Renderer, Scene, Texture, TextureLoader, TextureManager};
use sdl2::keyboard::Keycode;

use std::rc::Rc;
use std::time::Duration;

struct CatData<T> {
    animation: animation::Data<T>,
    dst: glm::IVec4,
}

struct Cat<T> {
    animation: Animation<T>,
    dst: glm::IVec4,
}

impl<'t, R: Renderer<'t>> Scene<R> for Cat<R::Texture> {
    fn show(&self, renderer: &mut R) -> moho_errors::Result<()> {
        renderer.copy_asset(&self.animation.tile(), options::at(&self.dst))
    }
}


pub struct PlayerSelect<T> {
    title: Rc<T>,
    collect_text: Rc<T>,
    avoid_text: Rc<T>,
    gem: Collectable<T>,
    coin: Collectable<T>,
    button_manager: ButtonManager<T>,
    cat: Cat<T>,
}

pub struct Data<T> {
    title: Rc<T>,
    collect_text: Rc<T>,
    avoid_text: Rc<T>,
    gem: collectable::Data<T>,
    coin: collectable::Data<T>,
    button_manager: ButtonManager<T>,
    cat: CatData<T>,
}

impl<T> Data<T> {
    pub fn load<'f, 't, FT, FL, TL>(
        font_manager: &mut FontManager<'f, FL>,
        texturizer: &'t FT,
        texture_manager: &mut TextureManager<'t, TL>,
        data: &data::Game,
    ) -> Result<Self>
    where
        T: Texture,
        TL: TextureLoader<'t, Texture = T>,
        FL: FontLoader<'f>,
        FT: FontTexturizer<'t, FL::Font, Texture = T>,
    {
        let font_details = FontDetails {
            path: "media/fonts/kenpixel_mini.ttf",
            size: 64,
        };
        let font = font_manager.load(&font_details)?;
        let title_color = ColorRGBA(255, 255, 0, 255);
        let title = Rc::new(texturizer.texturize(&*font, "Select Player", &title_color)?);
        let button_manager = ButtonManager::load(data, texture_manager)?;
        let collect_text = Rc::new(texturizer.texturize(&*font, "Collect", &title_color)?);
        let avoid_text = Rc::new(texturizer.texturize(&*font, "Avoid", &title_color)?);
        let collect_distance = 50;
        let coin = collectable::Data::load(
            glm::ivec2(
                320 - collect_distance / 2 - data.coin.out_size.x as i32,
                550,
            ),
            &data.coin,
            texture_manager,
        )?;
        let gem = collectable::Data::load(
            glm::ivec2(320 + collect_distance / 2, 550),
            &data.gem,
            texture_manager,
        )?;
        let cat = {
            let data = &data.cat;
            let animation = data.idle.load(texture_manager)?;
            let dims: glm::DVec2 = data.out_size.into();
            let dims = glm::to_ivec2(dims * 1.5);
            let dst = glm::ivec4(960 - dims.x / 2, 500, dims.x, dims.y);
            CatData { animation, dst }
        };

        Ok(Data {
            title,
            button_manager,
            collect_text,
            avoid_text,
            coin,
            gem,
            cat,
        })
    }

    pub fn activate(&self) -> PlayerSelect<T> {
        PlayerSelect {
            title: self.title.clone(),
            button_manager: self.button_manager.clone(),
            collect_text: self.collect_text.clone(),
            avoid_text: self.avoid_text.clone(),
            gem: Collectable::new(&self.gem),
            coin: Collectable::new(&self.coin),
            cat: Cat {
                dst: self.cat.dst,
                animation: self.cat.animation.clone().start(),
            },
        }
    }
}

impl<T> PlayerSelect<T> {
    pub fn update(&mut self, delta: Duration, input: &input::State) -> Option<super::Kind> {
        let next = self.button_manager.update(delta, input);
        if let None = next {
            self.gem.animate(delta);
            self.coin.animate(delta);
            self.cat.animation.animate(delta);
        }
        next.map(|k| super::Kind::GamePlay(k))
    }
}

impl<'t, R: Renderer<'t>> Scene<R> for PlayerSelect<R::Texture>
where
    R::Texture: Texture,
{
    fn show(&self, renderer: &mut R) -> moho_errors::Result<()> {

        //title+buttons
        {
            let dims = glm::to_ivec2(self.title.dims());
            let dst = glm::ivec4(640 - dims.x / 2, 50, dims.x, dims.y);
            renderer.copy(&self.title, options::at(&dst))
        }?;
        renderer.show(&self.button_manager)?;

        //avoid
        {
            let dims = glm::to_ivec2(self.avoid_text.dims());
            let dst = glm::ivec4(960 - dims.x / 2, 400, dims.x, dims.y);
            renderer.copy(&self.avoid_text, options::at(&dst))
        }?;
        renderer.show(&self.cat)?;

        //collect
        {
            let dims = glm::to_ivec2(self.collect_text.dims());
            let dst = glm::ivec4(320 - dims.x / 2, 400, dims.x, dims.y);
            renderer.copy(&self.collect_text, options::at(&dst))
        }?;
        renderer.show(&self.coin)?;
        renderer.show(&self.gem)
    }
}

enum Alignment {
    Left,
    Right,
}

trait ButtonLoader<T> {
    fn load(
        &mut self,
        player: &data::Player,
        offset: (i32, Alignment),
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
        (offset, alignment): (i32, Alignment),
    ) -> Result<button::Animated<TL::Texture>> {
        let idle = player.idle_texture.load(self)?;
        let animation = player.animation.load(self)?;
        let dims: glm::IVec2 = player.out_size.into();
        let dims = glm::to_ivec2(glm::to_dvec2(dims) * 1.5);
        let offset = match alignment {
            Alignment::Left => offset,
            Alignment::Right => offset - dims.x,
        };
        let dst = glm::ivec4(640 + offset, 300 - dims.y, dims.x, dims.y);
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
}

impl<T> Clone for ButtonManager<T> {
    fn clone(&self) -> Self {
        ButtonManager {
            duck: self.duck.clone(),
            husky: self.husky.clone(),
            selected: None,
        }
    }
}

impl<T> ButtonManager<T> {
    fn load<'t, L>(data: &data::Game, loader: &mut L) -> Result<Self>
    where
        L: ButtonLoader<T>,
    {
        let distance = 50;
        let husky = {
            Button {
                inner: loader.load(&data.husky, (-distance / 2, Alignment::Right))?,
                kind: super::PlayerKind::Husky,
            }
        };
        let duck = {
            Button {
                inner: loader.load(&data.duck, (distance / 2, Alignment::Left))?,
                kind: super::PlayerKind::Duck,
            }
        };
        Ok(ButtonManager {
            husky,
            duck,
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
}

impl<'b, 't, R: Renderer<'t>> ButtonRenderer<'b, 't, R> {
    fn show(&mut self, button: &Button<R::Texture>) -> moho_errors::Result<()> {
        match *self.selected {
            Some(ref b) if b.kind == button.kind => self.renderer
                .copy_asset(&b.animation.tile(), options::at(&button.inner.dst)),
            _ => self.renderer
                .copy(&*button.inner.idle, options::at(&button.inner.dst)),
        }
    }
}

impl<'t, R: Renderer<'t>> Scene<R> for ButtonManager<R::Texture>
where
    R::Texture: Texture,
{
    fn show(&self, renderer: &mut R) -> moho_errors::Result<()> {
        let mut renderer = ButtonRenderer {
            renderer: renderer,
            selected: &self.selected,
        };
        renderer.show(&self.husky)?;
        renderer.show(&self.duck)
    }
}
