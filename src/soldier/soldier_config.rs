use serde::Deserialize;
#[derive(Deserialize, Clone, Debug)]
pub struct SoldierConfig {
    pub name: String,
    pub addr: String,
    pub interval: u64,
}
