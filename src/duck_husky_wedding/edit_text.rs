use duck_husky_wedding::hud::{AsCached, CacheValue, TextCache};
use errors::*;
use utils::Try;

use glm;
use moho::errors as moho_errors;
use moho::input;
use moho::renderer::{options, ColorRGBA, Destination, FontTexturizer, Renderer, Scene, Texture};
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

enum FlickerState {
    Hide,
    Show,
}

impl FlickerState {
    fn toggle(&mut self) {
        *self = match *self {
            FlickerState::Hide => FlickerState::Show,
            FlickerState::Show => FlickerState::Hide,
        }
    }
}

struct Flicker {
    duration: Duration,
    remaining: Duration,
    state: FlickerState,
}

impl Flicker {
    fn new(duration: Duration) -> Self {
        Flicker {
            duration,
            state: FlickerState::Show,
            remaining: duration,
        }
    }

    fn update(&mut self, delta: Duration) {
        match self.remaining.checked_sub(delta) {
            None => {
                self.state.toggle();
                self.remaining = self.duration;
            }
            Some(d) => self.remaining = d,
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

impl<T, F> EditText<T, F> {
    pub fn load<'t, FT>(
        label: &str,
        tl: glm::IVec2,
        font: Rc<F>,
        texturizer: &'t FT,
    ) -> Result<Self>
    where
        FT: FontTexturizer<'t, F, Texture = T>,
    {
        let textures = {
            let font = &*font;
            [
                Self::load_char(CacheValue(None), font, texturizer)?,
                Self::load_char(CacheValue(None), font, texturizer)?,
                Self::load_char(CacheValue(None), font, texturizer)?,
                Self::load_char(CacheValue(None), font, texturizer)?,
                Self::load_char(CacheValue(None), font, texturizer)?,
                Self::load_char(CacheValue(None), font, texturizer)?,
            ]
        };
        let values = [None; 6];
        let label = texturizer.texturize(&*font, label, &ColorRGBA(255, 255, 255, 255))?;
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

    pub fn before_draw<'t, FT>(&mut self, texturizer: &'t FT) -> Result<()>
    where
        FT: FontTexturizer<'t, F, Texture = T>,
    {
        let updated = CacheValue(self.values[self.active]);
        if self.textures[self.active].value != updated {
            self.textures[self.active] = Self::load_char(updated, &*self.font, texturizer)?;
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

    fn load_char<'t, FT>(
        value: CacheValue<Option<char>>,
        font: &F,
        texturizer: &'t FT,
    ) -> Result<TextCache<T, Option<char>>>
    where
        FT: FontTexturizer<'t, F, Texture = T>,
    {
        TextCache::load(value, font, texturizer, &|c| c.to_string())
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
    fn show(&self, renderer: &mut R) -> moho_errors::Result<()> {
        let mut dst: Destination = self.tl.into();

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
                if i == self.active {
                    match self.flicker.state {
                        FlickerState::Hide => Ok(()),
                        FlickerState::Show => renderer.copy(t, options),
                    }
                } else {
                    renderer.copy(t, options)
                }
            })
            .try()
    }
}
