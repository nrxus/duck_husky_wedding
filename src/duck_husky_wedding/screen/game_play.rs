use duck_husky_wedding::player::Player;
use duck_husky_wedding::world::{self, World};
use duck_husky_wedding::camera::ViewPort;
use duck_husky_wedding::hud::Timer;
use duck_husky_wedding::score::Score;
use data;
use errors::*;

use glm;
use moho::input;
use moho::errors as moho_errors;
use moho::renderer::{options, Renderer, Scene};
use moho::renderer::{FontManager, FontLoader, FontTexturizer, Texture, TextureLoader,
                     TextureManager};

use std::time::Duration;

pub enum PlayerKind {
    Duck,
    Husky,
}

pub struct GamePlay<T, F> {
    player: Player<T>,
    world: World<T>,
    viewport: ViewPort,
    timer: Timer<T, F>,
    score: Score<T, F>,
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
        let timer = Timer::load(font_manager, texturizer)?;
        let score = Score::load(font_manager, texturizer)?;
        Ok(GamePlay {
            player,
            world,
            viewport,
            timer,
            score,
        })
    }
}

impl<T, F> GamePlay<T, F> {
    pub fn update(&mut self, delta: Duration, input: &input::State) -> Option<super::Kind> {
        self.world.update(delta);
        self.player.collide_cats(&self.world.enemies);
        self.player.process(input);
        self.timer.update(delta);
        let force = self.world.force(&self.player);
        self.player.update(force, delta);
        let center = {
            let dst = self.player.dst_rect;
            glm::ivec2((dst.x + dst.z / 2.) as i32, (dst.y + dst.w / 2.) as i32)
        };
        self.viewport.center(center);
        if (self.player.dst_rect.x + self.player.dst_rect.z) as i32 >= self.world.npc.x() {
            Some(super::Kind::Menu)
        } else {
            None
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

impl<'t, R: Renderer<'t>, F> Scene<R> for GamePlay<R::Texture, F>
where
    R::Texture: Texture,
{
    fn show(&self, renderer: &mut R) -> moho_errors::Result<()> {
        {
            let mut camera = self.viewport.camera(renderer);
            camera.show(&self.world)?;
            camera.show(&self.player)?;
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
        )
    }
}
