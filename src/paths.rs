use std::env;
use std::fs;
use std::io;
use std::path::PathBuf;
use tracing::debug;

pub struct AppPaths {
	pub global_config: PathBuf,
	pub local_config: Option<PathBuf>,
}

impl AppPaths {
	pub fn init() -> io::Result<Self> {
		let global_config = dirs::config_dir()
			.map(|p| p.join("rrun"))
			.unwrap_or_else(|| PathBuf::from(".rrun"));

		let current_dir = env::current_dir()?;
		let local_path = current_dir.join(".rrun");

		let local_config = if local_path.is_dir() {
			debug!("Found local config directory at {}", local_path.display());
			Some(local_path)
		} else {
			None
		};

		let paths = Self {
			global_config,
			local_config,
		};

		paths.ensure_global()?;

		Ok(paths)
	}

	fn ensure_global(&self) -> io::Result<()> {
		fs::create_dir_all(&self.global_config)?;
		debug!(
			"Ensured config directory exists at {}",
			self.global_config.display()
		);

		Ok(())
	}
}
