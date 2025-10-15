use clap::Subcommand;

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
