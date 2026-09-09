mod cli;
mod logging;
mod paths;

use clap::Parser;
use cli::Cli;
use logging::init_logging;
use paths::AppPaths;
use std::io;

fn main() -> io::Result<()> {
	init_logging();
	let args = Cli::parse();

	let paths = AppPaths::new();
	paths.ensure_dirs()?;

	println!("Running script {}", args.script);

	Ok(())
}
