extern crate glm;
extern crate moho;
#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate serde_derive;
extern crate serde_yaml;
extern crate sdl2;

mod duck_husky_wedding;
mod level_viewer;
mod data;

use duck_husky_wedding::DuckHuskyWedding;
use level_viewer::LevelViewer;

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
    let (renderer, creator, input_manager) =
        moho::init("Husky Loves Ducky", WINDOW_WIDTH, WINDOW_HEIGHT).unwrap();

    let mut level_viewer = false;
    for argument in std::env::args() {
        if argument == "--l" {
            level_viewer = true;
            break;
        }
    }

    if level_viewer {
        let mut game = LevelViewer::new(renderer, &creator, input_manager);
        game.run().unwrap();
    } else {
        let loader = sdl2::ttf::init().unwrap();
        let mut game = DuckHuskyWedding::new(renderer, &loader, &creator, input_manager);
        game.run().unwrap();
    }
}
