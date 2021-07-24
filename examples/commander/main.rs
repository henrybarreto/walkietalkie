mod thread_pool;

use log::info;
use simple_logger::SimpleLogger;
use std::{
    error::Error,
    net::Shutdown,
    sync::{Arc, Mutex},
};

use walkietalkie::commander::Commander;
use walkietalkie::commander::{command::Command, commander_config::CommanderConfig};
use walkietalkie::radio::Radio;

fn main() -> Result<(), Box<dyn Error>> {
    SimpleLogger::new().init().unwrap();

    let thread_pool = thread_pool::ThreadPool::new(4);

    let config: CommanderConfig = Commander::config();

    let commands: Vec<Command> = config.commands.clone(); //commands_defined();
    let boss = Commander::new(config.clone());

    let connections = boss.listen().expect("Could not listen on this addr");
    info!("Listening!");

    for connection in connections.incoming() {
        match connection {
            Ok(mut tcp_stream) => {
                info!("Connected with a client");
                info!("Client IP: {}", tcp_stream.peer_addr().unwrap().to_string());
                let (commander_channel_send, commander_channel_recv) = Commander::channel();
                let commands_clone = Arc::new(Mutex::new(commands.clone()));
                let commander_clone = Arc::new(Mutex::new(boss.clone()));
                info!("Opening a thread...");
                thread_pool.execute(move || {
                    let _commander_from_thread =
                        commander_clone.lock().expect("Could not lock commander");
                    let commands_from_thread =
                        commands_clone.lock().expect("Could not lock commands");
                    info!("Sending orders..");
                    Commander::send_information(&mut tcp_stream, commands_from_thread.clone())
                        .unwrap();
                    info!("Recieving reports...");
                    let reports = Commander::receive_information(&mut tcp_stream).unwrap();
                    info!("Sending reports through the channel...");
                    commander_channel_send.send(reports).unwrap();
                    info!("Desconnecting from a client..");
                    tcp_stream.shutdown(Shutdown::Both).unwrap();
                });
                let reports = commander_channel_recv.recv().unwrap();
                info!("Showing reports from the client...");
                for report in reports.iter() {
                    info!("----------");
                    info!("Status: {:#?}", report.status);
                    info!("Stdout: {:#?}", String::from_utf8_lossy(&report.stdout));
                    info!("Stderr: {:#?}", String::from_utf8_lossy(&report.stderr));
                }
            }
            Err(error) => {
                println!("{}", error);
            }
        }
    }
    Ok(())
}
