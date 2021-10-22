use crate::commander::command::Command;
use crate::devices::Device;
use serde::{Deserialize, Serialize};

/// Represents the configuration file for commander
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CommanderConfig {
    pub name: String,
    pub devices: Vec<Device>,
    pub commands: Vec<Command>,
}
