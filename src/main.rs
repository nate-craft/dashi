use std::process::exit;

use clap::Parser;
use color_eyre::Result;

use crate::command::{
    bluetooth::BluetoothSpec, bookmark::BookmarkSpec, brightness::BrightnessSpec,
    nightshift::NightShiftSpec, power::PowerSpec, volume::VolumeSpec, Command,
};

mod command;
mod daemon;
mod io;
mod notify;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    silent: bool,
    #[command(subcommand)]
    command: Command,
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let args = Args::parse();

    let result = match args.command {
        Command::Brightness { modifier } => BrightnessSpec::new(args.silent).run(modifier),
        Command::Volume { modifier } => VolumeSpec::new(args.silent).run(modifier),
        Command::Bookmark { modifier } => BookmarkSpec::new(args.silent)?.run(modifier),
        Command::Bluetooth { modifier } => BluetoothSpec::new(args.silent).run(modifier),
        Command::Power { modifier } => PowerSpec::new(args.silent).run(modifier),
        Command::Nightshift { modifier } => NightShiftSpec::new(args.silent).run(modifier),
    };

    match result {
        Ok(_) => Ok(()),
        Err(err) => {
            if cfg!(debug_assertions) {
                return Err(err);
            } else {
                eprintln!("{}", err.to_string());
                exit(1);
            }
        }
    }
}
