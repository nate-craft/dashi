use crate::{command::BluetoothCommand, notify::notify};
use color_eyre::{eyre::Error, Result};
use zbus::{
    blocking::{Connection, Proxy},
    zvariant::OwnedObjectPath,
};

pub struct BluetoothSpec {
    silent: bool,
}

// Dbus Docs: https://www.freedesktop.org/wiki/Software/systemd/dbus/

const SERVICE: &'static str = "bluetooth.service";
const TOGGLE_MODE: &'static str = "replace";

impl BluetoothSpec {
    pub fn new(silent: bool) -> Self {
        BluetoothSpec { silent }
    }

    pub fn run(&self, modifier: BluetoothCommand) -> Result<()> {
        let connection = Connection::system()?;
        let proxy = Proxy::new(
            &connection,
            "org.freedesktop.systemd1",
            "/org/freedesktop/systemd1",
            "org.freedesktop.systemd1.Manager",
        )?;

        match modifier {
            BluetoothCommand::Toggle => self.toggle_status(&proxy)?,
            BluetoothCommand::Status => {
                let enabled = self.get_status(&proxy)?;
                self.feedback(enabled)?;
            }
        }

        Ok(())
    }

    fn toggle_status(&self, proxy: &Proxy) -> Result<()> {
        let enabled = self.get_status(proxy)?;

        if enabled {
            proxy.call_method("StopUnit", &(SERVICE, TOGGLE_MODE))?;
        } else {
            proxy.call_method("StartUnit", &(SERVICE, TOGGLE_MODE))?;
        }

        self.feedback(!enabled)
    }

    fn get_status(&self, proxy: &Proxy) -> Result<bool> {
        type Unit = (
            String,
            String,
            String,
            String,
            String,
            String,
            OwnedObjectPath,
            u32,
            String,
            OwnedObjectPath,
        );

        proxy
            .call::<_, _, Vec<Unit>>("ListUnitsByNames", &(vec![SERVICE]))?
            .first()
            .map(|unit| &unit.3 == "active")
            .ok_or(Error::msg("Could not find bluetooth service"))
    }

    fn feedback(&self, enabled: bool) -> Result<()> {
        if enabled {
            notify(self.silent, "Bluetooth", "Enabled")?;
            println!("{}", "Enabled");
        } else {
            notify(self.silent, "Bluetooth", "Disabled")?;
            println!("{}", "Disabled");
        }

        Ok(())
    }
}
