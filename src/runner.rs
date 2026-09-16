use std::process::Command;

use crate::config::Config;
use anyhow::{Context, Result, bail};
use tracing::{info, warn};

pub struct Runner;

impl Runner {
	pub fn run(config: &Config, script_name: &str) -> Result<()> {
		let script_def = config
			.scripts
			.get(script_name)
			.ok_or_else(|| anyhow::anyhow!("Script '{}' not found", script_name))?;

		info!("Running Script: {}", script_name);

		for step in &script_def.steps {
			info!("Executing: {}", step.run);

			let status = Command::new("sh")
				.arg("-c")
				.arg(&step.run)
				.status()
				.with_context(|| format!("Failed to execute: {}", step.run))?;

			if !status.success() {
				bail!("Command failed with exit code: {:?}", status.code());
			}
		}
		Ok(())
	}

	pub fn list(config: &Config) {
		if config.scripts.is_empty() {
			warn!("No scripts found in configuration files.");
			return;
		}

		println!("Available scripts:");
		for (name, script) in &config.scripts {
			println!("- {}: {}", name, script.description);
		}
	}
}
