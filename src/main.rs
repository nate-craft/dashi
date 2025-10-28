use clap::Parser;
use color_eyre::Result;

use crate::command::{
    bluetooth::BluetoothSpec, bookmark::BookmarkSpec, brightness::BrightnessSpec,
    volume::VolumeSpec, Command,
};

mod command;
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
    let args = Args::parse();

    match args.command {
        Command::Brightness { modifier } => BrightnessSpec::new(args.silent).run(modifier)?,
        Command::Volume { modifier } => VolumeSpec::new(args.silent).run(modifier)?,
        Command::Bookmark { modifier } => BookmarkSpec::new(args.silent)?.run(modifier)?,
        Command::Bluetooth { modifier } => BluetoothSpec::new(args.silent).run(modifier)?,
        Command::Backup => todo!(),
        Command::Nightshift => todo!(),
    }

    Ok(())
}
