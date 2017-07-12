use data;
use duck_husky_wedding::button::{self, Button};
use duck_husky_wedding::collectable::{self, Collectable};
use errors::*;

use glm;
use moho::animation::{self, Animation};
use moho::errors as moho_errors;
use moho::input;
use moho::renderer::{options, ColorRGBA, FontDetails, FontManager, FontLoader, FontTexturizer,
                     Renderer, Scene, Texture, TextureManager, TextureLoader};
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
    husky: button::Animated<T>,
    duck: button::Animated<T>,
    cat: Cat<T>,
}

pub struct Data<T> {
    title: Rc<T>,
    collect_text: Rc<T>,
    avoid_text: Rc<T>,
    gem: collectable::Data<T>,
    coin: collectable::Data<T>,
    husky: button::Animated<T>,
    duck: button::Animated<T>,
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
        let button_distance = 50.;

        let husky = {
            let data = &data.husky;
            let idle = data.idle_texture.load(texture_manager)?;
            let animation = data.animation.load(texture_manager)?;
            let dims: glm::DVec2 = data.out_size.into();
            let dims = dims * 1.5;
            let top_left = glm::dvec2(640. - dims.x - button_distance / 2., 300. - dims.y);
            let body = Rectangle { top_left, dims };
            button::Animated::new(idle, animation, body)
        };
        let duck = {
            let data = &data.duck;
            let idle = data.idle_texture.load(texture_manager)?;
            let animation = data.animation.load(texture_manager)?;
            let dims: glm::DVec2 = data.out_size.into();
            let dims = dims * 1.5;
            let top_left = glm::dvec2(640. + button_distance / 2., 300. - dims.y);
            let body = Rectangle { top_left, dims };
            button::Animated::new(idle, animation, body)
        };
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
            husky,
            duck,
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
            duck: self.duck.clone(),
            husky: self.husky.clone(),
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
        if self.husky.update(input) {
            Some(super::Kind::GamePlay(super::PlayerKind::Husky))
        } else if self.duck.update(input) {
            Some(super::Kind::GamePlay(super::PlayerKind::Duck))
        } else {
            self.gem.animate(delta);
            self.coin.animate(delta);
            self.husky.animate(delta);
            self.duck.animate(delta);
            self.cat.animation.animate(delta);
            None
        }
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
        renderer.show(&self.husky)?;
        renderer.show(&self.duck)?;

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
