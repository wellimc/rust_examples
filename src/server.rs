
use std::os::unix::net::{UnixListener, UnixStream};

use anyhow::Context;

fn main() -> anyhow::Result<()> {
    let socket_path = "mysocket";

    let unix_listener =
        UnixListener::bind(socket_path).context("Could not create the unix socket")?;

    Ok(())
}
