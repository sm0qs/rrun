use std::fs;
use std::io;
use std::path::PathBuf;

use tracing::error;
use tracing::info;

pub struct AppPaths {
	pub config: PathBuf,
}

impl AppPaths {
	pub fn new() -> Self {
		let config = dirs::config_dir()
			.map(|p| p.join("rrun"))
			.unwrap_or_else(|| PathBuf::from(".rrun"));

		Self { config }
	}

	pub fn ensure_dirs(&self) -> io::Result<()> {
		if !self.config.exists() {
			if let Err(err) = fs::create_dir_all(&self.config) {
				error!(
					"Could not create directory {}: {}",
					self.config.display(),
					err
				);

				return Err(err);
			}
			info!("Created config directory {}", self.config.display());
		}

		Ok(())
	}
}
