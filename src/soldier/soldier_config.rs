use serde::{Serialize, Deserialize};
/// Represents the configuration archive structure for the server
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SoldierConfig {
    pub name: String,
    pub addr: String,
    pub interval: u64,
}
