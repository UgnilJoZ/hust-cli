use hust::Bridge;
use hust::error::Result;
use std::collections::HashMap;
use std::env::var;
use std::fs::File;
use std::path::{Path, PathBuf};

pub fn get_config_file() -> PathBuf {
	let config_dir = match var("XDG_CONFIG_HOME") {
		Ok(path) => Path::new(&path).to_owned(),
		Err(_) => match var("HOME") {
			Ok(path) => Path::new(&path).join(".config"),
			Err(_) => PathBuf::new(),
		}
	};
	return Path::new(&config_dir).join(".hustrc");
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct Config {
    pub bridges: Vec<Bridge>,
    pub usernames: HashMap<String, String>,
}

impl Config {
    pub fn load() -> Result<Config> {
		let file = File::open(get_config_file())?;
        Ok(serde_json::from_reader(file)?)
	}
	
	pub fn save(&self) -> Result<()> {
		let mut file = File::create(get_config_file())?;
		serde_json::to_writer_pretty(&mut file, &self)?;
		Ok(())
	}
}
