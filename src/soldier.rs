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

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Soldier {
    pub config: SoldierConfig,
}

impl Soldier {
    /// Creates a new Soldier. That server will receive the Commands from a Commander.
    pub fn new(config: SoldierConfig) -> Self {
        Soldier { config }
    }

    /// Runs the command.
    fn run_command(command: Command) -> Result<Output, impl Error> {
        // TODO: Add a time limit to command execute.
        trace!("Running a single command: {}", command.clone());
        std::process::Command::new(command.name)
            .args(command.args)
            .output()
    }

    /// Creates a simple output from the command executed.
    fn create_report_from_output(&self, output_from_command: Output) -> Report {
        Report {
            soldier: self.config.addr.to_string(),
            status: output_from_command.status.code().unwrap() as u8,
            stdout: output_from_command.stdout,
            stderr: output_from_command.stderr,
        }
    }

    /// Runs all commands and return a list of Report with the Command output or a empty Report if
    /// command fail.
    pub fn run_commands(&self, commands: Vec<String>) -> Vec<Report> {
        trace!("Running all commands");
        trace!("Commands: {:?}", self.config.commands);
        self.config
            .commands
            .iter()
            .filter(|command| commands.contains(&command.id))
            .map(|command| {
                trace!("Trying to executing a command {}", command.clone());
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
    /// Sends a list of Report to Commander.
    pub fn send_reports(
        tcp_connection: &mut TcpStream,
        reports: Vec<Report>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        trace!("Sending reports to commander...");
        Soldier::send_chucked(tcp_connection, bincode::serialize(&reports)?)
    }

    /// Receives a list of commands.
    pub fn receive_commands(
        tcp_connection: &mut TcpStream,
    ) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        trace!("Receiving commands from commander...");
        trace!("Receiving Data");
        let mut file = File::open(Soldier::receive_chucked(tcp_connection)?).unwrap();
        let mut buffer = Vec::new();
        let i = file.read_to_end(&mut buffer)?;
        println!("File lenth {}", i);
        trace!("Parsing Data");

        let commands: Vec<String> = bincode::deserialize(&*buffer)?;
        trace!("Received commands: {:?}", commands);
        Ok(commands)
    }

    /// Listen set the Soldier to wait to Commander connections.
    pub fn listen(&self) -> TcpListener {
        TcpListener::bind(self.config.addr.clone()).expect("Could not listen to address")
    }

    /// Gets a formatted channel()
    pub fn channel() -> (Sender<Vec<Report>>, Receiver<Vec<Report>>) {
        channel()
    }

    /// Disconnects the Soldier from the Commander.
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
            commands: vec![Command {
                id: "echo".to_string(),
                name: "echo".to_string(),
                args: vec!["Hello World!".to_string()],
            }],
        };
        let string = ron::ser::to_string_pretty(&config, PrettyConfig::default()).unwrap();
        let mut file = File::create(path).unwrap();
        file.write_all(string.as_bytes());
    }
}

impl Radio for Soldier {}
