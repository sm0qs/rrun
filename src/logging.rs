use tracing_subscriber::EnvFilter;

pub struct Logger;

impl Logger {
	pub fn init() {
		let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("warn"));
		tracing_subscriber::fmt().with_env_filter(filter).init();
	}
}
