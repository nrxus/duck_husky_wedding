extern crate moho;
#[macro_use]
extern crate error_chain;

mod duck_husky_wedding;

use duck_husky_wedding::DuckHuskyWedding;

pub mod errors {
    error_chain!{
        links {
            Moho(::moho::errors::Error, ::moho::errors::ErrorKind);
        }
    }
}

fn main() {
    const WINDOW_WIDTH: u32 = 1280;
    const WINDOW_HEIGHT: u32 = 720;
    let (renderer, input_manager) = moho::init("Master Smasher", WINDOW_WIDTH, WINDOW_HEIGHT)
        .unwrap();
    let mut game = DuckHuskyWedding::<moho::SdlMohoEngine>::new(renderer, input_manager);
    game.run().unwrap();
}
