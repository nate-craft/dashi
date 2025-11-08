use std::process::Command;

use crate::{command::NightShiftCommand, daemon::Daemon, notify::notify};
use color_eyre::{eyre::Error, Result};

pub struct NightShiftSpec {
    silent: bool,
}

const SHIFT_COMMAND: &'static str = "gammastep";

impl NightShiftSpec {
    pub fn new(silent: bool) -> Self {
        Self { silent }
    }

    pub fn run(&self, modifier: NightShiftCommand) -> Result<()> {
        if !self.cmd_installed()? {
            return Err(Error::msg(
                "Gammastep is not installed! See: https://gitlab.com/chinstrap/gammastep",
            ));
        }

        let status = self.status()?;

        match modifier {
            NightShiftCommand::Start => self.start(status)?,
            NightShiftCommand::Stop => self.stop()?,
            NightShiftCommand::Toggle => self.toggle(status)?,
            NightShiftCommand::Status => self.feedback(status)?,
        }

        Ok(())
    }

    fn start(&self, running: bool) -> Result<()> {
        if running {
            return Err(Error::msg("Gammastep is already running!"));
        } else {
            Command::new(SHIFT_COMMAND)
                .output()
                .map(|_| ())
                .map_err(|err| Error::new(err))?;
            self.feedback(true)
        }
    }

    fn stop(&self) -> Result<()> {
        Daemon::kill(SHIFT_COMMAND)?;
        self.feedback(false)
    }

    fn toggle(&self, running: bool) -> Result<()> {
        if running {
            self.stop()?;
        } else {
            self.start(running)?;
        }

        self.feedback(running)
    }

    fn status(&self) -> Result<bool, Error> {
        Daemon::new("dashi-nightshift")?.is_running()
    }

    fn cmd_installed(&self) -> Result<bool, Error> {
        Command::new("which")
            .arg(SHIFT_COMMAND)
            .output()
            .map(|result| result.status.success())
            .map_err(|err| Error::new(err))
    }

    fn feedback(&self, enabled: bool) -> Result<()> {
        if enabled {
            notify(self.silent, "Nightshift", "On")?;
            println!("{}", "On");
        } else {
            notify(self.silent, "Nightshift", "Off")?;
            println!("{}", "Off");
        }

        Ok(())
    }
}
