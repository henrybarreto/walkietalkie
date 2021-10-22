pub mod soldier_config;

use log::trace;
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
use crate::config::Config;
use crate::report::Report;
use std::io::{Read, Write};
use std::process::Output;

use crate::seal::Seal;
use ron::ser::PrettyConfig;

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

    fn run_command(command: Command) -> Result<Output, impl Error> {
        std::process::Command::new(command.name)
            .args(command.args)
            .output()
    }

    fn create_report_from_output(&self, output_from_command: Output) -> Report {
        Report {
            soldier: self.config.addr.to_string(),
            status: output_from_command.status.code().unwrap() as u8,
            stdout: output_from_command.stdout,
            stderr: output_from_command.stderr,
        }
    }

    /// Run all commands and return a list of Report with the Command output or a empty Report if command fail.
    pub fn run_commands(&self, commands: Vec<Command>) -> Vec<Report> {
        trace!("Running all commands");
        commands
            .into_iter()
            .map(|command| {
                trace!("Trying to executing a command {}",command.clone());
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
            .collect()
    }
    /// Send a list of Report to Commander
    pub fn send_reports(
        tcp_connection: &mut TcpStream,
        reports: Vec<Report>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        trace!("Sending reports to commander...");
        Soldier::send_chucked(tcp_connection, bincode::serialize(&reports)?)
    }
    /// Receive a list of Commander
    pub fn receive_commands(
        tcp_connection: &mut TcpStream,
    ) -> Result<Vec<Command>, Box<dyn std::error::Error>> {
        trace!("Receiving commands from commander...");
        trace!("Receiving Data");
        let mut file = File::open(Soldier::receive_chucked(tcp_connection)?).unwrap();
        let mut buffer = Vec::new();
        let i = file.read_to_end(&mut buffer)?;
        println!("File lenth {}",i);
        trace!("Parsing Data");

        let commands: Vec<Command> =
            bincode::deserialize(&*buffer)?;
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

impl Config<SoldierConfig> for Soldier {
    fn generate_config(path: &Path) {
        let config = SoldierConfig {
            name: "S. Buck".to_string(),
            addr: "127.0.0.1:14114".to_string(),
            group: "".to_string(),
            user: "".to_string(),
            seal: Seal {
                username: "".to_string(),
                password: "".to_string(),
            },
        };
        let string = ron::ser::to_string_pretty(&config, PrettyConfig::default()).unwrap();
        let mut file = File::create(path).unwrap();
        file.write_all(string.as_bytes());
    }
}

impl Radio for Soldier {}
