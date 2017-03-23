use duck_husky_wedding::game_data::SpriteData;
use errors::*;

use glm;
use moho::errors as moho_errors;
use moho::resource_manager::{FrameAnimator, Renderer, ResourceLoader, Scene, TileSheet};

use std::time::Duration;

pub struct Player {
    sheet: TileSheet,
    animator: FrameAnimator,
    dimensions: glm::UVec2,
    position: glm::IVec2,
}

impl Player {
    pub fn load<R: ResourceLoader>(data: SpriteData, loader: &R) -> Result<Self> {
        let file_name = format!("media/sprites/{}", data.file_name);
        let texture = loader.load_texture(&file_name)?;
        let sheet = TileSheet::new(data.tiles.into(), texture);
        let animator = FrameAnimator::new(data.frames, Duration::from_millis(50), true);
        let player = Player {
            sheet: sheet,
            animator: animator,
            dimensions: data.out_size.into(),
            position: glm::ivec2(0, 300),
        };
        Ok(player)
    }

    pub fn animate(&mut self, delta: Duration) {
        match self.animator.frame() {
            None => self.animator.start(),
            Some(_) => self.animator.animate(delta),
        }
    }

    pub fn update(&mut self) {
        self.position.x = (self.position.x + 5) % 1280;
    }
}

impl Scene for Player {
    fn show<R: Renderer>(&self, renderer: &mut R) -> moho_errors::Result<()> {
        let frame = self.animator.frame();
        match frame {
            None => Ok(()),
            Some(f) => {
                let tile = self.sheet.tile(f);
                let dst_rect = glm::ivec4(self.position.x,
                                          self.position.y,
                                          self.dimensions.x as i32,
                                          self.dimensions.y as i32);
                renderer.render(&tile, dst_rect)
            }
        }
    }
}
