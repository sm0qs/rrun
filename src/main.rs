mod cli;
mod logging;
mod paths;

use clap::Parser;
use cli::Cli;
use logging::Logger;
use paths::AppPaths;
use std::io;
use tracing::info;

fn main() -> io::Result<()> {
	Logger::init();

	AppPaths::init()?;

	let args = Cli::parse();
	match args.script {
		Some(script) => {
			info!("Running script: {}", script);
		}
		None => {
			info!("No script provided.");
		}
	}

	Ok(())
}
