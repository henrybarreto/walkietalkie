use std::error::Error;
use std::{fs::File, path::Path};

use crate::commander::command::Command;
use crate::radio::Radio;
use crate::report::Report;
use commander_config::CommanderConfig;
use log::{info, trace};
use std::net::{Shutdown, TcpStream};

use crate::config::Config;
use crate::devices::Device;
use crate::seal::Seal;
use ron::ser::PrettyConfig;
use std::io::{Read, Write};

pub mod command;
pub mod commander_config;

pub struct Commander {}

impl Commander {
    /// Connecting to a Soldier to a IP address
    pub fn connect(addr: String) -> Result<TcpStream, impl Error> {
        trace!("Trying to connect to the soldier {}", &addr);
        TcpStream::connect(addr)
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
        let mut file = Self::receive_chucked(tcp_connection).unwrap();
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer);
        bincode::deserialize::<Vec<Report>>(&buffer)
    }

    /// Disconnect from a TcpStream
    pub fn disconnect(tcp_connection: &TcpStream) {
        info!("Disconnecting from the stream");
        tcp_connection.shutdown(Shutdown::Both).unwrap()
    }
}
impl Config<CommanderConfig> for Commander {
    fn generate_config(path: &Path) {
        let config = CommanderConfig {
            name: "Cpt. Steven Rogers".to_string(),
            devices: vec![Device {
                address: "127.0.0.1:14114".to_string(),
                seal: Seal {
                    username: "".to_string(),
                    password: "".to_string(),
                },
            }],
            commands: vec![],
        };
        let string = ron::ser::to_string_pretty(&config, PrettyConfig::default()).unwrap();
        let mut file = File::create(path).unwrap();
        file.write_all(string.as_bytes());
    }
}
impl Radio for Commander {}
