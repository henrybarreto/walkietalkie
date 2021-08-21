
use std::{fs::File, path::Path};



use commander_config::CommanderConfig;








pub mod command;
pub mod commander_config;

/// Represents the methods needed to execute operation on client side
pub struct Commander;

impl Commander {
    /**
    Loading a configuration file called "commander.ron" containing a representation of SoldierConfig.

    It's panic if the file could not be open, if the file does not exists, if the content was
    not a SoldierConfig structure or it could not be deserialized.

     */
    fn load_config_file(path_config_file: String) -> File {
        if let Ok(config_file) = File::open(Path::new(&path_config_file)) {
            config_file
        } else {
            panic!("Could not read the commander.ron file");
        }
    }
    fn convert_config_to_struct(config_file: File) -> CommanderConfig {
        match ron::de::from_reader(config_file) {
            Ok(soldier_config) => soldier_config,
            Err(error) => {
                panic!(
                    "Could not deserialize the commander.ron file to Config: {}",
                    error
                )
            }
        }
    }
    pub fn config() -> CommanderConfig {
        Self::convert_config_to_struct(Self::load_config_file("commander.ron".to_string()))
    }
}
