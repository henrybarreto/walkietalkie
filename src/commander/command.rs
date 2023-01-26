use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

/// A Command contains attributes to represent a command from Commander to be executed on Soldier.
///
/// # Examples
/// ```
/// use walkietalkie::commander::command::Command;
///
/// let command = Command {
///     name: "echo".to_string(),
///     args: vec!["Hello, world!".to_string()],
/// };
/// ```
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Command {
    /// The command's name.
    pub name: String,
    /// The command's arguments.
    pub args: Vec<String>,
}
impl Display for Command {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", ron::to_string(self).unwrap())
    }
}
