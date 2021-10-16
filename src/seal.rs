use serde::{Deserialize, Serialize};
use std::error::Error;
use std::net::TcpStream;

use crate::radio::Radio;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Seal {
    pub username: String,
    pub password: String,
}

impl Seal {
    pub fn try_auth(&self, tcp_connection: &TcpStream) -> Result<bool, Box<dyn Error>> {
        Self::send_bytes(&bincode::serialize(&self)?, &tcp_connection)?;
        if !Self::is_okay(&tcp_connection)? {
            return Ok(false);
        }

        Ok(true)
    }
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
