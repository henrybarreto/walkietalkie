use log::error;
use std::{
    error::Error,
    io::{Read, Write},
    net::{Shutdown, TcpListener, TcpStream},
    process,
    sync::mpsc::{channel, Receiver, Sender},
};

use crate::commander::command::Command;
use crate::commander::commander_config::CommanderConfig;
use crate::communication::Communication;
use crate::reporter::report::Report;

/// Represents methods what works for the network communication.
#[derive(Clone, Debug)]
pub struct Boss {
    pub config: CommanderConfig,
}
impl Boss {
    /// Define the configuration to the server
    pub fn new(config: CommanderConfig) -> Self {
        Boss { config }
    }
    /// Listen for a tcp connection
    pub fn listen(&self) -> Result<TcpListener, Box<dyn Error>> {
        match TcpListener::bind(self.config.addr.clone()) {
            Ok(listener) => Ok(listener),
            Err(error) => Err(Box::new(error)),
        }
    }
    pub fn channel() -> (Sender<Vec<Report>>, Receiver<Vec<Report>>) {
        channel()
    }
    /// Send commands to the client execute
    pub fn send_orders(tcp_stream: &mut TcpStream, orders: Vec<Command>) {
        let buf_order = if let Ok(buf) = Command::from_vec_to_bytes(orders) {
            buf
        } else {
            error!("Could not convert from Commands to bytes");
            Boss::disconnect(&tcp_stream);
            process::exit(1);
        };

        tcp_stream
            .write(&buf_order)
            .expect("Could not write on the stream");
    }

    /// Receive responses from executed commands in the client side
    pub fn receive_reports(mut tcp_stream: &TcpStream) -> Vec<Report> {
        let mut buf_reports = [0 as u8; 1024];
        tcp_stream
            .read(&mut buf_reports)
            .expect("Cound not read the orders from stream");

        let reports = if let Ok(reports) = Report::from_bytes_to_vec(buf_reports.to_vec()) {
            reports
        } else {
            error!("Could not convert from bytes to Commands");
            Boss::disconnect(tcp_stream);
            process::exit(1);
        };

        reports
    }
    /// Disconnect a tcp connection
    pub fn disconnect(tcp_stream: &TcpStream) {
        tcp_stream.shutdown(Shutdown::Both).unwrap();
    }
}
