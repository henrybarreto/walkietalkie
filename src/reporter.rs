use std::{
    io::{Read, Write},
    net::{Shutdown, TcpStream},
    process,
};

use log::{error, info};

use report::Report;

use crate::commander::command::Command;
use crate::communication::Communication;

pub mod report;

/// Represents methods what works for the network communication.
pub struct Reporter;
impl Reporter {
    /// Connect to a address server
    pub fn connect(addr: String) -> TcpStream {
        info!("Trying to connect to the server {}", &addr);
        let tcp_stream = if let Ok(tcp_stream) = TcpStream::connect(addr) {
            tcp_stream
        } else {
            panic!("Could not connect with the commander server");
        };

        info!("Connected to the server");
        tcp_stream
    }

    /// Send a Report to a tcp connection
    pub fn send_report(tcp_stream: &mut TcpStream, reports: Vec<Report>) -> Result<usize, String> {
        info!("Trying to serialize the reports");
        match Report::from_vec_to_bytes(reports) {
            Ok(outputs_serialized) => {
                info!("Reports serialized");
                let reports_size: usize = outputs_serialized.len();
                let serialized_reports_size = bincode::serialize(&reports_size).unwrap();
                info!("Sending size...");
                tcp_stream.write(&serialized_reports_size).unwrap();
                let mut serialized_reports_status = [0];
                tcp_stream.read(&mut serialized_reports_status).unwrap();
                if serialized_reports_status == [0] {
                    error!("Status from the size of reports is not valid");
                    process::exit(1);
                }
                match tcp_stream.write(&outputs_serialized) {
                    Ok(buf_wrote) => {
                        info!("Bytes wrote to the stream");
                        return Ok(buf_wrote);
                    }
                    Err(_) => {
                        return Err("Could not write in the stream".to_string());
                    }
                }
            }
            Err(e) => {
                return Err(format!("{}", e));
            }
        }
    }

    /// Receive commands from a tcp connection
    pub fn receive_commands(tcp_stream: &mut TcpStream) -> Result<Vec<Command>, String> {
        info!("Trying to get bytes from the stream");
        let mut buf_size_of_commands = vec![0; 512];
        tcp_stream
            .read(&mut buf_size_of_commands)
            .expect("Could not read from the steam the size of commands");
        tcp_stream
            .write(&[1])
            .expect("Could not read the status of size of commands from the stream"); // Okay // TODO Check and return either [0] or [1]
        let size_of_commands = bincode::deserialize(&buf_size_of_commands)
            .expect("Could not deserialize the size of commands");
        let mut buf_commands = vec![0; size_of_commands];
        info!("Trying to read the commands from the stream");
        match tcp_stream.read(&mut buf_commands) {
            Ok(_buf_read) => {
                info!("Trying to deserialize the commands");
                match Command::from_bytes_to_vec(buf_commands) {
                    Ok(list_commands) => {
                        info!("Deserialized!");
                        let commands: Vec<Command> = list_commands;

                        info!("Returning commands deserialized");
                        return Ok(commands);
                    }
                    Err(_) => {
                        return Err(
                            "Could not deserialize the commands from the stream".to_string()
                        );
                    }
                }
            }
            Err(_) => {
                return Err("Could not read from stream".to_string());
            }
        }
    }

    /// Disconnect a tcp connection
    pub fn disconnect(tcp_stream: &TcpStream) {
        info!("Disconnecting from the stream");
        tcp_stream.shutdown(Shutdown::Both).unwrap()
    }
}
