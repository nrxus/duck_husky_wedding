use duck_husky_wedding::game_data::SpriteData;
use errors::*;

use glm;
use moho::errors as moho_errors;
use moho::renderer::{FrameAnimator, Renderer, ResourceLoader, Scene, Texture, TileSheet};

use std::time::Duration;
use std::rc::Rc;

pub struct Player<T: Texture> {
    sheet: TileSheet<T>,
    animator: FrameAnimator,
    dimensions: glm::UVec2,
    position: glm::IVec2,
}

impl<T: Texture> Player<T> {
    pub fn load<R>(data: SpriteData, loader: &R) -> Result<Self>
        where R: for<'a> ResourceLoader<'a, T>
    {
        let file_name = format!("media/sprites/{}", data.file_name);
        let texture = loader.load(&file_name)?;
        let sheet = TileSheet::new(data.tiles.into(), Rc::new(texture));
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

impl<T: Texture, R> Scene<R> for Player<T>
    where R: Renderer<T>
{
    fn show(&self, renderer: &mut R) -> moho_errors::Result<()> {
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
