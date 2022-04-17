use serde::{Deserialize, Serialize};

/// Report contains attributes to represent data from an executed Command.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Report {
    pub soldier: String,
    pub status: u8,
    pub stdout: Vec<u8>,
    pub stderr: Vec<u8>,
}
