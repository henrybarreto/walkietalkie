use crate::seal::Seal;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Device {
    pub address: String,
    pub seal: Seal,
}
