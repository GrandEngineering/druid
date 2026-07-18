# 🧙 Druid

[![CI](https://github.com/GrandEngineering/druid/actions/workflows/ci.yml/badge.svg)](https://github.com/GrandEngineering/druid/actions/workflows/ci.yml)
[![Rust 1.85+](https://img.shields.io/badge/rust-1.85%2B-orange.svg)](https://www.rust-lang.org)
[![License: GPL-3.0](https://img.shields.io/badge/license-GPL--3.0-blue.svg)](LICENSE.md)

Fast, time-ordered identifier generation in Rust. Druid provides both a high-entropy custom format and RFC 9562-compatible UUIDv7 bytes.

## Quick start

Add the Git dependency:

```toml
[dependencies]
druid = { git = "https://github.com/GrandEngineering/druid" }
```

Generate, format, and parse an ID:

```rust
use druid::{Druid, DruidV7};

let id = Druid::new();
println!("{id}"); // 80 lowercase hexadecimal characters
assert_eq!(id, id.to_string().parse()?);

let uuid_v7 = DruidV7::new();
assert_eq!(uuid_v7.as_bytes().len(), 16);
# Ok::<(), druid::ParseDruidError>(())
```

`Default::default()` remains available for both types.

## Formats

### `Druid` — 40 bytes

| Bytes | Contents |
|---|---|
| `0..16` | Big-endian Unix timestamp in nanoseconds |
| `16..39` | 184 random bits from the operating system RNG |
| `39` | Format version (`0`) |

### `DruidV7` — 16 bytes

`DruidV7` follows the UUIDv7 byte layout from [RFC 9562](https://www.rfc-editor.org/rfc/rfc9562.html#name-uuid-version-7), including its 48-bit millisecond timestamp, version, and variant bits.

Both types implement `Copy`, `Display`, `FromStr`, `Ord`, `Hash`, and `AsRef<[u8]>`. Raw bytes are available through `as_bytes()` and validated by `from_bytes()`.

## Ordering and uniqueness

The timestamp prefix makes IDs naturally sortable across **different clock ticks**. IDs generated in the same nanosecond (`Druid`) or millisecond (`DruidV7`) use random data for the remaining order and therefore do not necessarily preserve generation order. System clock rollback can also affect ordering.

A `Druid` has 184 random bits per timestamp value. Using the birthday bound, approximately `7.0 × 10²⁶` IDs generated in one timestamp bucket produce a 1% collision probability. This is a probabilistic guarantee, not a substitute for a database uniqueness constraint when duplicates must be impossible.

These identifiers expose creation time and are not intended to be secrets.

## Development

```sh
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-features
cargo doc --no-deps
cargo bench
```

The benchmark suite compares individual and parallel batches of Druid, UUIDv7, CUID2, and UUIDv4 generation. Results depend on hardware, Rust version, and dependency versions; run it locally rather than relying on stale published numbers.

## License

Licensed under [GPL-3.0-only](LICENSE.md).
