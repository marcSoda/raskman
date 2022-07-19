#[macro_use]
extern crate log;

mod cli;
mod app;

use cli::{ banner::BANNER };
use app::{
    Rask,
    // note::Note,
};

// use app::parser;

fn main() {
    env_logger::init();
    let clap = cli::get_clap();
    let res = cli::dispatch_commands(&clap, Rask::new());
    match res {
        Ok(rask) => {
            println!("rask: {:?}", rask)
            //todo: save the rask to disk with serde
        },
        Err(e) => error!("{}", e),
    };
}
