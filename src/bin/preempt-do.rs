use std::process::Command;

use anyhow::{Context, Result};
use clap::Parser;
use named_semaphore::NamedSemaphore;

#[derive(Parser)]
struct Args {
    #[arg(long)]
    name: String,
    #[arg(long)]
    count: usize,
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
