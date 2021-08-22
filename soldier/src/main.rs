use daemonize::Daemonize;
use std::fs::File;

use log::info;
use simple_logger::SimpleLogger;

use walkietalkie::report::Report;
use walkietalkie::soldier::Soldier;
use walkietalkie::commander::command::Command;

fn main() {
    SimpleLogger::new().init().unwrap();
    info!("Init Soldier daemon");
    let stdout = File::create("soldier.out").unwrap();
    let stderr = File::create("soldier.err").unwrap();

    let daemonize = Daemonize::new()
        .pid_file("soldier.pid") 
        .working_directory("./") 
        .user("henry")
        .group("henry") 
        .umask(0o777) 
        .stdout(stdout) 
        .stderr(stderr) 
        .privileged_action(|| "Executed before drop privileges");

    match daemonize.start() {
        Ok(_) => loop {
            let config = Soldier::config();
            let soldier = Soldier::new(config.clone());
            let connections = soldier.listen().expect("Could not connect to the addr");
            for connection in connections.incoming() {
                match connection {
                    Ok(mut tcp_connection) => {
                        info!("Connected with commander!");
                        info!("Receiving commands");
                        let commands_received: Vec<Command> = Soldier::recv_commands(&mut tcp_connection)
                            .expect("Could not receive command from commander");
                        info!("Executing commands");
                        let commands_output: Vec<Report> = soldier.run_commands(commands_received);
                        info!("Sending reports to commander");
                        let _bytes_sent = Soldier::send_reports(&mut tcp_connection, commands_output)
                            .expect("Could not send the reports to commander");
                        info!("Disconnecting soldier from commander");
                        Soldier::disconnect(&tcp_connection);
                    }
                    Err(error) => panic!("{}", error),
                }
            }
        },
        Err(error) => panic!("{}", error),
    }
}
