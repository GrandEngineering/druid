use std::time::{Instant, SystemTime, UNIX_EPOCH};

use chrono::{DateTime, Utc};
use rand::prelude::*;
const VERSION: u8 = 0;
pub struct Druid {
    pub id: [u8; 40],
}
impl Default for Druid {
    fn default() -> Self {
        let timestamp: u128 = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let mut bytes: [u8; 23] = [0u8; 23];

        rand::rng().fill_bytes(&mut bytes);

        let mut id = [0u8; 40];
        id[0..16].copy_from_slice(&timestamp.to_be_bytes());
        id[16..31].copy_from_slice(&bytes);
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

pub struct DruidV7 {
    pub id: [u8; 16],
}
impl Default for DruidV7 {
    fn default() -> Self {
        let mut bytes: [u8; 16] = [0u8; 16];
        let timestamp: [u8; 16] = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis()
            .to_be_bytes();
        rand::rng().fill_bytes(&mut bytes);
        bytes[0] = timestamp[10];
        bytes[1] = timestamp[11];
        bytes[2] = timestamp[12];
        bytes[3] = timestamp[13];
        bytes[4] = timestamp[14];
        bytes[5] = timestamp[15];
        bytes[6] = (bytes[6] & 0b00001111) | 0b01110000;
        bytes[8] = (bytes[8] & 0b00111111) | 0b10000000;
        Self { id: bytes }
    }
}
impl DruidV7 {
    pub fn to_hex(&self) -> String {
        let bytes = self.id;
        let hex_string: String = bytes
            .iter()
            .map(|byte| format!("{:02x}", byte)) // Format each byte as a 2-digit hex
            .collect();
        hex_string
    }
}
