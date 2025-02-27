#![allow(dead_code)]
use anyhow::Result;
use tokio::io::{self, AsyncWriteExt};

pub use pueue_lib::settings::PUEUE_DEFAULT_GROUP;

pub mod daemon;
pub mod envs;
pub mod fixtures;
pub mod network;
pub mod wait;

pub use daemon::*;
pub use envs::*;
pub use fixtures::*;
pub use network::*;
pub use wait::*;

/// A helper function to sleep for ms time.
/// Only used to avoid the biolerplate of importing the same stuff all over the place.
pub fn sleep_ms(ms: u64) {
    std::thread::sleep(std::time::Duration::from_millis(ms));
}

/// A small helper function, which instantly writes the given string to stdout with a newline.
/// Useful for debugging async tests.
pub async fn async_println(out: &str) -> Result<()> {
    let mut stdout = io::stdout();
    stdout
        .write_all(out.as_bytes())
        .await
        .expect("Failed to write to stdout.");

    stdout
        .write_all("\n".as_bytes())
        .await
        .expect("Failed to write to stdout.");
    stdout.flush().await?;

    Ok(())
}
