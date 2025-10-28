use clap::Subcommand;

pub mod bluetooth;
pub mod bookmark;
pub mod brightness;
pub mod volume;

#[derive(Subcommand)]
pub enum Command {
    Brightness {
        #[command(subcommand)]
        modifier: BrightnessCommand,
    },
    Volume {
        #[command(subcommand)]
        modifier: VolumeCommand,
    },
    Bookmark {
        #[command(subcommand)]
        modifier: BookmarkCommand,
    },
    Bluetooth {
        #[command(subcommand)]
        modifier: BluetoothCommand,
    },
    Backup,
    Nightshift,
}

#[derive(Subcommand)]
pub enum VolumeCommand {
    Add {
        #[arg(value_enum)]
        n: u32,
    },
    Sub {
        #[arg(value_enum)]
        n: u32,
    },
    Set {
        #[arg(value_enum)]
        n: u32,
    },
    Get,
    Muted,
    MutedMic,
    Mute,
    MuteMic,
}

#[derive(Subcommand)]
pub enum BrightnessCommand {
    Add {
        #[arg(value_enum)]
        n: u32,
    },
    Sub {
        #[arg(value_enum)]
        n: u32,
    },
    Set {
        #[arg(value_enum)]
        n: u32,
    },
    Get,
}

#[derive(Subcommand)]
pub enum BookmarkCommand {
    Stdout,
    Add {
        #[arg(value_enum)]
        bookmark: String,
    },
    Remove {
        #[arg(value_enum)]
        index: usize,
    },
}

#[derive(Subcommand)]
pub enum BluetoothCommand {
    Toggle,
    Status,
}
