use serde::{Deserialize, Serialize};

/// Represent a response from a executed Command
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Report {
    pub soldier: String,
    pub status: u8,
    pub stdout: Vec<u8>,
    pub stderr: Vec<u8>,
}
