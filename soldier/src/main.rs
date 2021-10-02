
use daemonize::Daemonize;
use std::fs::{File, read_to_string};
use std::io::{Stderr, Write};

use log::{error, info};
use simple_logger::SimpleLogger;

use walkietalkie::report::Report;
use walkietalkie::soldier::Soldier;
use std::path::Path;

fn main() {
    SimpleLogger::new().init().unwrap();
    info!("Init Soldier daemon");
    let config = Soldier::config();
    if config.user.is_empty() || config.group.is_empty() {
        error!("Incomplete config file!");
        return;
    }
    let stdout = File::create("soldier.out").unwrap();
    let stderr = File::create("soldier.err").unwrap();

    let daemonize = Daemonize::new()
        .pid_file("soldier.pid")
        .working_directory("./")
        .user(config.user.as_str())
        .group(config.group.as_str())
        .umask(0o777)
        .stdout(stdout)
        .stderr(stderr)
        .privileged_action(|| "Executed before drop privileges");

    match daemonize.start() {
        Ok(_) => loop {
            let soldier = Soldier::new(config.clone());
            let connections = soldier.listen();
            for connection in connections.incoming() {
                match connection {
                    Ok(mut tcp_connection) => {
                        info!("Connected with commander!");
                        info!("Receiving commands");
                        let commands_received = match Soldier::receive_commands(&mut tcp_connection)
                        {
                            Ok(commands_received) => commands_received,
                            Err(error) => {
                                error!("Could not receive command from commander: {:?}", error);
                                break;
                            }
                        };

                        // .expect("Could not receive command from commander");
                        info!("Executing commands");
                        // If cannot run a command, an empty structure is returned
                        let commands_output: Vec<Report> = soldier.run_commands(commands_received);
                        info!("Sending reports to commander");
                        let _bytes_sent =
                            match Soldier::send_reports(&mut tcp_connection, commands_output) {
                                Ok(bytes_sent) => bytes_sent,
                                Err(error) => {
                                    error!("Could not send the reports to commander: {:?}", error);
                                    break;
                                }
                            };
                        info!("Disconnecting soldier from commander");
                        Soldier::disconnect(&tcp_connection);
                    }
                    Err(_) => break,
                }
            }
        },
        Err(error) => panic!("{}", error),
    }
}
