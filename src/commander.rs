pub mod command;
pub mod commander_config;

use crate::config::Config;
use crate::devices::Soldier;
use crate::radio::Radio;
use crate::report::Report;
use crate::seal::Seal;

use commander_config::CommanderConfig;
use log::{info, trace};
use ron::ser::PrettyConfig;
use std::error::Error;
use std::io::{Read, Write};
use std::net::{Shutdown, TcpStream};
use std::{fs::File, path::Path};

pub struct Commander {}

impl Commander {
    /// Connect to a Soldier with the given IP address.
    pub fn connect(addr: String) -> Result<TcpStream, impl Error> {
        trace!("Trying to connect to the soldier {}", &addr);
        TcpStream::connect(addr)
    }
    /// Send a list of Command to Soldier.
    pub fn send_commands(
        tcp_connection: &mut TcpStream,
        commands: Vec<String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        trace!("Sending commands to soldier");
        Self::send_chucked(tcp_connection, bincode::serialize(&commands)?)
    }
    /// Receive a list of Report from Soldier.
    pub fn recv_reports(
        tcp_connection: &mut TcpStream,
    ) -> Result<Vec<Report>, Box<bincode::ErrorKind>> {
        trace!("Trying receiving report from soldier");
        // TODO Here has a problem with question mark...
        let mut file = File::open(Self::receive_chucked(tcp_connection).unwrap()).unwrap();
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer);
        bincode::deserialize::<Vec<Report>>(&buffer)
    }

    /// Disconnect from a Soldier.
    pub fn disconnect(tcp_connection: &TcpStream) {
        info!("Disconnecting from the stream");
        tcp_connection.shutdown(Shutdown::Both).unwrap()
    }
}
impl Config<CommanderConfig> for Commander {
    /// Generate a CommanderConfig file with a default value.
    fn generate_config(path: &Path) {
        let config = CommanderConfig {
            name: "Cpt. Steven Rogers".to_string(),
            soldiers: vec![Soldier {
                address: "127.0.0.1:14114".to_string(),
                seal: Seal {
                    username: "".to_string(),
                    password: "".to_string(),
                },
            }],
            commands: vec!["echo".to_string(), "curl".to_string()],
        };
        let string = ron::ser::to_string_pretty(&config, PrettyConfig::default()).unwrap();
        let mut file = File::create(path).unwrap();
        file.write_all(string.as_bytes());
    }
}
impl Radio for Commander {}
