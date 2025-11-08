use std::os::fd::{IntoRawFd, RawFd};
use std::process::Command;

use color_eyre::eyre::Error;
use color_eyre::Result;
use nix::sys::socket::{self, AddressFamily, SockFlag, SockType, UnixAddr};

use nix::errno::Errno as NixErrno;

pub struct Daemon {
    address: UnixAddr,
    socket: RawFd,
}

impl Daemon {
    pub fn kill(cmd_name: &str) -> Result<()> {
        Command::new("pkill")
            .arg(cmd_name)
            .output()
            .map(|_| ())
            .map_err(|err| Error::new(err))
    }

    pub fn new(socket_name: &str) -> Result<Daemon> {
        let address = UnixAddr::new_abstract(socket_name.as_bytes())?;
        let socket = socket::socket(
            AddressFamily::Unix,
            SockType::Stream,
            SockFlag::empty(),
            None,
        )?
        .into_raw_fd();

        Ok(Self { address, socket })
    }

    pub fn is_running(&self) -> Result<bool> {
        match socket::bind(self.socket, &self.address) {
            Ok(_) => Ok(false),
            Err(NixErrno::EADDRINUSE) => Ok(true),
            Err(generic) => Err(Error::new(generic)),
        }
    }
}
