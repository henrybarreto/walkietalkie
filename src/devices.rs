use crate::seal::Seal;
use serde::{Deserialize, Serialize};

/// Device contains attributes about a device.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Device {
    /// The device's address.
    pub address: String,
    /// The device's seal.
    /// Seal is the credentials to access a device.
    pub seal: Seal,
}
