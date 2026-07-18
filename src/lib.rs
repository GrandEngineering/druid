//! Fast, time-ordered identifier generation.
//!
//! [`Druid`] is the crate's native 40-byte format. [`DruidV7`] generates
//! standards-compatible UUIDv7 bytes. Both types sort by timestamp because
//! their timestamps are stored first in big-endian order.
//!
//! ```
//! use druid::{Druid, DruidV7};
//!
//! let id = Druid::new();
//! assert_eq!(id.to_string().len(), 80);
//!
//! let uuid_v7 = DruidV7::new();
//! assert_eq!(uuid_v7.as_bytes().len(), 16);
//! ```

use rand::prelude::*;
use std::{fmt, str::FromStr, time::UNIX_EPOCH};

const DRUID_VERSION: u8 = 0;

/// An error returned when parsing or validating an identifier.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ParseDruidError {
    /// The hexadecimal representation has the wrong length.
    InvalidLength { expected: usize, actual: usize },
    /// A non-hexadecimal byte was found at this byte offset.
    InvalidHex { index: usize },
    /// The identifier uses an unsupported version.
    InvalidVersion,
    /// The UUID variant bits are not RFC 9562 compatible.
    InvalidVariant,
}

impl fmt::Display for ParseDruidError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidLength { expected, actual } => {
                write!(
                    f,
                    "expected {expected} hexadecimal characters, got {actual}"
                )
            }
            Self::InvalidHex { index } => {
                write!(f, "invalid hexadecimal character at byte {index}")
            }
            Self::InvalidVersion => f.write_str("unsupported identifier version"),
            Self::InvalidVariant => f.write_str("invalid UUID variant"),
        }
    }
}

impl std::error::Error for ParseDruidError {}

#[inline]
fn unix_nanos() -> u128 {
    UNIX_EPOCH.elapsed().unwrap_or_default().as_nanos()
}

#[inline]
fn fmt_hex(bytes: &[u8], f: &mut fmt::Formatter<'_>) -> fmt::Result {
    for byte in bytes {
        write!(f, "{byte:02x}")?;
    }
    Ok(())
}

#[inline]
fn bytes_to_hex(bytes: &[u8]) -> String {
    const LUT: &[u8; 16] = b"0123456789abcdef";
    let mut out = vec![0u8; bytes.len() * 2];
    for (i, &byte) in bytes.iter().enumerate() {
        out[i * 2] = LUT[(byte >> 4) as usize];
        out[i * 2 + 1] = LUT[(byte & 0x0f) as usize];
    }
    // SAFETY: every byte comes from the ASCII lookup table above.
    unsafe { String::from_utf8_unchecked(out) }
}

fn decode_hex<const N: usize>(value: &str) -> Result<[u8; N], ParseDruidError> {
    let input = value.as_bytes();
    if input.len() != N * 2 {
        return Err(ParseDruidError::InvalidLength {
            expected: N * 2,
            actual: input.len(),
        });
    }

    let mut output = [0; N];
    for (index, pair) in input.chunks_exact(2).enumerate() {
        let high = hex_nibble(pair[0]).ok_or(ParseDruidError::InvalidHex { index: index * 2 })?;
        let low = hex_nibble(pair[1]).ok_or(ParseDruidError::InvalidHex {
            index: index * 2 + 1,
        })?;
        output[index] = (high << 4) | low;
    }
    Ok(output)
}

#[inline]
const fn hex_nibble(byte: u8) -> Option<u8> {
    match byte {
        b'0'..=b'9' => Some(byte - b'0'),
        b'a'..=b'f' => Some(byte - b'a' + 10),
        b'A'..=b'F' => Some(byte - b'A' + 10),
        _ => None,
    }
}

/// A 40-byte, time-ordered Druid identifier.
///
/// The layout is a 16-byte Unix timestamp in nanoseconds, 23 random bytes,
/// and one version byte. Ordering is chronological across distinct timestamp
/// values; IDs created during the same clock tick have random relative order.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Druid {
    id: [u8; 40],
}

impl Druid {
    /// Generates a new identifier using the system clock and OS-backed RNG.
    #[must_use]
    #[inline]
    pub fn new() -> Self {
        let mut id = [0; 40];
        id[..16].copy_from_slice(&unix_nanos().to_be_bytes());
        rand::rng().fill_bytes(&mut id[16..39]);
        id[39] = DRUID_VERSION;
        Self { id }
    }

    /// Creates an identifier from its binary representation.
    pub fn from_bytes(id: [u8; 40]) -> Result<Self, ParseDruidError> {
        if id[39] != DRUID_VERSION {
            return Err(ParseDruidError::InvalidVersion);
        }
        Ok(Self { id })
    }

    /// Returns the binary representation.
    #[must_use]
    pub const fn as_bytes(&self) -> &[u8; 40] {
        &self.id
    }

    /// Consumes the identifier and returns its binary representation.
    #[must_use]
    pub const fn into_bytes(self) -> [u8; 40] {
        self.id
    }

