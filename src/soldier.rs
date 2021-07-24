use std::process::Output;
use std::{fs::File, path::Path};

use log::{info};

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
    fn load_config_file(path_config_file: String) -> File {
        if let Ok(config_file) = File::open(Path::new(&path_config_file)) {
            config_file
        } else {
            panic!("Could not read the config.ron file");
        }
    }
    fn convert_config_to_struct(config_file: File) -> SoldierConfig {
        match ron::de::from_reader(config_file) {
            Ok(soldier_config) => soldier_config,
            Err(error) => {
                panic!(
                    "Could not deserialize the config.ron file to Config: {}",
                    error
                )
            }
        }
    }
    pub fn config() -> SoldierConfig {
        Self::convert_config_to_struct(Self::load_config_file("config.ron".to_string()))
    }

    fn run_command(command: Command) -> Output {
        match std::process::Command::new(command.name)
            .args(command.args)
            .output()
        {
            Ok(output_from_command) => output_from_command,
            Err(_) => {
                panic!("Could not execute a command")
            }
        }
    }

    fn create_report_from_output(output_from_command: Output) -> Report {
        Report {
            status: output_from_command.status.code().unwrap(),
            stdout: output_from_command.stdout,
            stderr: output_from_command.stderr,
        }
    }

    /// Run a command and return a Report with the result.
    pub fn run_commands(commands: Vec<Command>) -> Vec<Report> {
        info!("Running commands");
        commands
            .into_iter()
            .map(|command| {
                info!("Trying to executing the commands");
                Soldier::create_report_from_output(Soldier::run_command(command.clone()))
            })
            .collect()
    }
}
