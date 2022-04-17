use serde::{Deserialize, Serialize};
use std::error::Error;
use std::net::TcpStream;

use crate::radio::Radio;

/// Seal contains attributes and methods to authenticate a Commander to a Device.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Seal {
    /// The seal's username.
    pub username: String,
    /// The seal's password.
    pub password: String,
}

impl Seal {
    /// Try to authenticate a Commander to a Soldier. It sends the Seal to the Device and waits for
    /// a response.
    pub fn try_auth(&self, tcp_connection: &TcpStream) -> Result<bool, Box<dyn Error>> {
        Self::send_bytes(&bincode::serialize(&self)?, &tcp_connection)?;
        if !Self::is_okay(&tcp_connection)? {
            return Ok(false);
        }

        Ok(true)
    }
    /// Check if Commander authentication information is valid for the Soldier. It receives Seal and
    /// check if it is equal to the Seal specified.
    pub fn check_auth(&self, tcp_connection: &TcpStream) -> Result<bool, Box<dyn Error>> {
        let seal = bincode::deserialize::<Seal>(&Self::receive_bytes(128, tcp_connection)?)?;
        if seal.username != self.username || seal.password != self.password {
            Self::send_bytes(&bincode::serialize(&false)?, tcp_connection)?;
            return Ok(false);
        }
        Self::send_bytes(&bincode::serialize(&true)?, tcp_connection)?;

        Ok(true)
    }
}
impl Radio for Seal {}
