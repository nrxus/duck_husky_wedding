use errors::*;
use super::Dimension;

use serde_yaml;

use std::fs::File;

#[derive(Debug, Deserialize)]
pub enum CatKind {
    Idle,
    Moving(u32),
}

#[derive(Debug, Deserialize)]
pub struct Cat {
    pub kind: CatKind,
    pub bottom_left: Dimension,
}

#[derive(Debug, Deserialize)]
pub struct Obstacle {
    pub count: Dimension,
    pub bottom_left: Dimension,
}

#[derive(Debug, Deserialize)]
pub struct Level {
    pub obstacles: Vec<Obstacle>,
    pub goal: Dimension,
    pub gems: Vec<Dimension>,
    pub coins: Vec<Dimension>,
    pub cats: Vec<Cat>,
}

impl Level {
    pub fn load(path: &'static str) -> Result<Self> {
        let f = File::open(path)?;
        serde_yaml::from_reader(&f).map_err(Into::into)
    }
}
