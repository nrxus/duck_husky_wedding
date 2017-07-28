use errors::*;

use glm;
use moho::errors as moho_errors;
use moho::renderer::{Asset, ColorRGBA, FontDetails, FontLoader, FontManager, FontTexturizer,
                     Options, Renderer, Texture};

use std::rc::Rc;
use std::time::Duration;

struct TextCache<T> {
    secs: u64,
    texture: T,
}

pub struct Timer<T, F> {
    text: TextCache<T>,
    font: Rc<F>,
    pub remaining: Duration,
}

impl<F, T> Timer<T, F> {
    pub fn load<'t, 'f, FT, FL>(
        font_manager: &mut FontManager<'f, FL>,
        texturizer: &'t FT,
    ) -> Result<Self>
    where
        FT: FontTexturizer<'t, F, Texture = T>,
        FL: FontLoader<'f, Font = F>,
    {
        let secs = 100;
        let details = FontDetails {
            path: "media/fonts/kenpixel_mini.ttf",
            size: 32,
        };
        let font = font_manager.load(&details)?;
        let text = {
            let text = format!("Time: {:03}", secs);
            let color = ColorRGBA(255, 255, 0, 255);
            let texture = texturizer.texturize(&*font, &text, &color)?;
            TextCache { secs, texture }
        };
        let remaining = Duration::from_secs(secs);
        Ok(Timer {
            text,
            font,
            remaining,
        })
    }

    pub fn before_draw<'t, FT>(&mut self, texturizer: &'t FT) -> Result<()>
    where
        FT: FontTexturizer<'t, F, Texture = T>,
    {
        let secs = self.remaining.as_secs();
        if secs != self.text.secs {
            self.text.secs = secs;
            let text = format!("Time: {:03}", self.text.secs);
            let color = ColorRGBA(255, 255, 0, 255);
            self.text.texture = texturizer.texturize(&*self.font, &text, &color)?;
        }
        Ok(())
    }

    pub fn update(&mut self, elapsed: Duration) {
        self.remaining = match self.remaining.checked_sub(elapsed) {
            Some(d) => d,
            None => Duration::default(),
        }
    }

    pub fn dims(&self) -> glm::UVec2
    where
        T: Texture,
    {
        self.text.texture.dims()
    }
}

impl<'t, R: Renderer<'t>, F> Asset<R> for Timer<R::Texture, F> {
    fn draw(&self, options: Options, renderer: &mut R) -> moho_errors::Result<()> {
        renderer.copy(&self.text.texture, options)
    }
}
