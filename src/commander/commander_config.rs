use crate::commander::command::Command;
use serde::{Serialize, Deserialize};

/// Represents the configuration file for commander
#[derive(Serialize,Deserialize, Clone, Debug)]
pub struct CommanderConfig {
    pub name: String,
    pub addrs: Vec<String>,
    pub commands: Vec<Command>,
}