    /// Returns the Unix timestamp in nanoseconds encoded in this identifier.
    #[must_use]
    pub fn timestamp_nanos(&self) -> u128 {
        u128::from_be_bytes(self.id[..16].try_into().expect("slice has a fixed length"))
    }

    /// Returns the compact, lowercase hexadecimal representation.
    #[must_use]
    #[inline]
    pub fn to_hex(&self) -> String {
        bytes_to_hex(&self.id)
    }
}

impl Default for Druid {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Druid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt_hex(&self.id, f)
    }
}

impl FromStr for Druid {
    type Err = ParseDruidError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::from_bytes(decode_hex(value)?)
    }
}

impl AsRef<[u8]> for Druid {
    fn as_ref(&self) -> &[u8] {
        &self.id
    }
}

impl From<Druid> for [u8; 40] {
    fn from(value: Druid) -> Self {
        value.id
    }
}

/// A 16-byte UUID version 7 identifier following RFC 9562.
///
/// The first 48 bits contain a Unix timestamp in milliseconds. IDs generated
/// within the same millisecond have random relative order.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct DruidV7 {
    id: [u8; 16],
}

impl DruidV7 {
    /// Generates a new UUIDv7 using the system clock and OS-backed RNG.
    #[must_use]
    #[inline]
    pub fn new() -> Self {
        let timestamp = u64::try_from(unix_nanos() / 1_000_000)
            .unwrap_or(u64::MAX)
            .to_be_bytes();
        let mut id = [0; 16];
        id[..6].copy_from_slice(&timestamp[2..]);
        rand::rng().fill_bytes(&mut id[6..]);
        id[6] = (id[6] & 0x0f) | 0x70;
        id[8] = (id[8] & 0x3f) | 0x80;
        Self { id }
    }

    /// Creates a UUIDv7 from its binary representation.
    pub fn from_bytes(id: [u8; 16]) -> Result<Self, ParseDruidError> {
        if id[6] >> 4 != 7 {
            return Err(ParseDruidError::InvalidVersion);
        }
        if id[8] >> 6 != 2 {
            return Err(ParseDruidError::InvalidVariant);
        }
        Ok(Self { id })
    }

    /// Returns the binary UUID representation.
    #[must_use]
    pub const fn as_bytes(&self) -> &[u8; 16] {
        &self.id
    }

    /// Consumes the identifier and returns its binary UUID representation.
    #[must_use]
    pub const fn into_bytes(self) -> [u8; 16] {
        self.id
    }

    /// Returns the Unix timestamp in milliseconds encoded in this UUID.
    #[must_use]
    pub fn timestamp_millis(&self) -> u64 {
        let mut timestamp = [0; 8];
        timestamp[2..].copy_from_slice(&self.id[..6]);
        u64::from_be_bytes(timestamp)
    }

    /// Returns the compact, lowercase hexadecimal representation.
    #[must_use]
    #[inline]
    pub fn to_hex(&self) -> String {
        bytes_to_hex(&self.id)
    }
}

impl Default for DruidV7 {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for DruidV7 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt_hex(&self.id, f)
    }
}

impl FromStr for DruidV7 {
    type Err = ParseDruidError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::from_bytes(decode_hex(value)?)
    }
}

impl AsRef<[u8]> for DruidV7 {
    fn as_ref(&self) -> &[u8] {
        &self.id
    }
}

impl From<DruidV7> for [u8; 16] {
    fn from(value: DruidV7) -> Self {
        value.id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn druid_round_trips_through_hex() {
        let id = Druid::new();
        let encoded = id.to_string();
        assert_eq!(encoded.len(), 80);
        assert_eq!(encoded.parse::<Druid>(), Ok(id));
        assert_eq!(
            id.timestamp_nanos(),
            u128::from_be_bytes(id.id[..16].try_into().unwrap())
        );
    }

    #[test]
    fn druid_validates_version() {
        let mut bytes = [0; 40];
        bytes[39] = 1;
        assert_eq!(
            Druid::from_bytes(bytes),
            Err(ParseDruidError::InvalidVersion)
        );
    }

    #[test]
    fn uuid_v7_has_expected_version_and_variant() {
        let id = DruidV7::new();
        assert_eq!(id.id[6] >> 4, 7);
        assert_eq!(id.id[8] >> 6, 2);
        assert_eq!(id.to_string().parse::<DruidV7>(), Ok(id));
    }

    #[test]
    fn parsing_rejects_bad_input() {
        assert!(matches!(
            "00".parse::<Druid>(),
            Err(ParseDruidError::InvalidLength { .. })
        ));
        let invalid = "z".repeat(80);
        assert_eq!(
            invalid.parse::<Druid>(),
            Err(ParseDruidError::InvalidHex { index: 0 })
        );
    }

    #[test]
    fn byte_order_is_timestamp_order() {
        let mut earlier = [0; 40];
        earlier[..16].copy_from_slice(&1_u128.to_be_bytes());
        let mut later = earlier;
        later[..16].copy_from_slice(&2_u128.to_be_bytes());

        assert!(Druid::from_bytes(earlier).unwrap() < Druid::from_bytes(later).unwrap());
    }
}
