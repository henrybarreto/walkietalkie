//use std::{fs::File, net::{Shutdown, TcpStream}, thread, time::Duration};
//use std::io::{Read, Write};
//use std::{fs::File, io::Read, thread, time};
//use daemonize::Daemonize;
//use soldier::soldier::Soldier;
//use walkietalkie::walkietalkie::{Command, Response, Soldier};

//use log::info;
use simple_logger::SimpleLogger;
use walkietalkie::{reporter::Reporter, soldier::Soldier};
//use walkietalkie::{Reporter, soldier::Soldier};

fn main() {
    SimpleLogger::new().init().unwrap();
    /*let stdout = File::create("soldier.out").unwrap();
    let stderr = File::create("soldier.err").unwrap();

    let daemonize = Daemonize::new()
        .pid_file("soldier.pid") // Every method except `new` and `start`
        //.chown_pid_file(true)      // is optional, see `Daemonize` documentation
        .working_directory("./") // for default behaviour.
        .user("root")
        .group("root") // Group name
        .group(2)        // or group id.
        .umask(0o777)    // Set umask, `0o027` by default.
        .stdout(stdout)  // Redirect stdout to `/tmp/daemon.out`.
        .stderr(stderr)  // Redirect stderr to `/tmp/daemon.err`.
        .privileged_action(|| "Executed before drop privileges");

    match daemonize.start() {
      Ok(_) => {
        loop {*/
    let config = Soldier::config();
    let mut tcp_stream = Reporter::connect(config.addr.clone());
    let commands_recieved = Reporter::receive_commands(&mut tcp_stream).unwrap();
    let commands_output = Soldier::run_commands(commands_recieved);
    let _bytes_sent = Reporter::send_report(&mut tcp_stream, commands_output).unwrap();
    Reporter::disconnect(&tcp_stream);

    /*
        thread::sleep(time::Duration::from_secs(config.interval));
      }
    },
    Err(e) => eprintln!("Error, {}", e),
    }*/
}
