mod cli;
mod logging;
mod paths;

use anyhow::{Context, Result};
use clap::Parser;
use cli::Cli;
use logging::Logger;
use paths::AppPaths;
use tracing::{info, warn};

fn main() -> Result<()> {
	Logger::init();

	AppPaths::init().context("Failed to initialize application paths")?;

	let args = Cli::parse();
	match args.script {
		Some(script) => {
			info!("Running script: {}", script);
		}
		None => {
			warn!("No script provided.");
		}
	}

	Ok(())
}
