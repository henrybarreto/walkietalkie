use serde::{Deserialize, Serialize};
/// Represents the configuration file for soldier
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SoldierConfig {
    pub name: String,
    pub addr: String,
    pub group: String,
    pub user: String,
}
