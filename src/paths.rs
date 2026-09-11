use std::fs;
use std::io;
use std::path::Path;
use std::path::PathBuf;
use tracing::debug;

pub struct AppPaths {
	config: PathBuf,
}

impl AppPaths {
	pub fn init() -> io::Result<Self> {
		let config = dirs::config_dir()
			.map(|p| p.join("rrun"))
			.unwrap_or_else(|| PathBuf::from(".rrun"));

		let paths = Self { config };
		paths.ensure()?;

		Ok(paths)
	}

	fn ensure(&self) -> io::Result<()> {
		fs::create_dir_all(&self.config)?;
		debug!(
			"Ensured config directory exists at {}",
			self.config.display()
		);

		Ok(())
	}

	pub fn _config_dir(&self) -> &Path {
		&self.config
	}
}
