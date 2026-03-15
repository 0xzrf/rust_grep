use clap::Parser;
mod cli;
mod helpers;
use cli::GrepArgs;


pub fn run() -> Result<(), String> {
    let args = GrepArgs::parse();

    if args.match_pattern() { Ok(()) } else { Err("An unexpected error occured".to_string()) }
}
