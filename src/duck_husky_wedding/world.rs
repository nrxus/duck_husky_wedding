use duck_husky_wedding::game_data::GroundData;
use errors::*;

use glm;
use moho::shape::Rectangle;
use moho::renderer::{options, Scene, Renderer, Show, Texture, TextureLoader, TextureManager};
use moho::errors as moho_errors;

use std::rc::Rc;

pub struct Tile<T> {
    texture: Rc<T>,
    body: Rectangle,
}

pub struct Ground<T> {
    tiles: Vec<Tile<T>>,
}

impl<T> Ground<T> {
    fn load<'t, TL>(texture_manager: &mut TextureManager<'t, TL>, data: GroundData) -> Result<Self>
        where T: Texture,
              TL: TextureLoader<'t, Texture = T>
    {
        let file_name: &str = &format!("media/sprites/{}", data.file_name);
        let texture = texture_manager.load(file_name)?;
        let dims = glm::dvec2(data.out_size.x as f64, data.out_size.y as f64);
        let tiles = (0..13)
            .map(|i| {
                let top_left = glm::dvec2(dims.x * i as f64, 600.);
                let body = Rectangle {
                    top_left: top_left,
                    dims: dims,
                };
                Tile {
                    texture: texture.clone(),
                    body: body,
                }
            })
            .collect();
        Ok(Ground { tiles })
    }
}

impl<'t, R> Scene<R> for Ground<R::Texture>
    where R: Renderer<'t> + Show
{
    fn show(&self, renderer: &mut R) -> moho_errors::Result<()> {
        let results = self.tiles
            .iter()
            .map(|t| {
                     let body = &t.body;
                     let dst_rect = glm::to_ivec4(glm::dvec4(body.top_left.x,
                                                             body.top_left.y,
                                                             body.dims.x,
                                                             body.dims.y));
                     renderer.copy(&*t.texture, options::at(&dst_rect))
            });
        for r in results {
            r?
        }
        Ok(())
    }
}

pub struct World<T> {
    ground: Ground<T>,
}

impl<T> World<T> {
    pub fn load<'t, TL>(texture_manager: &mut TextureManager<'t, TL>, data: GroundData) -> Result<Self>
        where T: Texture,
              TL: TextureLoader<'t, Texture = T>
    {
        let ground = Ground::load(texture_manager, data)?;
        Ok(World { ground })
    }
}

impl<'t, R> Scene<R> for World<R::Texture>
    where R: Renderer<'t> + Show
{
    fn show(&self, renderer: &mut R) -> moho_errors::Result<()> {
        renderer.show(&self.ground)
    }
}
