pub mod command;
pub mod commander_config;

use commander_config::CommanderConfig;
use std::{fs::File, path::Path};
/// Represents the methods needed to execute operations on server side
pub struct Commander;
impl Commander {
    /**
        Loading a configuration file called "config.ron" containing a representation of CommanderConfig.

        It's panic if the file could not be open, if the file does not exists, if the content was
        not a SoldierConfig structure or it could not be deserialized.
     */
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
