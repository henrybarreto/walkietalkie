pub mod report;
pub mod soldier_config;

use log::{info, warn};
use std::{fs::File, path::Path};

use crate::commander::command::Command;
use report::Report;
use soldier_config::SoldierConfig;
pub struct Soldier;
impl Soldier {
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
