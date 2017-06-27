use errors::*;

use moho::errors as moho_errors;
use moho::renderer::{ColorRGBA, FontDetails, FontLoader, FontManager, FontTexturizer, Renderer,
                     Asset, Options};

use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

struct TextCache<T> {
    secs: u64,
    texture: T,
}

pub struct Timer<'f, 't, FT>
where
    FT: 't + FontTexturizer<'f, 't>,
{
    text: RefCell<TextCache<FT::Texture>>,
    font: Rc<FT::Font>,
    texturizer: &'t FT,
    remaining: Duration,
}

impl<'f, 't, FT> Timer<'f, 't, FT>
where
    FT: FontTexturizer<'f, 't>,
{
    pub fn load<FL>(font_manager: &mut FontManager<'f, FL>, texturizer: &'t FT) -> Result<Self>
    where
        FL: FontLoader<'f, Font = FT::Font>,
    {
        let secs = 100;
        let details = FontDetails {
            path: "media/fonts/kenpixel_mini.ttf",
            size: 32,
        };
        let font = font_manager.load(&details)?;
        let text = {
            let text = format!("Time: {:03}", 100);
            let color = ColorRGBA(255, 255, 0, 255);
            let texture = texturizer.texturize(&*font, &text, &color)?;
            RefCell::new(TextCache { secs, texture })
        };
        let remaining = Duration::from_secs(secs);
        Ok(
            (Timer {
                 text,
                 font,
                 texturizer,
                 remaining,
             }),
        )
    }

    fn update(&mut self, elapsed: Duration) {
        self.remaining = match self.remaining.checked_sub(elapsed) {
            Some(d) => d,
            None => Duration::default(),
        }
    }
}

impl<'f, 't, R, FT> Asset<R> for Timer<'f, 't, FT>
where
    R: Renderer<'t, Texture = FT::Texture>,
    FT: FontTexturizer<'f, 't>,
{
    fn draw(&self, options: Options, renderer: &mut R) -> moho_errors::Result<()> {
        let mut cache = self.text.borrow_mut();
        let secs = self.remaining.as_secs();
        if secs != cache.secs {
            cache.secs = secs;
            let text = format!("Time: {:03}", cache.secs);
            let color = ColorRGBA(255, 255, 0, 255);
            cache.texture = self.texturizer.texturize(&*self.font, &text, &color)?;
        }
        
        renderer.copy(&cache.texture, options)
    }
}
