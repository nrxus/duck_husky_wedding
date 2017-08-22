use data;
use duck_husky_wedding::button::{self, Button};
use duck_husky_wedding::collectable::{self, Collectable};
use errors::*;

use glm;
use moho::animation::{self, Animation};
use moho::errors as moho_errors;
use moho::input;
use moho::renderer::{options, ColorRGBA, FontDetails, FontLoader, FontManager, FontTexturizer,
                     Renderer, Scene, Texture, TextureLoader, TextureManager};
use moho::shape::Rectangle;

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
        offset: (f64, Alignment),
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
        (offset, alignment): (f64, Alignment),
    ) -> Result<button::Animated<TL::Texture>> {
        let idle = player.idle_texture.load(self)?;
        let animation = player.animation.load(self)?;
        let dims: glm::DVec2 = player.out_size.into();
        let dims = dims * 1.5;
        let offset = match alignment {
            Alignment::Left => offset,
            Alignment::Right => offset - dims.x,
        };
        let top_left = glm::dvec2(640. + offset, 300. - dims.y);
        let body = Rectangle { top_left, dims };
        Ok(button::Animated::new(idle, animation, body))
    }
}

struct ButtonManager<T> {
    selected: Option<super::PlayerKind>,
    duck: button::Animated<T>,
    husky: button::Animated<T>,
}

impl<T> Clone for ButtonManager<T> {
    fn clone(&self) -> Self {
        ButtonManager {
            duck: self.duck.clone(),
            husky: self.husky.clone(),
            selected: self.selected,
        }
    }
}

impl<T> ButtonManager<T> {
    fn load<'t, L>(data: &data::Game, loader: &mut L) -> Result<Self>
    where
        L: ButtonLoader<T>,
    {
        let distance = 50.;
        let husky = loader
            .load(&data.husky, (-distance / 2., Alignment::Right))?;
        let duck = loader.load(&data.duck, (distance / 2., Alignment::Left))?;
        Ok(ButtonManager {
            husky,
            duck,
            selected: None,
        })
    }

    fn update(&mut self, elapsed: Duration, input: &input::State) -> Option<super::PlayerKind> {
        self.husky.animate(elapsed);
        self.duck.animate(elapsed);

        if self.husky.update(input) {
            Some(super::PlayerKind::Husky)
        } else if self.duck.update(input) {
            Some(super::PlayerKind::Duck)
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
        renderer.show(&self.husky)?;
        renderer.show(&self.duck)
    }
}
