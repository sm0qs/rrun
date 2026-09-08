mod cli;

use clap::Parser;
use cli::Cli;

fn main() {
	let _args = Cli::parse();
}
