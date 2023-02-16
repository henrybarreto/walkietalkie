use crate::devices::Soldier;
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
///     soldiers: vec![Soldier {
///         address: "127.0.0.1:14114".to_string(),
///         seal: Seal {
///             username: "".to_string(),
///             password: "".to_string(),
///         },
///     }],
///    commands: vec![],
///  };
///
/// ```
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CommanderConfig {
    pub name: String,
    pub soldiers: Vec<Soldier>,
    pub commands: Vec<String>,
}
