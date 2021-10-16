use daemonize::Daemonize;
use std::fs::File;

use log::{error, info};
use simple_logger::SimpleLogger;

use walkietalkie::report::Report;

use walkietalkie::soldier::Soldier;

use walkietalkie::config::Config;

fn main() {
    SimpleLogger::new().init().unwrap();
    info!("Init Soldier daemon");
    let config = Soldier::config("soldier.ron".to_string());
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
                    Ok(mut conn) => {
                        info!("Connected with commander!");
                        info!("Checking authentication information...");
                        // TODO Fix it
                        if soldier.config.seal.check_auth(&conn).unwrap() == false {
                            error!("Could not authenticate");
                            Soldier::disconnect(&conn);
                            continue;
                        }
                        info!("Authenticated!");
                        info!("Receiving commands");
                        let commands_received = match Soldier::receive_commands(&mut conn) {
                            Ok(commands_received) => commands_received,
                            Err(error) => {
                                error!("Could not receive command from commander: {:?}", error);
                                continue;
                            }
                        };

                        info!("Executing commands...");
                        // If cannot run a command, an empty structure is returned
                        let commands_output: Vec<Report> = soldier.run_commands(commands_received);
                        info!("Sending reports to commander...");
                        let _bytes_sent = match Soldier::send_reports(&mut conn, commands_output) {
                            Ok(bytes_sent) => bytes_sent,
                            Err(error) => {
                                error!("Could not send the reports to commander: {:?}", error);
                                continue;
                            }
                        };
                        info!("Disconnecting soldier from commander");
                        Soldier::disconnect(&conn);
                    }
                    Err(_) => continue,
                }
            }
        },
        Err(error) => panic!("{}", error),
    }
}
