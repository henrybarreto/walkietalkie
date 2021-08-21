pub mod soldier_config;

use std::{
    error::Error,
    fs::File,
    net::{Shutdown, TcpListener, TcpStream},
    path::Path,
    sync::mpsc::{channel, Receiver, Sender},
};
use log::info;

use crate::radio::Radio;
use crate::reporter::report::Report;
use soldier_config::SoldierConfig;

use crate::commander::command::Command;
use std::process::{Output};

/// Represents methods what to open the connection to soldier
#[derive(Clone, Debug)]
pub struct Soldier {
    pub config: SoldierConfig,
}
impl Soldier {
    /// Define the configuration to the server
    pub fn new(config: SoldierConfig) -> Self {
        Soldier { config }
    }
    /**
       Loading a configuration file called "commander.ron" containing a representation of CommanderConfig.

       It's panic if the file could not be open, if the file does not exists, if the content was
       not a SoldierConfig structure or it could not be deserialized.
    */
    fn load_config_file(path_config_file: String) -> File {
        if let Ok(config_file) = File::open(Path::new(&path_config_file)) {
            config_file
        } else {
            panic!("Could not read the soldier.ron file");
        }
    }
    fn convert_config_to_struct(config_file: File) -> SoldierConfig {
        match ron::de::from_reader(config_file) {
            Ok(commander_config) => commander_config,
            Err(error) => {
                panic!(
                    "Could not deserialize the soldier.ron file to Config: {}",
                    error
                )
            }
        }
    }
    pub fn config() -> SoldierConfig {
        Self::convert_config_to_struct(Self::load_config_file("soldier.ron".to_string()))
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
            status: output_from_command.status.code().unwrap() as u8,
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
                Self::create_report_from_output(Self::run_command(command.clone()))
            })
            .collect()
    }

    /// Listen for a tcp connection
    pub fn listen(&self) -> Result<TcpListener, impl Error> {
        match TcpListener::bind(self.config.addr.clone()) {
            Ok(listener) => Ok(listener),
            Err(error) => Err(error),
        }
    }
    pub fn channel() -> (Sender<Vec<Report>>, Receiver<Vec<Report>>) {
        channel()
    }

    /// Disconnect a tcp connection
    pub fn disconnect(tcp_stream: &TcpStream) {
        tcp_stream.shutdown(Shutdown::Both).unwrap();
    }
}

impl Radio<'static, Report, Command> for Soldier {}
