use std::{env, error::Error, io::{Read, Write}, net::{Shutdown, TcpStream}};
use std::fs::{create_dir, create_dir_all, File, Permissions, remove_file};
use std::io::BufWriter;
use std::path::{Path, PathBuf};
use rand::distributions::Alphanumeric;
use rand::Rng;
use rand::rngs::OsRng;
use log::trace;

/// Radio has methods to send and receive data
pub trait Radio {
    fn send_bytes(buffer: &[u8], mut tcp_stream: &TcpStream) -> Result<usize, std::io::Error> {
        tcp_stream.write(&buffer)
    }
    fn receive_bytes(size: usize, mut tcp_stream: &TcpStream) -> Result<Vec<u8>, std::io::Error> {
        let mut buffer = vec![0; size];
        tcp_stream.read(&mut buffer).map(|_| Ok(buffer))?
    }

    fn is_okay(tcp_connection: &TcpStream) -> bincode::Result<bool> {
        bincode::deserialize::<bool>(&Self::receive_bytes(1, &tcp_connection)?)
    }

    fn send_chucked(tcp_connection: &TcpStream, data: Vec<u8>) -> Result<bool, Box<dyn Error>> {
        println!("Output {}", &data.len());
        let (chunks, remained) = data.as_chunks::<128>();
        for chunk in chunks {
            Self::send_bytes(chunk, &tcp_connection)?;
            if !Self::is_okay(&tcp_connection)? {
                // panic!("A response was not okay as expected because of a chuck bytes");
                return Ok(false);
            }
        }
        Self::send_bytes(remained, &tcp_connection)?;
        if !Self::is_okay(&tcp_connection)? {
            // panic!("A response was not okay as expected because of remained bytes");
            return Ok(false);
        }

        // Sends -1 to indicate the EOF
        Self::send_bytes(&bincode::serialize(&-1)?, &tcp_connection)?;
        if !Self::is_okay(&tcp_connection)? {
            // panic!("A response was not okay as expected because of end of file byte");
            return Ok(false);
        }

        Ok(true)
    }

    fn receive_chucked(tcp_connection: &TcpStream) -> Result<PathBuf, Box<dyn Error>> {
        let x: String = OsRng
            .sample_iter(&Alphanumeric)
            .take(5)
            .map(char::from)
            .collect();

        let buf = Path::new("save").join(format!("{}.tmp", x));
        if buf.exists() {
            remove_file(&buf);
        }
        trace!("Tmp File: {}", &buf.as_os_str().to_str().unwrap());
        let mut file = File::create(&buf)?;
        let mut data = vec![];
        loop {
            let mut data_received = Self::receive_bytes(128, &tcp_connection)?;
            if bincode::deserialize::<i32>(&data_received)? == -1 {
                Self::send_bytes(&bincode::serialize(&true)?, &tcp_connection);
                break;
            }
            data.append(&mut data_received);
            Self::send_bytes(&bincode::serialize(&true)?, &tcp_connection);
        }
        println!("Input {}", &data.len());

        file.write_all(&data);
        file.flush();
        Ok(buf)
    }

    /// Disconnect from a TcpStream
    fn disconnect(tcp_connection: &TcpStream) {
        tcp_connection.shutdown(Shutdown::Both).unwrap()
    }
}
