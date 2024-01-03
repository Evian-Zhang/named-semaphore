use std::process::Command;

use anyhow::{Context, Result};
use clap::Parser;
use named_semaphore::NamedSemaphore;

/// Execute command preemptively.
#[derive(Parser)]
struct Args {
    /// Name of the semaphore.
    ///
    /// In Linux, this name should begin with a "/".
    #[arg(long)]
    name: String,
    /// Semaphore's initial value.
    #[arg(long)]
    count: usize,
    /// Command to execute.
    #[arg(last = true)]
    command: Vec<String>,
}

fn main() -> Result<()> {
    let Args {
        name,
        count,
        command,
    } = Args::parse();

    let Some((command, args)) = command.split_first() else {
        return Err(anyhow::anyhow!("No program given"));
    };

    let mut command = Command::new(command);
    command.args(args);

    let mut semaphore = NamedSemaphore::create(name, count as u32)?;

    semaphore.wait_then_post(|| -> Result<()> {
        let mut child = command.spawn().context("Failed to spawn command")?;
        child.wait().context("Failed to wait for child")?;
        Ok(())
    })??;

    Ok(())
}
