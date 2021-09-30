use std::error::Error;
use std::{fs::File, path::Path};

use crate::commander::command::Command;
use crate::radio::Radio;
use crate::report::Report;
use commander_config::CommanderConfig;
use log::{debug, info, trace};
use std::net::{Shutdown, TcpStream};

use serde::de::StdError;

pub mod command;
pub mod commander_config;

/// Represents methods to open a connection to Soldier
pub struct Commander;

impl Commander {
    fn load_config_file(path_config_file: String) -> File {
        File::open(Path::new(&path_config_file)).expect("Could not read the commander.ron file")
    }
    fn convert_config_to_struct(config_file: File) -> CommanderConfig {
        ron::de::from_reader(config_file)
            .expect("Could not deserialize the commander.ron file to Config: {}")
    }
    /// Loading a configuration file called "commander.ron" containing a CommanderConfig.
    pub fn config() -> CommanderConfig {
        Self::convert_config_to_struct(Self::load_config_file("commander.ron".to_string()))
    }
    /// Connecting to a Soldier to a IP address
    pub fn connect(addr: String) -> TcpStream {
        trace!("Trying to connect to the soldier {}", &addr);
        let tcp_connection = TcpStream::connect(addr).expect("Could not connect to the soldier");
        info!("Connected to the server");

        tcp_connection
    }
    /// Sending a list of Command to Soldier
    pub fn send_commands(
        tcp_connection: &mut TcpStream,
        commands: Vec<Command>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        trace!("Sending commands to soldier");
        Self::send_chucked(tcp_connection, bincode::serialize(&commands)?)
    }
    /// Receiving a list of Report from Soldier
    pub fn recv_reports(
        tcp_connection: &mut TcpStream,
    ) -> Result<Vec<Report>, Box<bincode::ErrorKind>> {
        trace!("Trying receiving report from soldier");
        // TODO Here has a problem with question mark...
        bincode::deserialize::<Vec<Report>>(&Self::receive_chucked(tcp_connection).unwrap())
    }

    /// Disconnect from a TcpStream
    pub fn disconnect(tcp_connection: &TcpStream) {
        info!("Disconnecting from the stream");
        tcp_connection.shutdown(Shutdown::Write).unwrap()
    }
}
impl Radio for Commander {}
