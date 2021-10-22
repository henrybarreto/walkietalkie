use daemonize::Daemonize;
use std::fs::{create_dir, File};
use std::path::Path;

use log::{error, info};
use simple_logger::SimpleLogger;

use walkietalkie::report::Report;

use walkietalkie::soldier::Soldier;

use walkietalkie::config::Config;

fn main() {
    let path = Path::new("save");
    if !path.exists(){
        create_dir(path);
    }
    SimpleLogger::new().init().unwrap();
    info!("Init Soldier daemon");
    let config = Soldier::config("soldier.ron".to_string());
    if config.user.is_empty() || config.group.is_empty() {
        error!("Incomplete config file!");
        return;
    }

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
}
