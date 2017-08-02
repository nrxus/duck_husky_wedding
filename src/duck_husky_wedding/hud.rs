use errors::*;

use glm;
use moho::errors as moho_errors;
use moho::renderer::{Asset, ColorRGBA, FontTexturizer, Options, Renderer, Texture};

use std::rc::Rc;
use std::time::Duration;
use std::fmt::Display;

struct CacheValue<T>(T);
trait AsCached {
    type Value: PartialEq + Display;

    fn as_cached(&self) -> Self::Value;
}

impl<T: AsCached> PartialEq for CacheValue<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0.as_cached() == other.0.as_cached()
    }
}

impl AsCached for Duration {
    type Value = u64;

    fn as_cached(&self) -> u64 {
        self.as_secs()
    }
}

struct TextCache<T, V> {
    value: CacheValue<V>,
    texture: T,
}

impl<T, V: AsCached> TextCache<T, V> {
    fn load<'t, F, FT>(value: CacheValue<V>, font: &F, texturizer: &'t FT) -> Result<Self>
    where
        FT: FontTexturizer<'t, F, Texture = T>,
    {
        let text = format!("Time: {:03}", value.0.as_cached());
        let color = ColorRGBA(255, 255, 0, 255);
        let texture = texturizer.texturize(&*font, &text, &color)?;
        Ok(TextCache { value, texture })
    }
}

pub struct Timer<T, F> {
    text: TextCache<T, Duration>,
    font: Rc<F>,
    pub value: Duration,
}

impl<F, T> Timer<T, F> {
    pub fn load<'t, FT>(font: Rc<F>, texturizer: &'t FT) -> Result<Self>
    where
        FT: FontTexturizer<'t, F, Texture = T>,
    {
        let value = Duration::from_secs(100);
        let text = TextCache::load(CacheValue(value), &*font, texturizer)?;
        Ok(Timer { text, font, value })
    }

    pub fn before_draw<'t, FT>(&mut self, texturizer: &'t FT) -> Result<()>
    where
        FT: FontTexturizer<'t, F, Texture = T>,
    {
        let updated = CacheValue(self.value);
        if self.text.value != updated {
            self.text = TextCache::load(updated, &*self.font, texturizer)?;
        }
        Ok(())
    }

    pub fn update(&mut self, elapsed: Duration) {
        self.value = match self.value.checked_sub(elapsed) {
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
