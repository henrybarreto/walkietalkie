pub mod soldier_config;

use log::{info, trace};
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
    fn load_config_file(path_config_file: String) -> File {
        File::open(Path::new(&path_config_file)).expect("Could not read the soldier.ron file")
    }
    fn convert_config_to_struct(config_file: File) -> SoldierConfig {
        ron::de::from_reader(config_file)
            .expect("Could not deserialize the soldier.ron file to Config: {}")
    }
    /// Loading a configuration file called "soldier.ron" containing a SoldierConfig.
    pub fn config() -> SoldierConfig {
        Self::convert_config_to_struct(Self::load_config_file("soldier.ron".to_string()))
    }

    fn run_command(command: Command) -> Result<Output, impl Error> {
        std::process::Command::new(command.name)
            .args(command.args)
            .output()
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
        trace!("Running commands");
        let reports: Vec<Report> = commands
            .into_iter()
            .map(|command| {
                trace!("Trying to executing a command");
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
    ) -> Result<bool, Box<dyn std::error::Error>> {
        info!("Sending reports to commander...");
        Soldier::send_chucked(tcp_connection, bincode::serialize(&reports)?)
    }
    /// Receive a list of Commander
    pub fn receive_commands(
        tcp_connection: &mut TcpStream,
    ) -> Result<Vec<Command>, Box<dyn std::error::Error>> {
        info!("Receiving commands from commander...");
        let commands: Vec<Command> =
            bincode::deserialize(&Soldier::receive_chucked(tcp_connection)?)?;
        Ok(commands)
    }

    /// Listen a TcpStream
    pub fn listen(&self) -> TcpListener {
        TcpListener::bind(self.config.addr.clone()).expect("Could not listen to address")
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
