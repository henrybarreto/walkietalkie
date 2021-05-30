pub mod command;
pub mod commander_config;

use commander_config::CommanderConfig;
use std::{fs::File, path::Path};

pub struct Commander;
impl Commander {
    pub fn config() -> CommanderConfig {
        let config_file =
            File::open(Path::new("config.ron")).expect("Could not read the config.ron file");
        match ron::de::from_reader(config_file) {
            Ok(commander_config) => commander_config,
            Err(error) => {
                panic!(
                    "Could not deserialize the config.ron file to Config: {}",
                    error
                )
            }
        }
    }
}
