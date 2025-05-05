# 🧙‍♂️ Druid

A **blazingly fast**, **collision-resistant**, **sortable**, unique identifier generator written in Rust.

## 🔧 How It Works

Druid IDs are 40 bytes in total, composed of four parts:

1. **Timestamp** (16 bytes): Stored in big-endian format for natural sortability.
2. **Randomness** (23 bytes): Generated using a cryptographically secure pseudo-random number generator (CSPRNG).
3. **Version** (1 byte): Reserved for versioning and future compatibility.

## 🎯 Collision Probability

While a theoretical chance of collision exists, it's *astronomically* low. Druid has around **2¹⁸⁴ unique IDs** for each nano-second.

To put that into perspective:

> You would need to generate approximately **5.55 × 10²⁶ IDs per nanosecond** to reach a **1% chance of collision**.

That’s more IDs than grains of sand on Earth.
## ⚡ Performance

| Metric             | **[DRUID]()**|  **[DRUID v7](https://www.rfc-editor.org/rfc/rfc9562.html#name-uuid-version-7)**       | **[CUID](https://github.com/mplanchard/cuid-rust)**        | **[UUIDv4](https://github.com/uuid-rs/uuid)**          |
|--------------------|-----------|-----------|-----------------|-------------------|
| **Mean**           | 45.789 ns | 42.366 ns | 3.4375 µs       | 46.160 ns         |
| **Median**         | 45.684 ns | 42.239 ns | 3.4310 µs       | 46.046 ns         |
| **Std Dev**        | 784.44 ps | 691.97 ps | 52.284 ns       | 771.28 ps         |


**Benchmarked using:**

- UUIDv4
- CUIDv2
- Druid's UUIDv7
- Intel Core i5-10400F

---

Feel free to contribute, raise issues, or star the repo if you find it useful!
