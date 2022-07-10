mod cli;

use cli::{ banner::BANNER };

fn main() {
    let cmdln = cli::get_clap();
    let matches = cmdln.get_matches();
    cli::dispatch_commands(&matches);
}
