pub mod soldier_config;

use log::info;
use serde::{Deserialize, Serialize};
use std::{
    error::Error,
    fs::File,
    net::{Shutdown, TcpListener, TcpStream},
    os::unix::prelude::ExitStatusExt,
    path::Path,
    process::ExitStatus,
    sync::mpsc::{channel, Receiver, Sender},
};

use crate::radio::Radio;
use soldier_config::SoldierConfig;

use crate::commander::command::Command;
use crate::report::Report;
use std::process::Output;

/// Represents methods to listen connection from Commander
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Soldier {
    pub config: SoldierConfig,
}
impl Soldier {
    /// Create a new Soldier
    pub fn new(config: SoldierConfig) -> Self {
        Soldier { config }
    }
    /**
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
    /// Loading a configuration file called "soldier.ron" containing a SoldierConfig.
    pub fn config() -> SoldierConfig {
        Self::convert_config_to_struct(Self::load_config_file("soldier.ron".to_string()))
    }

    fn run_command(command: Command) -> Result<Output, impl Error> {
        match std::process::Command::new(command.name)
            .args(command.args)
            .output()
        {
            Ok(output_from_command) => Ok(output_from_command),
            Err(error) => {
                info!("Could not execute a command");
                Err(error)
            }
        }
    }

    fn create_report_from_output(&self, output_from_command: Output) -> Report {
        Report {
            soldier: self.clone(),
            status: output_from_command.status.code().unwrap() as u8,
            stdout: output_from_command.stdout,
            stderr: output_from_command.stderr,
        }
    }

    /// Run all commands and return a list of Report with the Command output or a empty Report if command fail.
    pub fn run_commands(&self, commands: Vec<Command>) -> Vec<Report> {
        info!("Running commands");
        let reports: Vec<Report> = commands
            .into_iter()
            .map(|command| {
                info!("Trying to executing a command");
                let output = match Self::run_command(command.clone()) {
                    Ok(output) => output,
                    Err(_) => Output {
                        status: ExitStatus::from_raw(1),
                        stdout: vec![],
                        stderr: vec![],
                    },
                };
                self.create_report_from_output(output)
            })
            .collect();
        reports
    }
    /// Send a list of Report to Commander
    pub fn send_reports(
        tcp_connection: &mut TcpStream,
        reports: Vec<Report>,
    ) -> Result<bool, Box<dyn Error>> {
        info!("Sending reports to commander...");
        Soldier::send_chucked(tcp_connection, bincode::serialize(&reports)?)
    }
    /// Receive a list of Commander
    pub fn recv_commands(tcp_connection: &mut TcpStream) -> Result<Vec<Command>, Box<dyn Error>> {
        info!("Receiving commands from commander...");
        let commands: Vec<Command> =
            bincode::deserialize(&Soldier::receive_chucked(tcp_connection)?)?;
        Ok(commands)
    }

    /// Listen a TcpStream
    pub fn listen(&self) -> Result<TcpListener, impl Error> {
        match TcpListener::bind(self.config.addr.clone()) {
            Ok(listener) => Ok(listener),
            Err(error) => Err(error),
        }
    }

    /// Return a formatted channel()
    pub fn channel() -> (Sender<Vec<Report>>, Receiver<Vec<Report>>) {
        channel()
    }

    /// Disconnect a TcpStream
    pub fn disconnect(tcp_stream: &TcpStream) {
        tcp_stream.shutdown(Shutdown::Both).unwrap();
    }
}

impl Radio for Soldier {}
