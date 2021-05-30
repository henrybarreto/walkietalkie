use log::info;
use std::{
    io::{Read, Write},
    net::{Shutdown, TcpStream},
};

use crate::commander::command::Command;
use crate::communication::Communication;
use crate::soldier::report::Report;

pub struct Reporter;
impl Reporter {
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

    pub fn send_report(mut tcp_stream: &TcpStream, reports: Vec<Report>) -> Result<usize, String> {
        info!("Trying to serialize the reports");
        match Report::from_vec_to_bytes(reports) {
            Ok(outputs_serialized) => {
                info!("Reports serialized");

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

    pub fn receive_commands(mut tcp_stream: &TcpStream) -> Result<Vec<Command>, String> {
        let mut buf = [0 as u8; 1024];
        info!("Trying to read from the stream");
        match tcp_stream.read(&mut buf) {
            Ok(_buf_read) => {
                info!("Trying to deserialize the commands");
                match Command::from_bytes_to_vec(buf.to_vec()) {
                    Ok(list_commands) => {
                        info!("Deserialized!");
                        let commands: Vec<Command> = list_commands;

                        info!("Returning commands deserialized");
                        return Ok(commands);
                    }
                    Err(_) => {
                        return Err("Could not deserilize the commands from the stream".to_string());
                    }
                }
            }
            Err(_) => {
                return Err("Could not read from stream".to_string());
            }
        }
    }

    pub fn desconnect(tcp_stream: &TcpStream) {
        info!("Desconnecting from the stream");
        tcp_stream.shutdown(Shutdown::Both).unwrap()
    }
}
