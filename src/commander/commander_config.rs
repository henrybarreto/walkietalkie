use crate::commander::command::Command;
use crate::devices::Device;
use serde::{Deserialize, Serialize};

/// CommanderConfig contains attributes for the Commander configuration file.
///
/// # Examples
/// ```
///  use walkietalkie::commander::command::Command;
///  use walkietalkie::commander::commander_config::CommanderConfig;
///  use walkietalkie::devices::Device;
///  use walkietalkie::seal::Seal;
///
///  let config = CommanderConfig {
///     name: "Cpt. Steven Rogers".to_string(),
///     devices: vec![Device {
///         address: "127.0.0.1:14114".to_string(),
///         seal: Seal {
///             username: "".to_string(),
///             password: "".to_string(),
///         },
///     }],
///     commands: vec![Command {
///         name: "".to_string(),
///         args: vec![],
///     }],
///  };
///
/// ```
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CommanderConfig {
    pub name: String,
    pub devices: Vec<Device>,
    pub commands: Vec<Command>,
}
