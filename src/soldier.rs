use std::{fs::File, path::Path};

use log::{info, warn};

use soldier_config::SoldierConfig;

use crate::commander::command::Command;
use crate::reporter::report::Report;

pub mod soldier_config;

/// Represents the methods needed to execute operation on client side
pub struct Soldier;
impl Soldier {
    /**
       Loading a configuration file called "config.ron" containing a representation of SoldierConfig.

       It's panic if the file could not be open, if the file does not exists, if the content was
       not a SoldierConfig structure or it could not be deserialized.

    */
    pub fn config() -> SoldierConfig {
        let config_file = if let Ok(config_file) = File::open(Path::new("config.ron")) {
            config_file
        } else {
            panic!("Could not read the config.ron file");
        };
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

    /// Run a command and return a Report with the result.
    pub fn run_commands(commands: Vec<Command>) -> Vec<Report> {
        info!("Running commands");
        let mut list_responses_from_commands: Vec<Report> = vec![];
        for command in commands {
            info!("Trying to executing the commands");
            let handle = if let Ok(output_from_a_command) = std::process::Command::new(command.name)
                .args(command.args)
                .output()
            {
                info!("Command executed!");
                output_from_a_command
            } else {
                warn!("A command could not be performed");
                break;
            };
            let response_from_command = Report {
                status: handle.status.code().unwrap(),
                stdout: handle.stdout,
                stderr: handle.stderr,
            };
            //info!("{:#?}", response_from_command);
            list_responses_from_commands.push(response_from_command);
        }
        list_responses_from_commands
    }
}
