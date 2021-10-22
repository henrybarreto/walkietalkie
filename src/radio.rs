use std::{env, error::Error, io::{Read, Write}, net::{Shutdown, TcpStream}};
use std::fs::{File, remove_file};

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

    fn receive_chucked(tcp_connection: &TcpStream) -> Result<File, Box<dyn Error>> {
        let buf = env::temp_dir().join("wt.tmp");
        if buf.exists(){
            remove_file(&buf);
        }
        let mut file = File::create(buf)?;
        loop {
            let mut data_received = Self::receive_bytes(128, &tcp_connection)?;
            if bincode::deserialize::<i32>(&data_received)? == -1 {
                Self::send_bytes(&bincode::serialize(&true)?, &tcp_connection);
                break;
            }
            file.write(&mut data_received);
            Self::send_bytes(&bincode::serialize(&true)?, &tcp_connection);
        }

        Ok(file)
    }

    /// Disconnect from a TcpStream
    fn disconnect(tcp_connection: &TcpStream) {
        tcp_connection.shutdown(Shutdown::Both).unwrap()
    }
}
