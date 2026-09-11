use clap::{
	Parser,
	builder::{Styles, styling::AnsiColor},
};

fn get_styles() -> Styles {
	Styles::styled()
		.header(AnsiColor::Yellow.on_default())
		.literal(AnsiColor::Cyan.on_default())
}

#[derive(Parser)]
#[command(version, about, styles = get_styles())]
pub struct Cli {
	/// Script to run
	pub script: Option<String>,
}
