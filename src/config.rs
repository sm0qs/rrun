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

#[derive(Debug, Default)]
pub struct Config {
	pub global_scripts: BTreeMap<String, Script>,
	pub local_scripts: BTreeMap<String, Script>,
}

#[derive(Debug, Deserialize, Default)]
struct ConfigFile {
	#[allow(dead_code)]
	pub log_level: Option<String>,
	#[serde(default)]
	pub scripts: BTreeMap<String, Script>,
}

impl Config {
	pub fn find_script(&self, name: &str) -> Option<&Script> {
		self.local_scripts
			.get(name)
			.or_else(|| self.global_scripts.get(name))
	}

	pub fn load(global_dir: &Path, local_dir: Option<&Path>) -> anyhow::Result<Self> {
		let global_scripts = Self::load_scripts_from_dir(global_dir, true)?;

		let local_scripts = match local_dir {
			Some(dir) => Self::load_scripts_from_dir(dir, false)?,
			None => BTreeMap::new(),
		};

		Ok(Self {
			global_scripts,
			local_scripts,
		})
	}

	fn load_scripts_from_dir(
		dir: &Path,
		create_sample_if_empty: bool,
	) -> anyhow::Result<BTreeMap<String, Script>> {
		let mut files = Vec::new();
		Self::find_toml_files(dir, &mut files)?;

		if files.is_empty() && create_sample_if_empty {
			let hello_path = dir.join("hello.toml");
			const HELLO_TOML: &str = r#"# Sample hello.toml configuration file
[scripts.hello]
description = "A simple hello world script"
steps = [
	{ run = "echo 'Hello, world!'" }
]
"#;

			fs::write(&hello_path, HELLO_TOML)?;
			files.push(hello_path);
		}
		let mut scripts = BTreeMap::new();
		for file in files {
			match Self::load_file(&file) {
				Ok(cfg) => {
					scripts.extend(cfg.scripts);
				}
				Err(err) => warn!("Could not load {}: {}", file.display(), err),
			}
		}

		Ok(scripts)
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

	fn load_file(path: &Path) -> anyhow::Result<ConfigFile> {
		let content = fs::read_to_string(path)?;
		let config = toml::from_str(&content)?;

		Ok(config)
	}
}
