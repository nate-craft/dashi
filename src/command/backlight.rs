use std::{
    fs,
    path::{Path, PathBuf},
};

use color_eyre::{eyre::Error, Result};

use crate::{command::BacklightCommand, notify::notify};

pub struct BacklightSpec {
    silent: bool,
}

impl BacklightSpec {
    pub fn new(silent: bool) -> Self {
        BacklightSpec { silent }
    }

    pub fn run(&self, modifier: BacklightCommand) -> Result<()> {
        let result = match modifier {
            BacklightCommand::Add { n } => {
                self.set_brightness(self.get_brightness()? as f32 + n as f32)
            }
            BacklightCommand::Sub { n } => {
                self.set_brightness(self.get_brightness()? as f32 - n as f32)
            }
            BacklightCommand::Set { n } => self.set_brightness((n as f32).min(100.0).max(0.01)),
            BacklightCommand::Get => Ok(()),
        };

        match result {
            Ok(_) => self.show_brightness()?,
            Err(ref e) => {
                notify(
                    false,
                    "Dashi Error",
                    "Backlight brightness cannot be modified. See documentation for more information",
                )?;
                eprintln!(
                    r#"Backlight file is not writable without giving the current user access. See
                    https://github.com/nate-craft/dashi for more information"#
                );
                eprintln!("Error: {}", e);
            }
        }

        result
    }

    fn show_brightness(&self) -> Result<()> {
        let brightness_new = self.get_brightness()?;

        if brightness_new == 0 {
            Ok(notify(self.silent, "Backlight", "Minimum")?)
        } else {
            Ok(notify(
                self.silent,
                "Backlight",
                format!("{}%", brightness_new),
            )?)
        }
    }

    fn set_brightness(&self, percent: f32) -> Result<()> {
        let raw = self.get_raw_max_brightness()? as f32;
        let written = (percent.min(100.0).max(0.01) / 100.0 * raw) as u32;

        Ok(fs::write(
            self.get_path_brightness()?,
            format!("{}\0", written),
        )?)
    }

    fn get_brightness(&self) -> Result<u32, Error> {
        let num = self.get_raw_brightness()? as f32;
        let denom = self.get_raw_max_brightness()? as f32;
        let percent = (num / denom * 100.0) as u32;
        let rounded = (percent + 2) / 5 * 5 as u32;
        Ok((rounded as u32).min(100))
    }

    fn get_raw_brightness(&self) -> Result<u32, Error> {
        let as_string = fs::read_to_string(self.get_path_brightness()?)?;
        Ok(as_string[..as_string.len() - 1].parse::<u32>()?)
    }

    fn get_raw_max_brightness(&self) -> Result<u32, Error> {
        let as_string = fs::read_to_string(self.get_path_max_brightness()?)?;
        Ok(as_string[..as_string.len() - 1].parse::<u32>()?)
    }

    fn get_path_system(&self) -> Result<PathBuf, Error> {
        Path::new("/sys/class/leds/")
            .read_dir()
            .map_err(|err| Error::new(err))?
            .filter_map(|device| device.ok())
            .filter(|device| {
                device
                    .file_name()
                    .to_str()
                    .map(|str| str.contains("backlight"))
                    .unwrap_or(false)
            })
            .last()
            .map(|entry| entry.path())
            .ok_or_else(|| Error::msg("Could not find device in /sys/class/leds/"))
    }

    fn get_path_brightness(&self) -> Result<PathBuf, Error> {
        Ok(self.get_path_system()?.join("brightness"))
    }

    fn get_path_max_brightness(&self) -> Result<PathBuf, Error> {
        Ok(self.get_path_system()?.join("max_brightness"))
    }
}
