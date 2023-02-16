use crate::commander::command::Command;
use crate::seal::Seal;
use serde::{Deserialize, Serialize};

/// SoldierConfig contains attributes for the Soldier configuration file.
///
/// # Examples
/// ```
/// use walkietalkie::soldier::soldier_config::SoldierConfig;
/// use walkietalkie::seal::Seal;
///
/// let config = SoldierConfig {
///     name: "S. Buck".to_string(),
///     addr: "127.0.0.1:14114".to_string(),
///     group: "".to_string(),
///     user: "".to_string(),
///     seal: Seal {
///     username: "".to_string(),
///     password: "".to_string(),
///    },
///     commands: vec![Command {
///         id: "".to_string(),
///         name: "".to_string(),
///         args: vec![],
///     }],
/// };
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SoldierConfig {
    pub name: String,
    pub addr: String,
    pub group: String,
    pub user: String,
    pub seal: Seal,
    pub commands: Vec<Command>,
}
