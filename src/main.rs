extern crate glm;
extern crate moho;
#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate serde_derive;
extern crate serde_yaml;

mod duck_husky_wedding;

use duck_husky_wedding::game_data::GameData;
use duck_husky_wedding::DuckHuskyWedding;

pub mod errors {
    error_chain!{
        links {
            Moho(::moho::errors::Error, ::moho::errors::ErrorKind);
        }
        foreign_links {
            Io(::std::io::Error);
            Yaml(::serde_yaml::Error);
        }
    }
}

fn main() {
    const WINDOW_WIDTH: u32 = 1280;
    const WINDOW_HEIGHT: u32 = 720;
    let game_data = GameData::load("media/game_data.yaml").unwrap();
    let (renderer, input_manager) = moho::init("Master Smasher", WINDOW_WIDTH, WINDOW_HEIGHT)
        .unwrap();
    let mut game = DuckHuskyWedding::<moho::SdlMohoEngine>::new(renderer, input_manager, game_data);
    game.run().unwrap();
}
