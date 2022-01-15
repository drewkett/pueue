use std::fs::File;
use std::io::Read;
use std::path::Path;

use anyhow::{anyhow, bail, Context, Result};
use procfs::process::Process;
use pueue_lib::network::message::*;
use pueue_lib::settings::*;

use super::*;

/// Send the Shutdown message to the test daemon.
pub async fn shutdown_daemon(shared: &Shared) -> Result<Message> {
    let message = Message::DaemonShutdown(Shutdown::Graceful);

    send_message(shared, message)
        .await
        .context("Failed to send Shutdown message")
}

/// Get a daemon pid from a specific pueue directory.
/// This function gives the daemon a little time to boot up, but ultimately crashes if it takes too
/// long.
pub fn get_pid(pueue_dir: &Path) -> Result<i32> {
    let pid_file = pueue_dir.join("pueue.pid");

    // Give the daemon about 1 sec to boot and create the pid file.
    let tries = 20;
    let mut current_try = 0;

    while current_try < tries {
        // The daemon didn't create the pid file yet. Wait for 100ms and try again.
        if !pid_file.exists() {
            sleep_ms(50);
            current_try += 1;
            continue;
        }

        let mut file = File::open(&pid_file).context("Couldn't open pid file")?;
        let mut content = String::new();
        file.read_to_string(&mut content)
            .context("Couldn't write to file")?;

        // The file has been created but not yet been written to.
        if content.is_empty() {
            sleep_ms(50);
            current_try += 1;
            continue;
        }

        let pid = content
            .parse::<i32>()
            .map_err(|_| anyhow!("Couldn't parse value: {content}"))?;
        return Ok(pid);
    }

    bail!("Couldn't find pid file after about 1 sec.");
}

/// Waits for a daemon to shut down.
/// This is done by waiting for the pid to disappear.
pub fn wait_for_shutdown(pid: i32) -> Result<()> {
    // Try to read the process. If this fails, the daemon already exited.
    let process = match Process::new(pid) {
        Ok(process) => process,
        Err(_) => return Ok(()),
    };

    // Give the daemon about 1 sec to shutdown.
    let tries = 40;
    let mut current_try = 0;

    while current_try < tries {
        // Process is still alive, wait a little longer
        if process.is_alive() {
            sleep_ms(50);
            current_try += 1;
            continue;
        }

        return Ok(());
    }

    bail!("Couldn't find pid file after about 2 sec.");
}
