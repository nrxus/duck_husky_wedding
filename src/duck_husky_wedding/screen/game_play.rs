use duck_husky_wedding::player::Player;
use duck_husky_wedding::world::{self, World};
use duck_husky_wedding::camera::ViewPort;
use duck_husky_wedding::hud::Timer;
use duck_husky_wedding::score::Score;
use utils::{Center, Try};
use data;
use errors::*;

use utils::VecUtils;

use glm;
use moho::input;
use moho::errors as moho_errors;
use moho::renderer::{options, Canvas, ColorRGBA, Font, FontDetails, FontLoader, FontManager,
                     FontTexturizer, Renderer, Scene, Texture, TextureLoader, TextureManager};

use std::rc::Rc;
use std::time::Duration;

struct Splash<T> {
    texture: T,
    duration: Duration,
    dst: glm::IVec4,
}

impl<'t, R: Renderer<'t>> Scene<R> for Splash<R::Texture> {
    fn show(&self, renderer: &mut R) -> moho_errors::Result<()> {
        if self.is_active() {
            renderer.copy(&self.texture, options::at(&self.dst))
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
        self.dst.y -= 2;

        self.duration = match self.duration.checked_sub(delta) {
            None => Duration::default(),
            Some(d) => d,
        }
    }
}

pub enum PlayerKind {
    Duck,
    Husky,
}

enum State<T> {
    Running,
    Transition,
    Finished(super::finish::Finish<T>),
}

pub struct GamePlay<T, F> {
    player: Player<T>,
    world: World<T>,
    viewport: ViewPort,
    timer: Timer<T, F>,
    score: Score<T, F>,
    splashes: Vec<Splash<T>>,
    splash_font: Rc<F>,
    finish: super::finish::Data<F>,
    state: State<T>,
}

pub struct Data<T> {
    world: world::Data<T>,
    game: data::Game,
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
        Ok(Data { game, world })
    }

    pub fn activate<'t, 'f, TL, FL, FT>(
        &self,
        texture_manager: &mut TextureManager<'t, TL>,
        font_manager: &mut FontManager<'f, FL>,
        texturizer: &'t FT,
        kind: PlayerKind,
    ) -> Result<GamePlay<T, FL::Font>>
    where
        TL: TextureLoader<'t, Texture = T>,
        TL::Texture: Texture,
        FL: FontLoader<'f>,
        FT: FontTexturizer<'t, FL::Font, Texture = T>,
    {
        let (player, npc) = match kind {
            PlayerKind::Duck => (&self.game.duck, &self.game.husky),
            PlayerKind::Husky => (&self.game.husky, &self.game.duck),
        };
        let player = Player::load(player, glm::uvec2(100, 300), texture_manager)?;
        let world = self.world.activate(npc, texture_manager)?;
        let viewport = ViewPort::new(glm::ivec2(1280, 720));
        let font = font_manager.load(&FontDetails {
            path: "media/fonts/kenpixel_mini.ttf",
            size: 32,
        })?;
        let timer = Timer::load(font.clone(), texturizer)?;
        let score = Score::load(font, texturizer)?;
        let splashes = vec![];
        let splash_font = {
            let details = FontDetails {
                path: "media/fonts/kenpixel_mini.ttf",
                size: 24,
            };
            font_manager.load(&details)
        }?;
        let finish = {
            let x_margin = 100;
            let y_margin = 180;
            super::finish::Data {
                title_font: font_manager.load(&FontDetails {
                    path: "media/fonts/kenpixel_mini.ttf",
                    size: 48,
                })?,
                detail_font: font_manager.load(&FontDetails {
                    path: "media/fonts/joystix.monospace.ttf",
                    size: 36,
                })?,
                view: glm::ivec4(x_margin, y_margin, 1280 - x_margin * 2, 720 - y_margin * 2),
            }
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
                if (self.player.dst_rect.y + self.player.dst_rect.w) as i32 >=
                    self.world.npc.bottom()
                {
                    self.state = State::Finished(
                        super::finish::Finish::load(
                            &self.finish,
                            texturizer,
                            self.score.value,
                            self.timer.value,
                        ).unwrap(),
                    );
                } else {
                    self.player.dst_rect.y += 4.;
                }
                None
            }
            State::Finished(ref mut f) => f.update(input),
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
        let force = self.world.force(&self.player);
        self.player.update(force, delta);
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
                let dims = texture.dims();
                let splash = Splash {
                    texture,
                    duration: Duration::from_secs(1),
                    dst: glm::ivec4(
                        c.body.top_left.x as i32,
                        c.body.top_left.y as i32,
                        dims.x as i32,
                        dims.y as i32,
                    ),
                };
                self.splashes.push(splash);
                self.score.update(c.score as i32);
            }
            if !self.player.invincibility.is_active() {
                if let Some(_) = self.world
                    .enemies
                    .iter()
                    .map(|e| e.body())
                    .find(|b| b.collides(&player))
                {
                    let dmg = -20;
                    self.player.invincibility.activate();
                    let color = ColorRGBA(255, 0, 0, 255);
                    let texture = texturizer
                        .texturize(self.splash_font.as_ref(), &format!("{}", dmg), &color)
                        .unwrap();
                    let dims = glm::to_ivec2(texture.dims());
                    let tl = glm::to_ivec2(self.player.dst_rect.center()) - dims / 2;
                    let splash = Splash {
                        texture,
                        duration: Duration::from_secs(1),
                        dst: glm::ivec4(tl.x, tl.y, dims.x, dims.y),
                    };
                    self.splashes.push(splash);
                    self.score.update(dmg);
                }
            }
        }
        if (self.player.dst_rect.x + self.player.dst_rect.z) as i32 >= self.world.npc.x() {
            self.player.invincibility.deactivate();
            self.state = State::Transition;
        }
    }

    pub fn before_draw<'t, FT>(&mut self, texturizer: &'t FT) -> Result<()>
    where
        FT: FontTexturizer<'t, F, Texture = T>,
    {
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
        }

        let sc = glm::to_ivec2(self.score.dims());
        renderer.copy_asset(
            &self.score,
            options::at(&glm::ivec4(320 - sc.x / 2, 0, sc.x, sc.y)),
        )?;

        let td = glm::to_ivec2(self.timer.dims());
        renderer.copy_asset(
            &self.timer,
            options::at(&glm::ivec4(960 - td.x / 2, 0, td.x, td.y)),
        )?;

        if let State::Finished(ref f) = self.state {
            renderer.show(f)
        } else {
            Ok(())
        }
    }
}
