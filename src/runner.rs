use crate::config::Config;
use comfy_table::{Cell, Color, Table};
use process::Command;
use std::process;
use tracing::{error, info, warn};

pub struct Runner;

impl Runner {
	pub fn run(config: &Config, script_name: &str) {
		let Some(script_def) = config.find_script(script_name) else {
			warn!("Script '{}' not found", script_name);
			process::exit(1);
		};

		info!("Running Script: {}", script_name);

		for step in &script_def.steps {
			info!("Executing: {}", step.run);

			let status = match Command::new("sh").arg("-c").arg(&step.run).status() {
				Ok(status) => status,
				Err(err) => {
					error!("Failed to execute '{}': {}", step.run, err);
					process::exit(1);
				}
			};

			if !status.success() {
				warn!("Command failed with exit code: {:?}", status.code());
				process::exit(status.code().unwrap_or(1));
			}
		}
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

		println!("{}", table);
	}
}
