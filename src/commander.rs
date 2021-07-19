pub mod command;
pub mod commander_config;

use log::{error};
use std::{
    error::Error,
    fs::File,
    io::{Read, Write},
    net::{Shutdown, TcpListener, TcpStream},
    path::Path,
    process,
    sync::mpsc::{channel, Receiver, Sender},
};

use crate::communication::Communication;
use crate::reporter::report::Report;

use commander_config::CommanderConfig;

use command::Command;

/// Represents methods what works for the network communication.
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
    pub fn config() -> CommanderConfig {
        let config_file =
            File::open(Path::new("config.ron")).expect("Could not read the config.ron file");
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
    /// Send commands to the client execute
    pub fn send_commands(tcp_stream: &mut TcpStream, commands: Vec<Command>) {
        let buf_commands = if let Ok(buf) = Command::from_vec_to_bytes(commands) {
            buf
        } else {
            error!("Could not convert from Commands to bytes");
            Commander::disconnect(&tcp_stream);
            process::exit(1);
        };

        let size_of_commands = buf_commands.len();
        let serialized_size_of_commands = bincode::serialize(&size_of_commands)
            .expect("Could not serialize the size of commands");

        if let Err(_) = tcp_stream.write(&serialized_size_of_commands) {
            error!("Could not send the size of commands");
            Commander::disconnect(&tcp_stream);
        }
        let mut size_of_commands_status = [0];
        if let Err(_) = tcp_stream.read(&mut size_of_commands_status) {
            error!("Could not read the status of size of commands from the stream");
        }
        if size_of_commands_status == [0] {
            error!("Status from the size of commands is not valid");
            Commander::disconnect(&tcp_stream);
        }

        if let Err(_) = tcp_stream.write(&buf_commands) {
            error!("Could not write on the stream");
        }
    }

    /// Receive responses from executed commands in the client side
    pub fn receive_reports(tcp_stream: &mut TcpStream) -> Vec<Report> {
        let mut buf_size_reports = [0 as u8; 512];
        if let Err(_) = tcp_stream.read(&mut buf_size_reports) {
            error!("Could not read the size of commings reports");
        }
        let size_reports: usize = if let Ok(size_reports) = bincode::deserialize(&buf_size_reports) {
            size_reports
        } else {
            error!("Could not deserialize the size of reports");
            Commander::disconnect(tcp_stream);
            process::exit(1);
        };
        let mut buf_reports = vec![0 as u8; size_reports];
        let buf_reports_status = [1]; // Okay
        if let Err(_) = tcp_stream.write(&buf_reports_status) {
            error!("Could not write the status from the reports to the stream");
            Commander::disconnect(tcp_stream);
        }
        if let Err(_) = tcp_stream.read(&mut buf_reports) {
            error!("Cound not read the orders from stream");
            Commander::disconnect(tcp_stream);
        }

        let reports = if let Ok(reports) = Report::from_bytes_to_vec(buf_reports) {
            reports
        } else {
            error!("Could not convert from bytes to Commands");
            Commander::disconnect(tcp_stream);
            process::exit(1);
        };


        reports
    }
    /// Disconnect a tcp connection
    pub fn disconnect(tcp_stream: &TcpStream) {
        tcp_stream.shutdown(Shutdown::Both).unwrap();
    }
}
