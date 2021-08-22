use serde::{Deserialize, Serialize};

/// Represent a command to be sent through socket and executed in client side
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Command {
    pub name: String,
    pub args: Vec<String>,
}
