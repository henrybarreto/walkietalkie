use std::{
    error::Error,
    io::{Read, Write},
    net::{Shutdown, TcpStream},
    rc::Rc,
};

use crate::communication::Communication;
use serde::{de::DeserializeOwned, Serialize};
/// Represents methods what works for the network communication between the commander and soldier
pub trait Radio<'a, R, S>
where
    R: Communication + DeserializeOwned + 'a,
    S: Communication + Serialize,
{
    fn send_data(buffer: &[u8], mut tcp_stream: TcpStream) -> Result<usize, std::io::Error> {
        tcp_stream.write(&buffer)
    }
    fn recv_data(size: usize, mut tcp_stream: TcpStream) -> Result<Vec<u8>, std::io::Error> {
        let mut buffer = vec![0; size];
        match tcp_stream.read(&mut buffer) {
            Ok(_) => Ok(buffer),
            Err(e) => Err(e),
        }
    }

    /// Send information from a tcp connection
    fn send_information(
        tcp_stream: &TcpStream,
        informations: Vec<S>,
    ) -> Result<(), Box<dyn Error>> {
        let tcp_stream_ref: Rc<TcpStream> = Rc::new(tcp_stream.try_clone()?);

        let informations_bytes = S::from_vec_to_bytes(informations)?;

        Self::send_data(
            &bincode::serialize(&informations_bytes.len())?,
            tcp_stream_ref.try_clone()?,
        )?;

        let _is_okay = Self::recv_data(1, tcp_stream_ref.try_clone()?)?; //[0] [1]

        Self::send_data(&informations_bytes, tcp_stream_ref.try_clone()?)?;

        Ok(())
    }

    /// Receive information from a tcp connection
    fn receive_information(tcp_stream: &TcpStream) -> Result<Vec<R>, Box<dyn Error>> {
        let tcp_stream_ref: Rc<TcpStream> = Rc::new(tcp_stream.try_clone()?);

        let information_size: usize =
            bincode::deserialize(&Self::recv_data(512, tcp_stream_ref.try_clone()?)?)?;

        Self::send_data(&[1], tcp_stream_ref.try_clone()?)?;

        let commands = bincode::deserialize::<Vec<R>>(&Self::recv_data(
            information_size,
            tcp_stream_ref.try_clone()?,
        )?)?;

        Ok(commands)
    }

    /// Disconnect a tcp connection
    fn disconnect(tcp_stream: &TcpStream) {
        tcp_stream.shutdown(Shutdown::Both).unwrap()
    }
}
