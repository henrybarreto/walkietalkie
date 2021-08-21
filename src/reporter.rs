use std::net::{Shutdown, TcpStream};

use log::info;

use report::Report;

use crate::{radio::Radio, commander::command::Command};

pub mod report;

/// Represents methods to connect to the soldier
pub struct Reporter;
impl Reporter {
    /// Connect to a address server
    pub fn connect(addr: String) -> TcpStream {
        info!("Trying to connect to the server {}", &addr);
        let tcp_stream = if let Ok(tcp_stream) = TcpStream::connect(addr) {
            tcp_stream
        } else {
            panic!("Could not connect with the soldier server");
        };

        info!("Connected to the server");
        tcp_stream
    }

    /// Disconnect a tcp connection
    pub fn disconnect(tcp_stream: &TcpStream) {
        info!("Disconnecting from the stream");
        tcp_stream.shutdown(Shutdown::Both).unwrap()
    }
}

impl Radio<'static, Command, Report> for Reporter {}
