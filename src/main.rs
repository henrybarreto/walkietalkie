//! # Walkietalkie
//!
//! Walkietalkie is an application to help system admins to execute simple payloads in many remote
//! devices at once.
#![feature(slice_as_chunks)]
pub mod commander;
pub mod communication;
pub mod config;
pub mod devices;
pub mod radio;
pub mod report;
pub mod seal;
pub mod soldier;

use crate::commander::Commander;
use crate::config::Config;
use crate::report::Report;
use crate::soldier::Soldier;

use clap::Command;
use log::{error, info};
use simple_logger::SimpleLogger;
use std::fs::create_dir;
use std::path::Path;

fn main() {
    SimpleLogger::new().init().unwrap();

    let matches = Command::new("walkietalkie")
        .version("1.0")
        .author("Henry Barreto <me@henrybarreto.dev>")
        .about("Walkietalkie is an application to help system admins to execute simple payloads in many remote devices at once.")
        .subcommand(
            Command::new("soldier").about("Start a soldier with the right configuration file"),
        )
        .subcommand(
            Command::new("commander")
                .about("Send a command to all soldiers configured in the commander.ron file"),
        )
        .get_matches();

    if let Some(_matches) = matches.subcommand_matches("soldier") {
        start_soldier();
    } else if matches.subcommand_matches("commander").is_some() {
        start_commander();
    }
}

fn start_soldier() {
    let path = Path::new("save");
    if !path.exists() {
        create_dir(path);
    }
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

fn start_commander() {
    let path = Path::new("save");
    if !path.exists() {
        create_dir(path);
    }
    info!("Init commander");
    let config = Commander::config("commander.ron".to_string());
    for device in config.devices {
        info!("Trying to connect with a soldier...");
        let mut connection = match Commander::connect(device.address.clone()) {
            Ok(connection) => connection,
            Err(_error) => {
                error!("Could not connect to {}", device.address);
                continue;
            }
        };
        info!("Connected to {}!", device.address);
        info!("Trying to authenticate with the soldier...");
        if device.seal.try_auth(&connection).unwrap() == false {
            error!("Could not authenticate");
            Commander::disconnect(&connection);
            continue;
        }
        info!("Authenticated to {}!", device.address);
        info!("Trying to send commands...");
        if let Err(_) = Commander::send_commands(&mut connection, config.commands.clone()) {
            error!("Could not send the commands to soldier");
            Commander::disconnect(&connection);
            continue;
        }
        info!("Trying receive reports...");
        let reports = if let Ok(reports) = Commander::recv_reports(&mut connection) {
            reports
        } else {
            error!("Could not receive reports from soldier");
            Commander::disconnect(&connection);
            continue;
        };

        info!("Showing reports...");
        for report in reports {
            // FIXME: Show the report in a better way.
            info!("Report from: {:?} at {:?}", report.soldier, device.address);
            info!("status: {:#?}", report.status);
            info!("stdout: {:#?}", String::from_utf8_lossy(&report.stdout));
            info!("stderr: {:#?}", String::from_utf8_lossy(&report.stderr));
        }
        info!("Disconnecting from soldier");
        Commander::disconnect(&mut connection)
    }
}
