use errors::*;
use super::Dimension;

use serde_yaml;

use std::fs::File;

#[derive(Debug, Deserialize)]
pub struct Obstacle {
    pub count: Dimension,
    pub top_left: Dimension,
}

#[derive(Debug, Deserialize)]
pub struct Level {
    pub obstacles: Vec<Obstacle>,
}

impl Level {
    pub fn load(path: &'static str) -> Result<Self> {
        let f = File::open(path)?;
        serde_yaml::from_reader(&f).map_err(Into::into)
    }
}