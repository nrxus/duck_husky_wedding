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
mod utils;

use duck_husky_wedding::DuckHuskyWedding;
use level_viewer::LevelViewer;

use moho::input;
use sdl2::image::{INIT_JPG, INIT_PNG};

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
    let name = "Husky Loves Ducky";

    let sdl_ctx = sdl2::init().unwrap();
    let video_ctx = sdl_ctx.video().unwrap();
    let bounds = video_ctx.display_bounds(0).unwrap();
    let _image_ctx = sdl2::image::init(INIT_PNG | INIT_JPG).unwrap();

    let window = video_ctx
        .window(name, bounds.width(), bounds.height())
        .position_centered()
        .opengl()
        .fullscreen()
        .build()
        .unwrap();

    let mut renderer = window
        .into_canvas()
        .accelerated()
        .present_vsync()
        .build()
        .unwrap();
    let creator = renderer.texture_creator();

    renderer
        .set_logical_size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .unwrap();
    renderer.clear();
    renderer.present();
    let input_manager = input::Manager::new(sdl_ctx.event_pump().unwrap());

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
