#[macro_use] extern crate log;
#[macro_use] extern crate prettytable;

mod cli;
mod app;
mod net;

use cli::banner::BANNER;
use app::Rask;

use std::path::Path;
use std::error::Error;
use std::fs::OpenOptions;
use std::io::{
    SeekFrom,
    prelude::Seek
};

//todo: you're gonna have to do this with multithreading
//create a network thread, pass it to dispather, on register, tell the thread to authenticate
//use channels: https://stackoverflow.com/questions/26199926/how-to-terminate-or-suspend-a-rust-thread-from-another-thread

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();
    let clap = cli::get_clap();
    //check if data existed before running
    let data_existed = Path::new("data.json").exists();
    //open file for writing. creat if not exists
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("data.json")?;
    //read file into rask struct
    let mut rask: Rask = match serde_json::from_reader(&file) {
        Ok(r) => r,
        Err(e) => {
            //if data existed and from_reader errors: throw error
            if data_existed { return Err(Box::new(e)); }
            Rask::new()
        },
    };
    match cli::dispatch_commands(&clap, &mut rask) {
        Ok(_) => (),
        Err(e) => error!("{}", e),
    };
    file.set_len(0)?;
    file.seek(SeekFrom::Start(0))?;
    serde_json::to_writer_pretty(&file, &rask)?;
    Ok(())
}
