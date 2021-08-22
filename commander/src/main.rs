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
        
        info!("{:#?}", reports);
        info!("Disconnecting from soldier");
        Commander::disconnect(&mut connection)
    };

}
