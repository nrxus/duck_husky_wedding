use errors::*;

use moho;
use moho::renderer::{Asset, ColorRGBA, Font, Options, Renderer};

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
    pub value: CacheValue<V>,
    pub texture: T,
}

impl<T, V: AsCached> TextCache<T, V> {
    pub fn load<F>(value: CacheValue<V>, font: &F, pattern: &Fn(V::Value) -> String) -> Result<Self>
    where
        F: Font<Texture = T>,
    {
        let text = pattern(value.0.as_cached());
        let color = ColorRGBA(255, 255, 0, 255);
        let texture = font.texturize(&text, &color)?;
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

impl<T, F: Font<Texture = T>, V: AsCached + Copy> TextBox<T, F, V> {
    pub fn load(value: V, font: Rc<F>, pattern: Box<Fn(V::Value) -> String>) -> Result<Self> {
        let text = TextCache::load(CacheValue(value), &*font, pattern.as_ref())?;
        Ok(TextBox {
            text,
            font,
            value,
            pattern,
        })
    }

    pub fn before_draw(&mut self) -> Result<()> {
        let updated = CacheValue(self.value);
        if self.text.value != updated {
            self.text = TextCache::load(updated, &*self.font, self.pattern.as_ref())?;
        }
        Ok(())
    }
}

impl<'t, R: Renderer<'t>, F, V: AsCached> Asset<R> for TextBox<R::Texture, F, V> {
    fn draw(&self, options: Options, renderer: &mut R) -> moho::errors::Result<()> {
        renderer.copy(&self.text.texture, options)
    }
}
