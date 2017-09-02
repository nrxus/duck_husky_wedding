use errors::*;

use serde_yaml;

use std::fs::File;

const PATH: &'static str = "media/high_scores.yaml";

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ScoreEntry {
    pub score: u32,
    pub name: String,
}

pub fn get() -> Vec<ScoreEntry> {
    File::open(PATH)
        .chain_err(|| "")
        .and_then(|f| serde_yaml::from_reader(f).map_err(Into::into))
        .unwrap_or_default()
}

pub fn create(entries: &[ScoreEntry]) -> Result<()> {
    let file = File::create(PATH)?;
    serde_yaml::to_writer(file, entries).map_err(Into::into)
}
