use duck_husky_wedding::hud::{AsCached, CacheValue, TextCache};
use duck_husky_wedding::flicker::Flicker;
use errors::*;
use utils::Try;

use glm;
use moho::{self, input};
use moho::renderer::{options, ColorRGBA, Font, Renderer, Scene, Texture};
use sdl2::keyboard::Keycode;

use std::cmp;
use std::rc::Rc;
use std::time::Duration;

impl AsCached for Option<char> {
    type Value = char;

    fn as_cached(&self) -> char {
        match *self {
            None => '_',
            Some(c) => c,
        }
    }
}

pub struct EditText<T, F> {
    textures: [TextCache<T, Option<char>>; 6],
    values: [Option<char>; 6],
    active: usize,
    flicker: Flicker,
    font: Rc<F>,
    label: T,
    tl: glm::IVec2,
}

impl<T, F: Font<Texture = T>> EditText<T, F> {
    pub fn load(label: &str, tl: glm::IVec2, font: Rc<F>) -> Result<Self> {
        let textures = {
            let font = &*font;
            [
                Self::load_char(CacheValue(None), font)?,
                Self::load_char(CacheValue(None), font)?,
                Self::load_char(CacheValue(None), font)?,
                Self::load_char(CacheValue(None), font)?,
                Self::load_char(CacheValue(None), font)?,
                Self::load_char(CacheValue(None), font)?,
            ]
        };
        let values = [None; 6];
        let label = font.texturize(label, &ColorRGBA(255, 255, 255, 255))?;
        Ok(EditText {
            label,
            tl,
            textures,
            values,
            font,
            active: 0,
            flicker: Flicker::new(Duration::from_millis(400)),
        })
    }

    pub fn update(&mut self, elapsed: Duration, input: &input::State) {
        if input.did_press_key(Keycode::Left) {
            self.move_left();
        }
        if input.did_press_key(Keycode::Right) {
            self.move_right();
        }
        if input.did_press_key(Keycode::Up) {
            self.values[self.active] = match self.values[self.active] {
                None => Some('a'),
                Some('z') => None,
                Some(c) => Some((c as u8 + 1) as char),
            };
        }
        if input.did_press_key(Keycode::Down) {
            self.values[self.active] = match self.values[self.active] {
                None => Some('z'),
                Some('a') => None,
                Some(c) => Some((c as u8 - 1) as char),
            };
        }
        self.flicker.update(elapsed);
    }

    pub fn before_draw(&mut self) -> Result<()> {
        let updated = CacheValue(self.values[self.active]);
        if self.textures[self.active].value != updated {
            self.textures[self.active] = Self::load_char(updated, &*self.font)?;
        }
        Ok(())
    }

    pub fn extract(&self) -> String {
        self.values
            .iter()
            .map(|v| v.unwrap_or(' '))
            .collect::<String>()
            .trim()
            .into()
    }

    fn load_char(value: CacheValue<Option<char>>, font: &F) -> Result<TextCache<T, Option<char>>> {
        TextCache::load(value, font, &|c| c.to_string())
    }

    fn move_left(&mut self) {
        if self.active > 0 {
            self.active -= 1;
        }
    }

    fn move_right(&mut self) {
        self.active = cmp::min(self.active + 1, 5);
    }
}

impl<'t, R: Renderer<'t>, F> Scene<R> for EditText<R::Texture, F>
where
    R::Texture: Texture,
{
    fn show(&self, renderer: &mut R) -> moho::errors::Result<()> {
        let mut dst: options::Destination = self.tl.into();

        {
            let texture = &self.label;
            let options = options::at(dst);
            dst = dst.nudge(glm::ivec2(texture.dims().x as i32, 0));
            renderer.copy(texture, options)
        }?;

        self.textures
            .iter()
            .map(|t| &t.texture)
            .enumerate()
            .map(|(i, t)| {
                let options = options::at(dst);
                dst = dst.nudge(glm::ivec2(t.dims().x as i32, 0));
                if i != self.active || self.flicker.is_shown() {
                    renderer.copy(t, options)
                } else {
                    Ok(())
                }
            })
            .try()
    }
}
