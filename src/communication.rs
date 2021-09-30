use std::error::Error;

use log::error;
use serde::{Deserialize, Serialize};

pub trait Communication {
    fn from_bytes(bytes: &'static [u8]) -> Result<Self, Box<bincode::ErrorKind>>
    where
        Self: Sized + Deserialize<'static>,
    {
        bincode::deserialize(&bytes)
    }
    fn to_bytes(&self) -> Result<Vec<u8>, Box<bincode::ErrorKind>>
    where
        Self: Sized + Serialize,
    {
        bincode::serialize(&self)
    }
    fn from_bytes_to_vec(bytes: &'static [u8]) -> Result<Vec<Self>, Box<bincode::ErrorKind>>
    where
        Self: Sized + Deserialize<'static>,
    {
        bincode::deserialize(&bytes)
    }
    fn from_vec_to_bytes(communication: Vec<Self>) -> Result<Vec<u8>, Box<bincode::ErrorKind>>
    where
        Self: Sized + Serialize,
    {
        bincode::serialize(&communication)
    }
}
