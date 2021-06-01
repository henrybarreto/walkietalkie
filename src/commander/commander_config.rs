use super::command::Command;
use serde::Deserialize;
/// Represents the configuration archive structure for the server
#[derive(Deserialize, Clone, Debug)]
pub struct CommanderConfig {
    pub name: String,
    pub addr: String,
    pub commands: Vec<Command>,
}
