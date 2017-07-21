use errors::*;

use glm;
use moho::errors as moho_errors;
use moho::renderer::{ColorRGBA, FontDetails, FontLoader, FontManager, FontTexturizer, Renderer,
                     Texture, Asset, Options};

use std::rc::Rc;

struct TextCache<T> {
    value: u64,
    texture: T,
}

pub struct Score<T, F> {
    text: TextCache<T>,
    font: Rc<F>,
    pub value: u64,
}

impl<F, T> Score<T, F> {
    pub fn load<'t, 'f, FT, FL>(
        font_manager: &mut FontManager<'f, FL>,
        texturizer: &'t FT,
    ) -> Result<Self>
    where
        FT: FontTexturizer<'t, F, Texture = T>,
        FL: FontLoader<'f, Font = F>,
    {
        let value = 0;
        let details = FontDetails {
            path: "media/fonts/kenpixel_mini.ttf",
            size: 32,
        };
        let font = font_manager.load(&details)?;
        let text = {
            let text = format!("Score: {:03}", value);
            let color = ColorRGBA(255, 255, 0, 255);
            let texture = texturizer.texturize(&*font, &text, &color)?;
            TextCache { value, texture }
        };
        Ok(Score { text, font, value })
    }

    pub fn before_draw<'t, FT>(&mut self, texturizer: &'t FT) -> Result<()>
    where
        FT: FontTexturizer<'t, F, Texture = T>,
    {
        if self.value != self.text.value {
            self.text.value = self.value;
            let text = format!("Score: {:03}", self.text.value);
            let color = ColorRGBA(255, 255, 0, 255);
            self.text.texture = texturizer.texturize(&*self.font, &text, &color)?;
        }
        Ok(())
    }

    pub fn update(&mut self, delta: i32) {
        if delta >= 0 {
            self.value += delta as u64;
        } else {
            match self.value.checked_sub(delta.abs() as u64) {
                None => self.value = 0,
                Some(v) => self.value = v,
            }
        }
    }

    pub fn dims(&self) -> glm::UVec2
    where
        T: Texture,
    {
        self.text.texture.dims()
    }
}

impl<'t, R: Renderer<'t>, F> Asset<R> for Score<R::Texture, F> {
    fn draw(&self, options: Options, renderer: &mut R) -> moho_errors::Result<()> {
        renderer.copy(&self.text.texture, options)
    }
}
