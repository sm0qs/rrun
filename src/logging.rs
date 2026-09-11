use std::env;
use tracing_subscriber::EnvFilter;

pub struct Logger;

impl Logger {
	pub fn init() {
		let level = match env::var("RUST_LOG") {
			Ok(val) if val.eq_ignore_ascii_case("trace") => "trace",
			Ok(val) if val.eq_ignore_ascii_case("debug") => "debug",
			_ => "info",
		};

		tracing_subscriber::fmt()
			.with_env_filter(EnvFilter::new(level))
			.init();
	}
}
