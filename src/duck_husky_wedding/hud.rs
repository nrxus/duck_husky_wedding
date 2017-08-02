use errors::*;

use glm;
use moho::errors as moho_errors;
use moho::renderer::{Asset, ColorRGBA, FontTexturizer, Options, Renderer, Texture};

use std::rc::Rc;
use std::time::Duration;
use std::fmt::Display;

pub struct CacheValue<T>(pub T);
pub trait AsCached {
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

impl AsCached for u32 {
    type Value = u32;

    fn as_cached(&self) -> u32 {
        *self
    }
}

pub struct TextCache<T, V> {
    value: CacheValue<V>,
    pub texture: T,
}

impl<T, V: AsCached> TextCache<T, V> {
    pub fn load<'t, F, FT>(
        value: CacheValue<V>,
        font: &F,
        texturizer: &'t FT,
        pattern: &Fn(V::Value) -> String,
    ) -> Result<Self>
    where
        FT: FontTexturizer<'t, F, Texture = T>,
    {
        let text = pattern(value.0.as_cached());
        let color = ColorRGBA(255, 255, 0, 255);
        let texture = texturizer.texturize(font, &text, &color)?;
        Ok(TextCache { value, texture })
    }
}

impl<T, F> TextBox<T, F, Duration> {
    pub fn update(&mut self, elapsed: Duration) {
        self.value = match self.value.checked_sub(elapsed) {
            Some(d) => d,
            None => Duration::default(),
        }
    }
}

impl<T, F> TextBox<T, F, u32> {
    pub fn update(&mut self, delta: i32) {
        if delta >= 0 {
            self.value += delta as u32;
        } else {
            match self.value.checked_sub(delta.abs() as u32) {
                None => self.value = 0,
                Some(v) => self.value = v,
            }
        }
    }
}

pub struct TextBox<T, F, V: AsCached> {
    text: TextCache<T, V>,
    font: Rc<F>,
    pattern: Box<Fn(V::Value) -> String>,
    pub value: V,
}

impl<T, F, V: AsCached + Copy> TextBox<T, F, V> {
    pub fn load<'t, FT>(
        value: V,
        font: Rc<F>,
        texturizer: &'t FT,
        pattern: Box<Fn(V::Value) -> String>,
    ) -> Result<Self>
    where
        FT: FontTexturizer<'t, F, Texture = T>,
    {
        let text = TextCache::load(CacheValue(value), &*font, texturizer, pattern.as_ref())?;
        Ok(TextBox {
            text,
            font,
            value,
            pattern,
        })
    }

    pub fn before_draw<'t, FT>(&mut self, texturizer: &'t FT) -> Result<()>
    where
        FT: FontTexturizer<'t, F, Texture = T>,
    {
        let updated = CacheValue(self.value);
        if self.text.value != updated {
            self.text = TextCache::load(updated, &*self.font, texturizer, self.pattern.as_ref())?;
        }
        Ok(())
    }

    pub fn dims(&self) -> glm::UVec2
    where
        T: Texture,
    {
        self.text.texture.dims()
    }
}

impl<'t, R: Renderer<'t>, F, V: AsCached> Asset<R> for TextBox<R::Texture, F, V> {
    fn draw(&self, options: Options, renderer: &mut R) -> moho_errors::Result<()> {
        renderer.copy(&self.text.texture, options)
    }
}
