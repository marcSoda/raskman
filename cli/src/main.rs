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
            //todo: known bug: if there are no tasks, OpenOptions will create an empty file which will not be written to. any future calls to the app with an empty data file will cause an error
            if data_existed { return Err(Box::new(e)); }
            Rask::new()
        },
    };
    rask.needs_saving = false;
    match cli::dispatch_commands(&clap, &mut rask) {
        Ok(_) => (),
        Err(e) => error!("{}", e),
    };
    if rask.needs_saving {
        file.set_len(0)?;
        file.seek(SeekFrom::Start(0))?;
        serde_json::to_writer_pretty(&file, &rask)?;
    }
    Ok(())
}
