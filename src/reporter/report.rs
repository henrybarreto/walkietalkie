use serde::{Deserialize, Serialize};
/// Represent a response from a executed command
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Report {
    pub status: i32,
    pub stdout: Vec<u8>,
    pub stderr: Vec<u8>,
}
