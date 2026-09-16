use serde::Deserialize;
use std::{
	collections::BTreeMap,
	fs, io,
	path::{Path, PathBuf},
};
use tracing::warn;

#[derive(Debug, Deserialize)]
pub struct Step {
	pub run: String,
	#[allow(dead_code)]
	pub os: Option<Vec<String>>,
	#[allow(dead_code)]
	pub host: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct Script {
	pub description: String,
	pub steps: Vec<Step>,
	#[allow(dead_code)]
	pub os: Option<Vec<String>>,
	#[allow(dead_code)]
	pub host: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Default)]
pub struct Config {
	#[allow(dead_code)]
	pub log_level: Option<String>,
	#[serde(default)]
	pub scripts: BTreeMap<String, Script>,
}

impl Config {
	pub fn load_from_dir(dir: &Path) -> anyhow::Result<Self> {
		let files = Self::discover_files(dir)?;
		let mut combined_config = Config::default();

		for file in files {
			match Self::load_from_file(&file) {
				Ok(cfg) => {
					combined_config.scripts.extend(cfg.scripts);
				}
				Err(err) => warn!("Could not load {}: {}", file.display(), err),
			}
		}
		Ok(combined_config)
	}
	fn discover_files(dir: &Path) -> io::Result<Vec<PathBuf>> {
		let mut results = Vec::new();
		Self::find_toml_files(dir, &mut results)?;

		if results.is_empty() {
			let hello_path = dir.join("hello.toml");
			const HELLO_TOML: &str = r#"# Sample hello.toml configuration file
[scripts.hello]
description = "A simple hello world script"
steps = [
	{ run = "echo 'Hello, world!'" }
]
"#;

			fs::write(&hello_path, HELLO_TOML)?;
			results.push(hello_path);
		}

		Ok(results)
	}

	fn find_toml_files(dir: &Path, results: &mut Vec<PathBuf>) -> io::Result<()> {
		for entry in fs::read_dir(dir)? {
			let entry = entry?;
			let path = entry.path();
			if path.is_dir() {
				Self::find_toml_files(&path, results)?;
			} else if path.extension().and_then(|s| s.to_str()) == Some("toml") {
				results.push(path);
			}
		}
		Ok(())
	}

	fn load_from_file(path: &Path) -> anyhow::Result<Self> {
		let content = fs::read_to_string(path)?;
		let config = toml::from_str(&content)?;
		Ok(config)
	}
}
