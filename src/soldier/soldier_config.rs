use serde::Deserialize;
/// Represents the configuration archive structure for the server
#[derive(Deserialize, Clone, Debug)]
pub struct SoldierConfig {
    pub name: String,
    pub addr: String,
    pub interval: u64,
}
