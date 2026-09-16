use std::process::Command;

use crate::config::Config;
use anyhow::{Context, Result, bail};
use tracing::{info, warn};

pub struct Runner;

impl Runner {
	pub fn run(config: &Config, script_name: &str) -> Result<()> {
		let script_def = config
			.find_script(script_name)
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
		let has_local = !config.local_scripts.is_empty();
		let has_global = !config.global_scripts.is_empty();

		if !has_local && !has_global {
			warn!("No scripts found in configuration files.");
			return;
		}

		if has_local {
			println!("Project Scripts (.rrun):");
			for (name, script) in &config.local_scripts {
				println!("  {}: {}", name, script.description);
			}
		}

		if has_global {
			println!("Global scripts:");
			for (name, script) in &config.global_scripts {
				println!("  {}: {}", name, script.description);
			}
		}
	}
}
