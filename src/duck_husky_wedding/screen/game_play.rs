use duck_husky_wedding::player::Player;
use duck_husky_wedding::world::{self, World};
use duck_husky_wedding::camera::ViewPort;
use duck_husky_wedding::hud::TextBox;
use duck_husky_wedding::font;
use utils::{Center, Try};
use data;
use errors::*;

use utils::VecUtils;

use glm;
use moho::input;
use moho::errors as moho_errors;
use moho::renderer::{align, options, Canvas, ColorRGBA, Destination, Font, FontTexturizer,
                     Renderer, Scene, Texture, TextureLoader, TextureManager};
use moho::shape::Shape;
use sdl2::rect::Rect;
use sdl2::keyboard::Keycode;

use std::cmp;
use std::rc::Rc;
use std::time::Duration;

struct Splash<T> {
    texture: T,
    duration: Duration,
    dst: Destination,
}

impl<'t, R: Renderer<'t>> Scene<R> for Splash<R::Texture> {
    fn show(&self, renderer: &mut R) -> moho_errors::Result<()> {
        if self.is_active() {
            renderer.copy(&self.texture, options::at(self.dst))
        } else {
            Ok(())
        }
    }
}

impl<T> Splash<T> {
    fn is_active(&self) -> bool {
        self.duration.as_secs() > 0 || self.duration.subsec_nanos() > 0
    }

    fn update(&mut self, delta: Duration) {
        self.dst = self.dst.nudge(glm::ivec2(0, -2));

        self.duration = match self.duration.checked_sub(delta) {
            None => Duration::default(),
            Some(d) => d,
        }
    }
}

pub struct Heart<T> {
    zoom: f64,
    size: glm::UVec2,
    texture: Rc<T>,
}

#[derive(Clone, Copy, PartialEq)]
pub enum PlayerKind {
    Duck,
    Husky,
}

enum State<T, F> {
    Running,
    Transition,
    Finished(super::finish::Finish<T, F>),
    TimeUp {
        view: glm::IVec4,
        title: T,
        instructions: T,
    },
}

pub struct GamePlay<T, F> {
    player: Player<T>,
    world: World<T>,
    viewport: ViewPort,
    timer: TextBox<T, F, Duration>,
    score: TextBox<T, F, u32>,
    splashes: Vec<Splash<T>>,
    splash_font: Rc<F>,
    finish: super::finish::Data<F>,
    heart: Heart<T>,
    time_up_font: Rc<F>,
    state: State<T, F>,
}

pub struct Data<T> {
    world: world::Data<T>,
    game: data::Game,
    heart: Rc<T>,
}

