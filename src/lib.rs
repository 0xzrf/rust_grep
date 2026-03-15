use clap::Parser;
mod cli;
mod helpers;
mod types;
use cli::GrepArgs;
use types::*;
mod errors;
use errors::*;

pub fn run() -> Result<(), GrepError> {
    let args = GrepArgs::parse();

    args.match_pattern()?;

    Ok(())
}
