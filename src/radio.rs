use std::{error::Error, io::{Read, Write}, fs::File, net::{Shutdown, TcpStream}, rc::Rc, env};

use crate::communication::Communication;
use log::debug;
use serde::{de::DeserializeOwned, Serialize};
use std::fs::remove_file;

/// Represents methods to work with network communication between the Soldier and Commander
pub trait Radio {
    fn send_bytes(buffer: &[u8], mut tcp_stream: TcpStream) -> Result<usize, std::io::Error> {
        tcp_stream.write(&buffer)
    }
    fn receive_bytes(size: usize, mut tcp_stream: TcpStream) -> Result<Vec<u8>, std::io::Error> {
        let mut buffer = vec![0; size];
        tcp_stream.read(&mut buffer).map(|_| Ok(buffer))?
    }

    fn send_chucked(tcp_connection: &TcpStream, data: Vec<u8>) -> Result<bool, Box<dyn Error>> {
        let tcp_connection_ref: Rc<TcpStream> = Rc::new(tcp_connection.try_clone()?);

        let (chunks, remained) = data.as_chunks::<128>();
        for chunk in chunks {
            Self::send_bytes(chunk, tcp_connection_ref.try_clone()?)?;
            let okay = Self::receive_bytes(1, tcp_connection_ref.try_clone()?)?;
            if okay != [1] {
                panic!("A response was not okay as expected because of a chuck bytes");
            }
        }
        Self::send_bytes(remained, tcp_connection_ref.try_clone()?)?;
        let okay = Self::receive_bytes(1, tcp_connection_ref.try_clone()?)?;
        if okay != [1] {
            panic!("A response was not okay as expected because of remained bytes");
        }

        Self::send_bytes(&bincode::serialize(&-1)?, tcp_connection_ref.try_clone()?)?;
        let okay = Self::receive_bytes(1, tcp_connection_ref.try_clone()?)?;
        if okay != [1] {
            panic!("A response was not okay as expected because of end of file byte");
        }

        Ok(true)
    }

    fn receive_chucked(tcp_connection: &TcpStream) -> Result<File, Box<dyn Error>> {
        let tcp_connection_ref: Rc<TcpStream> = Rc::new(tcp_connection.try_clone()?);

        let buf = env::temp_dir().join("wt.tmp");
        if buf.exists(){
            remove_file(&buf);
        }
        let mut file = File::create(buf)?;
        loop {
            let mut data_received = Self::receive_bytes(128, tcp_connection_ref.try_clone()?)?;
            if bincode::deserialize::<i32>(&data_received)? == -1 {
                Self::send_bytes(&[1], tcp_connection_ref.try_clone()?);
                break;
            }
            file.write(&mut data_received);
            Self::send_bytes(&[1], tcp_connection_ref.try_clone()?);
        }
        Ok(file)
    }

    /// Disconnect from a TcpStream
    fn disconnect(tcp_connection: &TcpStream) {
        tcp_connection.shutdown(Shutdown::Both).unwrap()
    }
}
