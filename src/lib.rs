use std::time::{Instant, SystemTime, UNIX_EPOCH};

use chrono::{DateTime, Utc};
use rand::prelude::*;
const VERSION: u8 = 0;
pub struct Druid {
    pub id: [u8; 32],
}
impl Default for Druid {
    fn default() -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let mut bytes: [u8; 23] = [0u8; 23];

        rand::rng().fill_bytes(&mut bytes);

        let mut id = [0u8; 32];
        id[0..8].clone_from_slice(&timestamp.to_be_bytes());
        id[8..31].clone_from_slice(&bytes);
        id[31] = VERSION; //version byte
        Self { id }
    }
}
impl Druid {
    pub fn to_hex(&self) -> String {
        let bytes = self.id;
        let hex_string: String = bytes
            .iter()
            .map(|byte| format!("{:02x}", byte)) // Format each byte as a 2-digit hex
            .collect();
        hex_string
    }
}