impl<T> Data<T> {
    pub fn load<'t, TL>(
        texture_manager: &mut TextureManager<'t, TL>,
        level: &data::Level,
        game: data::Game,
    ) -> Result<Self>
    where
        T: Texture,
        TL: TextureLoader<'t, Texture = T>,
    {
        let world = world::Data::load(texture_manager, level, &game)?;
        let heart = game.heart.texture.load(texture_manager)?;
        Ok(Data { game, world, heart })
    }

    pub fn activate<'t, 'f, TL, FM, FT>(
        &self,
        texture_manager: &mut TextureManager<'t, TL>,
        font_manager: &mut FM,
        texturizer: &'t FT,
        kind: PlayerKind,
    ) -> Result<GamePlay<T, FM::Font>>
    where
        TL: TextureLoader<'t, Texture = T>,
        TL::Texture: Texture,
        FM: font::Manager,
        FT: FontTexturizer<'t, FM::Font, Texture = T>,
    {
        let (player, npc) = match kind {
            PlayerKind::Duck => (&self.game.duck, &self.game.husky),
            PlayerKind::Husky => (&self.game.husky, &self.game.duck),
        };
        let player = Player::load(player, glm::uvec2(100, 300), texture_manager)?;
        let world = self.world.activate(npc, texture_manager)?;
        let viewport = ViewPort::new(glm::ivec2(1280, 720));
        let font = font_manager.load(font::Kind::KenPixel, 32)?;
        let timer = TextBox::load(
            Duration::from_secs(100),
            font.clone(),
            texturizer,
            Box::new(|v| format!("Time: {:03}", v)),
        )?;
        let score = TextBox::load(
            0,
            font.clone(),
            texturizer,
            Box::new(|s| format!("Score: {:05}", s)),
        )?;
        let splashes = vec![];
        let splash_font = font_manager.load(font::Kind::KenPixel, 24)?;
        let time_up_font = font_manager.load(font::Kind::KenPixel, 64)?;
        let finish = {
            let x_size = 1080;
            let y_size = 360;
            super::finish::Data {
                title_font: font_manager.load(font::Kind::KenPixel, 48)?,
                detail_font: font_manager.load(font::Kind::Joystix, 36)?,
                view: glm::ivec4(640 - x_size / 2, 360 - y_size / 2, x_size, y_size),
            }
        };
        let heart = Heart {
            texture: self.heart.clone(),
            size: self.game.heart.out_size.into(),
            zoom: 0.,
        };

        Ok(GamePlay {
            player,
            world,
            viewport,
            timer,
            score,
            splashes,
            splash_font,
            finish,
            time_up_font,
            heart,
            state: State::Running,
        })
    }
}

impl<T, F> GamePlay<T, F> {
    pub fn update<'t, FT>(
        &mut self,
        delta: Duration,
        input: &input::State,
        texturizer: &'t FT,
    ) -> Option<super::Kind>
    where
        T: Texture,
        F: Font,
        FT: FontTexturizer<'t, F, Texture = T>,
    {
        self.splashes.retain(|s| s.is_active());
        for s in &mut self.splashes {
            s.update(delta);
        }

        match self.state {
            State::Running => {
                self.update_running(delta, input, texturizer);
                None
            }
            State::Transition => {
                if self.heart.zoom >= 1. {
                    self.state = State::Finished(
                        super::finish::Finish::load(
                            &self.finish,
                            texturizer,
                            self.score.value,
                            self.timer.value,
                        ).unwrap(),
                    );
                } else if (self.player.dst_rect.y + self.player.dst_rect.w) as i32 >=
                    self.world.npc.bottom()
                {
                    self.heart.zoom += 0.05;
                } else {
                    self.player.dst_rect.y += 4.;
                }
                None
            }
            State::Finished(ref mut f) => f.update(delta, input),
            State::TimeUp { .. } => if input.did_press_key(Keycode::Return) {
                Some(super::Kind::Menu)
            } else {
                None
            },
        }
    }

    pub fn update_running<'t, FT>(
        &mut self,
        delta: Duration,
        input: &input::State,
        texturizer: &'t FT,
    ) where
        T: Texture,
        FT: FontTexturizer<'t, F, Texture = T>,
    {
        self.world.update(delta);

        self.player.process(input);
        self.timer.update(delta);
        let (force, legs, touch_spikes) = self.world.force(&self.player);
        self.player.update((force, legs), delta);
        let center = {
            let dst = self.player.dst_rect;
            glm::ivec2((dst.x + dst.z / 2.) as i32, (dst.y + dst.w / 2.) as i32)
        };
        self.viewport.center(center);
        {
            let player = self.player.body();
            let color = ColorRGBA(0, 200, 125, 255);
            for c in self.world
                .collectables
                .retain_or_drain(|c| !player.intersects(&c.body))
            {
                let texture = texturizer
                    .texturize(self.splash_font.as_ref(), &format!("+{}", c.score), &color)
                    .unwrap();
                let splash = Splash {
                    texture,
                    duration: Duration::from_secs(1),
                    dst: glm::to_ivec2(c.body.center()).into(),
                };
                self.splashes.push(splash);
                self.score.update(c.score as i32);
            }
            let dmg = if self.player.invincibility.is_some() {
                None
            } else if self.world
                .enemies
                .iter()
                .map(|e| e.body())
                .any(|b| b.collides(&player))
            {
                Some(20)
            } else if touch_spikes {
                Some(40)
            } else {
                None
            };

            if let Some(d) = dmg {
                let dmg = -d;
                self.player.invincible();
                let color = ColorRGBA(255, 0, 0, 255);
                let texture = texturizer
                    .texturize(self.splash_font.as_ref(), &format!("{}", dmg), &color)
                    .unwrap();
                let splash = Splash {
                    texture,
                    duration: Duration::from_secs(1),
                    dst: glm::to_ivec2(self.player.dst_rect.center()).into(),
                };
                self.splashes.push(splash);
                self.score.update(dmg);
            }
        }
        if (self.player.dst_rect.x + self.player.dst_rect.z) as i32 >= self.world.npc.x() {
            self.player.invincibility = None;
            self.state = State::Transition;
        }
        if self.timer.value.as_secs() == 0 && self.timer.value.subsec_nanos() == 0 {
            self.player.invincibility = None;
            let x_size = 800;
            let y_size = 200;
            self.state = State::TimeUp {
                view: glm::ivec4(640 - x_size / 2, 360 - y_size / 2, x_size, y_size),
                title: texturizer
                    .texturize(
                        &*self.time_up_font,
                        "TIME'S UP!",
                        &ColorRGBA(255, 0, 0, 255),
                    )
                    .unwrap(),
                instructions: texturizer
                    .texturize(
                        &*self.time_up_font,
                        "<PRESS ENTER>",
                        &ColorRGBA(255, 255, 255, 255),
                    )
                    .unwrap(),
            };
        }
    }

    pub fn before_draw<'t, FT>(&mut self, texturizer: &'t FT) -> Result<()>
    where
        FT: FontTexturizer<'t, F, Texture = T>,
    {
        if let State::Finished(ref mut f) = self.state {
            f.before_draw(texturizer)?;
        }
        self.score.before_draw(texturizer)?;
        self.timer.before_draw(texturizer)
    }
}

