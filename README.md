# 🧙‍♂️ Druid

A **blazingly fast**, **collision-resistant**, **sortable**, and **unique identifier** generator written in Rust.

## 🔧 How It Works

Druid IDs are 32 bytes in total, composed of four parts:

1. **Timestamp** (8 bytes): Stored in big-endian format for natural sortability.
2. **Randomness** (23 bytes): Generated using a cryptographically secure pseudo-random number generator (CSPRNG).
3. **Version** (1 byte): Reserved for versioning and future compatibility.

## 🎯 Collision Probability

While a theoretical chance of collision exists, it's *astronomically* low. Druid can produce around **2¹⁸⁴ unique IDs per second**.

To put that into perspective:

> You would need to generate approximately **5.55 × 10²⁶ IDs per second** to reach a **1% chance of collision**.

That’s more IDs than grains of sand on Earth.
## ⚡ Performance

| Metric             | **[DRUID]()**       | **[CUID](https://github.com/mplanchard/cuid-rust)**        | **[UUID](https://github.com/uuid-rs/uuid)**          |
|--------------------|-----------------|-----------------|-------------------|
| **Mean**           | 45.789 ns       |  	3.4375 µs    | 46.160 ns         |
| **Median**         | 45.684 ns       | 3.4310 µs       | 46.046 ns         |
| **Std Dev**        | 784.44 ps       | 52.284 ns       | 771.28 ps         |


**Benchmarked using:**

- UUIDv4
- CUIDv2
- Intel Core i5-10400F

> To reach a 1% chance of collision, Druid would need to be **6.09 × 10³¹ times faster**—more than the number of grains of sand on Earth.

---

Feel free to contribute, raise issues, or star the repo if you find it useful!
