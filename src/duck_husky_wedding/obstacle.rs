use data;
use duck_husky_wedding::try::Try;
use errors::*;

use moho::shape::{Intersect, Rectangle};
use moho::renderer::{options, Renderer, Scene, TextureLoader, TextureManager};
use moho::errors as moho_errors;

use glm;

use std::rc::Rc;

#[derive(Clone)]
struct Vec2D<T> {
    count: glm::UVec2,
    vector: Vec<T>,
}

impl<T> Vec2D<T> {
    fn get(&self, index: (u32, u32)) -> &T {
        &self.vector[(index.0 % self.count.x + index.1 * self.count.x) as usize]
    }
}

struct Textures<T> {
    center: Rc<T>,
    left: Rc<T>,
    right: Rc<T>,
    top_center: Rc<T>,
    top_left: Rc<T>,
    top_right: Rc<T>,
}

impl<T> Clone for Textures<T> {
    fn clone(&self) -> Self {
        Textures {
            center: self.center.clone(),
            left: self.left.clone(),
            right: self.right.clone(),
            top_center: self.top_center.clone(),
            top_left: self.top_left.clone(),
            top_right: self.top_right.clone(),
        }
    }
}

pub struct Obstacle<T> {
    tiles: Vec2D<Rc<T>>,
    dims: glm::UVec2,
    tl: glm::IVec2,
    textures: Textures<T>,
}

impl<T> Clone for Obstacle<T> {
    fn clone(&self) -> Self {
        Obstacle {
            tiles: self.tiles.clone(),
            textures: self.textures.clone(),
            dims: self.dims,
            tl: self.tl,
        }
    }
}

impl<T> Obstacle<T> {
    pub fn load<'t, TL>(
        texture_manager: &mut TextureManager<'t, TL>,
        ground: &data::Ground,
        obstacle: &data::Obstacle,
    ) -> Result<Self>
    where
        TL: TextureLoader<'t, Texture = T>,
    {
        let count: glm::UVec2 = obstacle.count.into();
        let texture = ground.center.load(texture_manager)?;
        let vector = vec![texture; (count.x * count.y) as usize];
        let tiles = Vec2D { count, vector };
        let dims = ground.out_size.into();
        let tl: glm::UVec2 = obstacle.top_left.into();
        let tl = glm::to_ivec2(tl * dims);
        let textures = Textures {
            center: ground.center.load(texture_manager)?,
            left: ground.left.load(texture_manager)?,
            right: ground.right.load(texture_manager)?,
            top_center: ground.top.load(texture_manager)?,
            top_left: ground.top_left.load(texture_manager)?,
            top_right: ground.top_right.load(texture_manager)?,
        };
        Ok(Obstacle {
            tiles,
            dims,
            tl,
            textures,
        })
    }

    pub fn mtv(&self, object: &Rectangle) -> Option<glm::DVec2> {
        let obstacle = Rectangle {
            top_left: glm::to_dvec2(self.tl),
            dims: glm::to_dvec2(self.dims * self.tiles.count),
        };
        object.mtv(&obstacle)
    }
}

impl<'t, R: Renderer<'t>> Scene<R> for Obstacle<R::Texture> {
    fn show(&self, renderer: &mut R) -> moho_errors::Result<()> {
        (0..self.tiles.count.x)
            .flat_map(|i| (0..self.tiles.count.y).map(move |j| (i, j)))
            .map(|(i, j)| {
                let texture = if j == 0 {
                    &self.textures.top_center
                } else {
                    &self.textures.center
                };
                (
                    texture,
                    glm::ivec4(
                        self.tl.x + (self.dims.x * i) as i32,
                        self.tl.y + (self.dims.y * j) as i32,
                        self.dims.x as i32,
                        self.dims.y as i32,
                    ),
                )
            })
            .map(|(t, d)| renderer.copy(t, options::at(&d)))
            .try()
    }
}
