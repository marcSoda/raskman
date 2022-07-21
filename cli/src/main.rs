#[macro_use] extern crate log;
#[macro_use] extern crate prettytable;

mod cli;
mod app;

use cli::banner::BANNER;
use app::Rask;
use std::error::Error;
use std::fs::OpenOptions;
use std::io::{
    SeekFrom,
    prelude::Seek
};

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();
    let clap = cli::get_clap();
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("data.json")?;
    let rask: Rask = serde_json::from_reader(&file).unwrap_or_else(|_| Rask::new());
    file.set_len(0)?;
    file.seek(SeekFrom::Start(0))?;

    let res = cli::dispatch_commands(&clap, rask);
    match res {
        Ok(rask) => {
            serde_json::to_writer_pretty(&file, &rask)?
        },
        Err(e) => error!("{}", e),
    };
    Ok(())
}
