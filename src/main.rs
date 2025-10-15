use std::error::Error;

use clap::{command, Parser};

use crate::command::{brightness::Brightness, volume::Volume, Command};

mod command;
mod notify;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    silent: bool,
    #[command(subcommand)]
    command: Command,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    match args.command {
        Command::Brightness { modifier } => Brightness::new(args.silent).run(modifier)?,
        Command::Volume { modifier } => Volume::new(args.silent).run(modifier)?,
        Command::Backup => todo!(),
        Command::Nightshift => todo!(),
    }

    Ok(())
}
