use crate::commander::command::Command;
use serde::Deserialize;

/// Represents the configuration archive structure for the client
#[derive(Deserialize, Clone, Debug)]
pub struct CommanderConfig {
    pub name: String,
    pub addrs: Vec<String>,
    pub commands: Vec<Command>,
}
