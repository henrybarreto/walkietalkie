
use std::{
    fs::File,
};
use daemonize::Daemonize;

use log::info;

use walkietalkie::radio::Radio;
use walkietalkie::reporter::Reporter;
use walkietalkie::soldier::Soldier;

fn main() {
    info!("Init Soldier daemon...");
    let stdout = File::create("soldier.out").unwrap();
    let stderr = File::create("soldier.err").unwrap();

    let daemonize = Daemonize::new()
        .pid_file("soldier.pid") // Every method except `new` and `start`
        //.chown_pid_file(true)      // is optional, see `Daemonize` documentation
        .working_directory("./") // for default behaviour.
        .user("root")
        .group("root") // Group name
        .group(2) // or group id.
        .umask(0o777) // Set umask, `0o027` by default.
        .stdout(stdout) // Redirect stdout to `/tmp/daemon.out`.
        .stderr(stderr) // Redirect stderr to `/tmp/daemon.err`.
        .privileged_action(|| "Executed before drop privileges");

    match daemonize.start() {
        Ok(_) => loop {
            let config = Soldier::config();
            let mut tcp_stream = Reporter::connect(config.addr.clone());
            let commands_received = Reporter::receive_information(&mut tcp_stream).unwrap();
            let commands_output = Soldier::run_commands(commands_received);
            let _bytes_sent = Reporter::send_information(&mut tcp_stream, commands_output).unwrap();
            Reporter::disconnect(&tcp_stream);
        },
        Err(e) => eprintln!("Error, {}", e),
    }
}
