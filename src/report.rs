use serde::{Deserialize, Serialize};
use crate::{soldier::Soldier};

/// Represent a response from a executed command
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Report {
    pub soldier: Soldier,
    pub status: u8,
    pub stdout: Vec<u8>,
    pub stderr: Vec<u8>,
}
