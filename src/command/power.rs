use std::{fs, path::Path, thread, time::Duration};
use color_eyre::{eyre::Error, Result};
use crate::{command::PowerCommand, daemon::Daemon, notify::notify};

pub struct PowerSpec {
    silent: bool,
}

const PATH_CAPACITY: &'static str = "/sys/class/power_supply/BAT0/capacity";
const PATH_PLUGGED: &'static str = "/sys/class/power_supply/AC/online";

impl PowerSpec {
    pub fn new(silent: bool) -> Self {
        Self { silent }
    }

    pub fn run(&self, modifier: PowerCommand) -> Result<()> {
        match modifier {
            PowerCommand::Level => {
                let capacity = self.capacity()?;
                println!("{}", capacity);
                notify(self.silent, "Battery", capacity.to_string())?;
            }
            PowerCommand::Plugged => {
                let status = if self.is_plugged()? {
                    "Charging"
                } else {
                    "Discharging"
                };

                println!("{}", status);
                notify(self.silent, "Battery Status", status)?;
            }
            PowerCommand::Info => {
                let capacity = self.capacity()?;
                let capacity_str = format!("{}%", capacity);

                if self.is_plugged()? || capacity > 20 {
                    println!("Battery: {}", &capacity_str);
                    notify(self.silent, "Battery", &capacity_str)?;
                } else {
                    println!("Low Battery: {}", &capacity_str);
                    notify(self.silent, "Low Battery", &capacity_str)?;
                }
            }
            PowerCommand::Daemon => self.daemon()?,
        }

        Ok(())
    }

    fn daemon(&self) -> Result<()> {
        if Daemon::new("dashi-power")?.is_running()? {
            return Err(Error::msg("Dashi power daemon is already in used"));
        }

        println!("Dashi battery daemon started");

        loop {
            if let Ok(capacity) = self.capacity() && !self.is_plugged()? && self.capacity()? < 20 {
                notify(self.silent, "Low Battery", format!("{}%", capacity))?;
            }
            thread::sleep(Duration::from_mins(5));
        }
    }

    fn is_plugged(&self) -> Result<bool> {
        let string = fs::read_to_string(Path::new(PATH_PLUGGED))?;
        Ok(string[..string.len() - 1].parse::<i32>()? != 0)
    }

    fn capacity(&self) -> Result<i32> {
        let string = fs::read_to_string(Path::new(PATH_CAPACITY)).map_err(|_| {
            Error::msg("Could not find BAT0 capacity. Does this computer have an internal battery?")
        })?;

        string[..string.len() - 1]
            .parse::<i32>()
            .map_err(|err| Error::new(err))
    }
}
