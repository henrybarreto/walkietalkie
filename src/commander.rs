pub mod command;
pub mod commander_config;


use std::{
    error::Error,
    fs::File,
    net::{Shutdown, TcpListener, TcpStream},
    path::Path,
    sync::mpsc::{channel, Receiver, Sender},
};

use crate::reporter::report::Report;
use crate::{radio::Radio};
use commander_config::CommanderConfig;

use command::Command;

/// Represents methods what to open the connection to commander
#[derive(Clone, Debug)]
pub struct Commander {
    pub config: CommanderConfig,
}
impl Commander {
    /// Define the configuration to the server
    pub fn new(config: CommanderConfig) -> Self {
        Commander { config }
    }
    /**
       Loading a configuration file called "config.ron" containing a representation of CommanderConfig.

       It's panic if the file could not be open, if the file does not exists, if the content was
       not a SoldierConfig structure or it could not be deserialized.
    */
    fn load_config_file(path_config_file: String) -> File {
        if let Ok(config_file) = File::open(Path::new(&path_config_file)) {
            config_file
        } else {
            panic!("Could not read the config.ron file");
        }
    }
    fn convert_config_to_struct(config_file: File) -> CommanderConfig {
        match ron::de::from_reader(config_file) {
            Ok(commander_config) => commander_config,
            Err(error) => {
                panic!(
                    "Could not deserialize the config.ron file to Config: {}",
                    error
                )
            }
        }
    }
    pub fn config() -> CommanderConfig {
        Self::convert_config_to_struct(Self::load_config_file("config.ron".to_string()))
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

impl Radio<'static, Report, Command> for Commander {}