impl<'t, R: Canvas<'t>, F> Scene<R> for GamePlay<R::Texture, F>
where
    R::Texture: Texture,
{
    fn show(&self, renderer: &mut R) -> moho_errors::Result<()> {
        {
            let mut renderer = self.viewport.camera(renderer);
            renderer.show(&self.world)?;
            renderer.show(&self.player)?;
            self.splashes.iter().map(|s| renderer.show(s)).try()?;

            if self.heart.zoom > 0. {
                let dst = align::center(self.world.npc.x())
                    .middle(cmp::min(self.world.npc.y(), self.player.dst_rect.y as i32))
                    .dims(glm::to_uvec2(
                        glm::to_dvec2(self.heart.size) * self.heart.zoom,
                    ));
                renderer.copy(&*self.heart.texture, options::at(dst))?;
            }
        }

        renderer.copy_asset(&self.score, options::at(align::top(0).center(320)))?;
        renderer.copy_asset(&self.timer, options::at(align::top(0).center(960)))?;

        match self.state {
            State::Finished(ref f) => renderer.show(f),
            State::TimeUp {
                ref title,
                ref instructions,
                ref view,
            } => {
                //border
                renderer.set_draw_color(ColorRGBA(0, 0, 0, 255));
                renderer.fill_rects(&[Rect::new(view.x, view.y, view.z as u32, view.w as u32)])?;
                //background
                renderer.set_draw_color(ColorRGBA(60, 0, 70, 255));
                renderer.fill_rects(&[
                    Rect::new(
                        view.x + 6,
                        view.y + 6,
                        view.z as u32 - 12,
                        view.w as u32 - 12,
                    ),
                ])?;

                renderer.copy(title, options::at(align::bottom(360).center(640)))?;
                renderer.copy(instructions, options::at(align::top(360).center(640)))
            }
            _ => Ok(()),
        }
    }
}
