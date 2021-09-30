use std::process::Output;

use simple_logger::SimpleLogger;
use walkietalkie::commander::Commander;

use log::info;

fn main() {
    SimpleLogger::new().init().unwrap();
    info!("Init commander");
    let config = Commander::config();
    for addr in config.addrs {
        info!("Trying to connect with a soldier");
        let mut connection = Commander::connect(addr);
        info!("Connected!");
        info!("Trying to send commands");
        Commander::send_commands(&mut connection, config.commands.clone())
            .expect("Could not send the commands to soldier");
        info!("Trying receive reports");
        let reports = Commander::recv_reports(&mut connection)
            .expect("Could not receive reports from soldier");

        info!("Showing reports...");
        for report in reports {
            info!(
                "Report from: {:?} at {:?}",
                report.soldier.config.name, report.soldier.config.addr
            );
            info!("status: {:#?}", report.status);
            info!("stdout: {:#?}", String::from_utf8_lossy(&report.stdout));
            info!("stderr: {:#?}", String::from_utf8_lossy(&report.stderr));
        }
        info!("Disconnecting from soldier");
        Commander::disconnect(&mut connection)
    }
}
