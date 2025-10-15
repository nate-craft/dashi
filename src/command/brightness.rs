use std::{
    error::Error,
    fs::{self},
    path::{Path, PathBuf},
};

use crate::{command::BrightnessCommand, notify::notify};

pub struct Brightness {
    silent: bool,
}

impl Brightness {
    pub fn new(silent: bool) -> Self {
        Brightness { silent }
    }

    pub fn run(&self, modifier: BrightnessCommand) -> Result<(), Box<dyn Error>> {
        match modifier {
            BrightnessCommand::Add { n } => {
                self.set_brightness(self.get_brightness()? as f32 + n as f32)?
            }
            BrightnessCommand::Sub { n } => {
                self.set_brightness(self.get_brightness()? as f32 - n as f32)?
            }
            BrightnessCommand::Set { n } => self.set_brightness((n as f32).min(0.01).max(100.0))?,
            BrightnessCommand::Get => {}
        }

        let brightness_new = self.get_brightness()?;

        if brightness_new == 0 {
            Ok(notify(self.silent, "Brightness", "Minimum")?)
        } else {
            Ok(notify(
                self.silent,
                "Brightness",
                format!("{}%", brightness_new),
            )?)
        }
    }

    fn set_brightness(&self, percent: f32) -> Result<(), Box<dyn Error>> {
        let raw = self.get_raw_max_brightness()? as f32;
        let written = (percent.min(100.0).max(0.01) / 100.0 * raw) as u32;

        Ok(fs::write(
            self.get_path_brightness(),
            format!("{}\0", written),
        )?)
    }

    fn get_brightness(&self) -> Result<u32, Box<dyn Error>> {
        let num = self.get_raw_brightness()? as f32;
        let denom = self.get_raw_max_brightness()? as f32;
        let percent = (num / denom * 100.0) as u32;
        let rounded = (percent + 2) / 5 * 5 as u32;

        Ok((rounded as u32).min(100))
    }

    fn get_raw_brightness(&self) -> Result<u32, Box<dyn Error>> {
        let as_string = fs::read_to_string(self.get_path_brightness())?;
        Ok(as_string[..as_string.len() - 1].parse::<u32>()?)
    }

    fn get_raw_max_brightness(&self) -> Result<u32, Box<dyn Error>> {
        let as_string = fs::read_to_string(self.get_path_max_brightness())?;
        Ok(as_string[..as_string.len() - 1].parse::<u32>()?)
    }

    fn get_path_brightness(&self) -> PathBuf {
        Path::new("/sys/class/backlight/intel_backlight/brightness").to_path_buf()
    }

    fn get_path_max_brightness(&self) -> PathBuf {
        Path::new("/sys/class/backlight/intel_backlight/max_brightness").to_path_buf()
    }
}
