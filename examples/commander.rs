use log::{error, info};
use simple_logger::SimpleLogger;
use std::fs::create_dir;
use std::path::Path;
use walkietalkie::commander::Commander;
use walkietalkie::config::Config;

fn main() {
    let path = Path::new("save");
    if !path.exists() {
        create_dir(path);
    }
    SimpleLogger::new().init().unwrap();
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
            info!("Report from: {:?} at {:?}", report.soldier, device.address);
            info!("status: {:#?}", report.status);
            info!("stdout: {:#?}", String::from_utf8_lossy(&report.stdout));
            info!("stderr: {:#?}", String::from_utf8_lossy(&report.stderr));
        }
        info!("Disconnecting from soldier");
        Commander::disconnect(&mut connection)
    }
}
