use crate::config::Config;
use anyhow::{Context, Result, bail};
use comfy_table::{Cell, Color, Table};
use std::process::Command;
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

		let mut table = Table::new();
		table.set_header(vec![
			Cell::new("Name").fg(Color::Cyan),
			Cell::new("Description").fg(Color::Cyan),
			Cell::new("Scope").fg(Color::Cyan),
		]);

		for (name, script) in &config.local_scripts {
			table.add_row(vec![
				Cell::new(name).fg(Color::Green),
				Cell::new(&script.description),
				Cell::new("Project").fg(Color::Yellow),
			]);
		}

		for (name, script) in &config.global_scripts {
			table.add_row(vec![
				Cell::new(name).fg(Color::Green),
				Cell::new(&script.description),
				Cell::new("Global").fg(Color::Red),
			]);
		}

		println!("{table}");
	}
}
