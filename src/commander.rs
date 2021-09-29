use std::error::Error;
use std::{fs::File, path::Path};

use crate::commander::command::Command;
use crate::radio::Radio;
use crate::report::Report;
use commander_config::CommanderConfig;
use log::{debug, info};
use std::net::{Shutdown, TcpStream};

pub mod command;
pub mod commander_config;

/// Represents methods to open a connection to Soldier
pub struct Commander;

impl Commander {
    /**
    It's panic if the file could not be open, if the file does not exists, if the content was
    not a CommanderConfig structure or it could not be deserialized.
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
    /// Loading a configuration file called "commander.ron" containing a CommanderConfig.
    pub fn config() -> CommanderConfig {
        Self::convert_config_to_struct(Self::load_config_file("commander.ron".to_string()))
    }
    /// Connecting to a Soldier to a IP address
    pub fn connect(addr: String) -> TcpStream {
        debug!("Trying to connect to the soldier {}", &addr);
        let tcp_stream = if let Ok(tcp_stream) = TcpStream::connect(addr) {
            tcp_stream
        } else {
            panic!("Could not connect with the soldier");
        };

        info!("Connected to the server");
        tcp_stream
    }
    /// Sending a list of Command to Soldier
    pub fn send_commands(
        tcp_connection: &mut TcpStream,
        commands: Vec<Command>,
    ) -> Result<bool, Box<dyn Error>> {
        debug!("Sending commands to soldier");
        Self::send_chucked(tcp_connection, bincode::serialize(&commands)?)
    }
    /// Receiving a list of Report from Soldier
    pub fn recv_reports(tcp_connection: &mut TcpStream) -> Result<Vec<Report>, Box<dyn Error>> {
        debug!("Trying receiving report from soldier");
        let reports: Vec<Report> = bincode::deserialize(&Self::receive_chucked(tcp_connection)?)?;
        Ok(reports)
    }

    /// Disconnect from a TcpStream
    pub fn disconnect(tcp_connection: &TcpStream) {
        debug!("Disconnecting from the stream");
        tcp_connection.shutdown(Shutdown::Both).unwrap()
    }
}
impl Radio for Commander {}
