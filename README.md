[![crates.io](https://img.shields.io/crates/v/emulator-hal.svg)](https://crates.io/crates/emulator-hal)
[![Documentation](https://docs.rs/emulator-hal/badge.svg)](https://docs.rs/emulator-hal)
![Minimum Supported Rust Version](https://img.shields.io/badge/rustc-1.60+-blue.svg)

# `emulator-hal`

>  A set of traits for interfacing between emulated hardware devices

These crates are based on the
[embedded-hal](https://github.com/rust-embedded/embedded-hal) crates.  They are
intended for abstracting emulated hardware devices, primarily the memory bus
and stepping functions common to emulated computer and console hardware, to
allow easier code reuse.

## Crates

| Crate | crates.io | Docs | |
|-|-|-|-|
| [emulator-hal](./emulator-hal) | [![crates.io](https://img.shields.io/crates/v/emulator-hal.svg)](https://crates.io/crates/emulator-hal) | [![Documentation](https://docs.rs/emulator-hal/badge.svg)](https://docs.rs/emulator-hal) | A set of traits for interfacing between emulated hardware devices |
| [emulator-hal-memory](./emulator-hal-memory) | [![crates.io](https://img.shields.io/crates/v/emulator-hal-memory.svg)](https://crates.io/crates/emulator-hal-memory) | [![Documentation](https://docs.rs/emulator-hal-memory/badge.svg)](https://docs.rs/emulator-hal-memory) |  |

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
