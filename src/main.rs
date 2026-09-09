mod cli;
mod logging;

use clap::Parser;
use cli::Cli;
use logging::init_logging;

fn main() {
	init_logging();
	let args = Cli::parse();

	println!("Running script {}", args.script);
}
