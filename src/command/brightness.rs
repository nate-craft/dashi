use std::{
    error::Error,
    fs::{self},
    io::{self, ErrorKind},
    path::{Path, PathBuf},
};

use crate::{command::BrightnessCommand, notify::notify};

pub struct BrightnessSpec {
    silent: bool,
}

impl BrightnessSpec {
    pub fn new(silent: bool) -> Self {
        BrightnessSpec { silent }
    }

    pub fn run(&self, modifier: BrightnessCommand) -> Result<(), Box<dyn Error>> {
        match modifier {
            BrightnessCommand::Add { n } => {
                self.set_brightness(self.get_brightness()? as f32 + n as f32)?
            }
            BrightnessCommand::Sub { n } => {
                self.set_brightness(self.get_brightness()? as f32 - n as f32)?
            }
            BrightnessCommand::Set { n } => self.set_brightness((n as f32).min(100.0).max(0.01))?,
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
            self.get_path_brightness()?,
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
        let as_string = fs::read_to_string(self.get_path_brightness()?)?;
        Ok(as_string[..as_string.len() - 1].parse::<u32>()?)
    }

    fn get_raw_max_brightness(&self) -> Result<u32, Box<dyn Error>> {
        let as_string = fs::read_to_string(self.get_path_max_brightness()?)?;
        Ok(as_string[..as_string.len() - 1].parse::<u32>()?)
    }

    fn get_path_system(&self) -> Result<PathBuf, io::Error> {
        Path::new("/sys/class/backlight/")
            .read_dir()?
            .last()
            .ok_or(io::Error::from(ErrorKind::NotFound))?
            .map(|entry| entry.path())
    }

    fn get_path_brightness(&self) -> Result<PathBuf, io::Error> {
        Ok(self.get_path_system()?.join("brightness"))
    }

    fn get_path_max_brightness(&self) -> Result<PathBuf, io::Error> {
        Ok(self.get_path_system()?.join("max_brightness"))
    }
}
