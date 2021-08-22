use daemonize::Daemonize;
use std::fs::File;

use log::info;

use walkietalkie::commander::Commander;
use walkietalkie::radio::Radio;
use walkietalkie::report::Report;
use walkietalkie::soldier::Soldier;
use walkietalkie::commander::command::Command;

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
        .umask(0o777) // Set umask, `0o027` by default.
        .stdout(stdout) // Redirect stdout to `/tmp/daemon.out`.
        .stderr(stderr) // Redirect stderr to `/tmp/daemon.err`.
        .privileged_action(|| "Executed before drop privileges");

    match daemonize.start() {
        Ok(_) => loop {
            let config = Soldier::config();
            let soldier = Soldier::new(config.clone());
            let connections = soldier.listen().expect("Could not connect to the addr");
            for connection in connections.incoming() {
                match connection {
                    Ok(mut tcp_connection) => {
                        let commands_received: Vec<Command> = Soldier::recv_commands(&mut tcp_connection).unwrap();
                        let commands_output: Vec<Report> = soldier.run_commands(commands_received).unwrap();
                        let _bytes_sent = Soldier::send_reports(&mut tcp_connection, commands_output);
                        Soldier::disconnect(&tcp_connection);
                    }
                    Err(error) => panic!(error),
                }
            }
        },
        Err(error) => panic!(error),
    }
}
