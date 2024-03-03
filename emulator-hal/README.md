[![crates.io](https://img.shields.io/crates/v/emulator-hal.svg)](https://crates.io/crates/emulator-hal)
[![Documentation](https://docs.rs/emulator-hal/badge.svg)](https://docs.rs/emulator-hal)
![Minimum Supported Rust Version](https://img.shields.io/badge/rustc-1.60+-blue.svg)

# `emulator-hal`

>  A set of traits for interfacing between emulated hardware devices

## Design Goals

- Must be useable using only static components, to allow maximum performance for simple
  static emulators, and test components

- Must be able to create a trait object from these traits, to allow maximum modularity for
  complex emulators

- Create abstractions that could be used either in a block or non-blocking (async) execution
  model

- Must be minimal, to abstract the most common parts needed to build an emulator, without
  trying to support all possible extended features

- Make it possible to abstract CPU, memory, and peripherals emulation implementations that
  can be reused between different emulator ecosystems

## Out of Scope

- a complete framework for constructing an emulator.  These traits are meant to only be a
  minimal interface to the most core functions of components used during runtime to make
  it easier to implement the glue to reuse existing components within a larger system

