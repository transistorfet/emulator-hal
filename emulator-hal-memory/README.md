[![crates.io](https://img.shields.io/crates/v/emulator-hal.svg)](https://crates.io/crates/emulator-hal)
[![Documentation](https://docs.rs/emulator-hal/badge.svg)](https://docs.rs/emulator-hal)
![Minimum Supported Rust Version](https://img.shields.io/badge/rustc-1.60+-blue.svg)

# `emulator-hal-memory`

>  Implementations of the emulator-hal traits relating to memory and adpaters

These basic implementations use a `Vec` to emulate memory, and implement the `BusAccess`
trait of the `emulator-hal` crate.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
