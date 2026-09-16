mod cli;
mod config;
mod logging;
mod paths;
mod runner;

use crate::config::Config;
use crate::logging::Logger;
use crate::paths::AppPaths;
use crate::{cli::Cli, runner::Runner};
use anyhow::{Context, Result};
use clap::Parser;

fn main() -> Result<()> {
	Logger::init();

	let paths = AppPaths::init().context("Failed to initialize application paths")?;
	let config = Config::load(&paths.global_config, paths.local_config.as_deref())?;

	let args = Cli::parse();
	match args.script {
		Some(script_name) => Runner::run(&config, &script_name)?,
		None => {
			Runner::list(&config);
		}
	}

	Ok(())
}
