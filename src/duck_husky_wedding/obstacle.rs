use data;
use duck_husky_wedding::body::Body;
use utils::Try;
use errors::*;

use moho;
use moho::shape::Rectangle;
use moho::renderer::{options, Renderer, Scene, TextureLoader, TextureManager};

use glm;

use std::rc::Rc;

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
            center: Rc::clone(&self.center),
            left: Rc::clone(&self.left),
            right: Rc::clone(&self.right),
            top_center: Rc::clone(&self.top_center),
            top_left: Rc::clone(&self.top_left),
            top_right: Rc::clone(&self.top_right),
        }
    }
}

pub struct Obstacle<T> {
    count: glm::UVec2,
    dims: glm::UVec2,
    tl: glm::IVec2,
    textures: Textures<T>,
}

impl<T> Clone for Obstacle<T> {
    fn clone(&self) -> Self {
        Obstacle {
            count: self.count,
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
        let count = obstacle.count.into();
        let dims: glm::UVec2 = ground.out_size.into();
        let mut bl: glm::IVec2 = obstacle.bottom_left.into();
        bl.y += obstacle.count.y as i32;
        bl = bl * glm::to_ivec2(dims);
        let tl = glm::ivec2(bl.x, 720 - bl.y);
        let textures = Textures {
            center: ground.center.load(texture_manager)?,
            left: ground.left.load(texture_manager)?,
            right: ground.right.load(texture_manager)?,
            top_center: ground.top.load(texture_manager)?,
            top_left: ground.top_left.load(texture_manager)?,
            top_right: ground.top_right.load(texture_manager)?,
        };
        Ok(Obstacle {
            count,
            dims,
            tl,
            textures,
        })
    }

    pub fn mtv(&self, object: &Body) -> Option<glm::DVec2> {
        let obstacle = Rectangle {
            top_left: glm::to_dvec2(self.tl),
            dims: glm::to_dvec2(self.dims * self.count),
        };
        object.mtv(&obstacle)
    }
}

impl<'t, R: Renderer<'t>> Scene<R> for Obstacle<R::Texture> {
    fn show(&self, renderer: &mut R) -> moho::errors::Result<()> {
        (0..self.count.x)
            .flat_map(|i| (0..self.count.y).map(move |j| (i, j)))
            .map(|(i, j)| {
                let texture = if j == 0 {
                    if i == self.count.x - 1 {
                        &self.textures.top_right
                    } else if i == 0 {
                        &self.textures.top_left
                    } else {
                        &self.textures.top_center
                    }
                } else if i == self.count.x - 1 {
                    &self.textures.right
                } else if i == 0 {
                    &self.textures.left
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
            .map(|(t, d)| renderer.copy(t, options::at(d)))
            .try()
    }
}
