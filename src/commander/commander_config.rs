use super::command::Command;
use serde::Deserialize;

#[derive(Deserialize, Clone, Debug)]
pub struct CommanderConfig {
    pub name: String,
    pub addr: String,
    pub commands: Vec<Command>,
}
