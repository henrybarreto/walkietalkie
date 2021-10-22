use std::fmt::{Display, Formatter};
use serde::{Deserialize, Serialize};

/// Represent a command that will be sent through socket and executed in client side
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Command {
    pub name: String,
    pub args: Vec<String>,
}
impl Display for Command{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}",ron::to_string(self).unwrap())
    }
}