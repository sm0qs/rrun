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
use std::process;
use tracing::error;

fn main() {
	Logger::init();

	if let Err(err) = try_main() {
		error!("{:#}", err);
		process::exit(1);
	}
}

fn try_main() -> Result<()> {
	let args = Cli::parse();
	let paths = AppPaths::init().context("Failed to initialize application paths")?;
	let config = Config::load(&paths.global_config, paths.local_config.as_deref())?;

	match args.script {
		Some(script_name) => Runner::run(&config, &script_name),
		None => {
			Runner::list(&config);
		}
	}

	Ok(())
}
