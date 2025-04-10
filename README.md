# Druid
A blazingly fast, colision resistant, sortable, Unique Identifier made in Rust.
## How it works?
Druid IDs are composed of 4 components.
First 8 bytes are the timestamp in big endian for sorting.
Then the next 23 bytes are randomly generated using a CSPRNG
And the last byte is for the version
## Odds of collision
While there is a theoretical chance of a collision, the total number of possible IDs in a second is on the order of 2^184. Taking this into account, you would need to generate approximately 5.55×10^26 IDs in a single second to have a 1% chance of a collision.
## Performance

| Metric             | **[DRUID]()**       | **[CUID](https://github.com/mplanchard/cuid-rust)**        | **[UUID](https://github.com/uuid-rs/uuid)**          |
|--------------------|-----------------|-----------------|-------------------|
| **Mean**           | 45.789 ns       |  	3.4375 µs    | 46.160 ns         |
| **Median**         | 45.684 ns       | 3.4310 µs       | 46.046 ns         |
| **Std Dev**        | 784.44 ps       | 52.284 ns       | 771.28 ps         |


**Notes:**
UUIDv4
CUIDv2
Intel Core i5-10400F
To get a 1% chance of colision you would need to speed up Druid 6.09x10^31 times.
