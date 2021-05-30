use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Command {
    pub name: String,
    pub args: Vec<String>,
}
