pub mod soldier_config;

use std::{error::Error, fs::File, net::{Shutdown, TcpListener, TcpStream}, os::unix::prelude::ExitStatusExt, path::Path, process::{ExitStatus}, sync::mpsc::{channel, Receiver, Sender}};
use serde::{Deserialize, Serialize};
use log::info;

use crate::radio::Radio;
use soldier_config::SoldierConfig;

use crate::commander::command::Command;
use std::process::{Output};
use crate::report::Report;

/// Represents methods what to open the connection to soldier
#[derive(Serialize, Deserialize, Clone, Debug)]
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

    /// Run a command and return a Report with the command output or a empty Report if command fail.
    pub fn run_commands(&self, commands: Vec<Command>) -> Vec<Report> {
        info!("Running commands");
        let reports: Vec<Report> = commands
            .into_iter()
            .map(|command| {
                info!("Trying to executing a command");
                let output = match Self::run_command(command.clone()) {
                    Ok(output) => output,
                    Err(_) => {
                                    Output {
                                        status: ExitStatus::from_raw(1),
                                        stdout: vec![],
                                        stderr: vec![],
                                    }
                                }
                };
                self.create_report_from_output(output)
            })
            .collect();
        reports
    }

    pub fn send_reports(tcp_connection: &mut TcpStream, reports: Vec<Report>) -> Result<bool, Box<dyn Error>> {
        info!("Sending reports to commander...");
        Soldier::send_information(tcp_connection, reports)
    }

    pub fn recv_commands(tcp_connection: &mut TcpStream) -> Result<Vec<Command>, Box<dyn Error>> {
        info!("Receiving commands from commander...");
        Soldier::receive_information(tcp_connection)
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

impl Radio<'static, Command, Report> for Soldier {}
