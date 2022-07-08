# uhppote-sys

This crate contains FFI bindings for [uhppoted-dll](https://github.com/uhppoted/uhppoted-dll)
that's part of the [uhppoted](https://github.com/uhppoted/uhppoted) project.

This crate provides low-level and unsafe access to `uhppoted-dll`. Please use ['uhppote-rs'](https://crates.io/crates/uhppote-rs) for safe Rust access to `uhppoted-dll`.

**NOTE** Because `uhppoted-dll` is a Golang library, a working golang installation is necessary to build and use this crate.

## How this is built

Most of this crate is generated in `build.rs`.

- The original `uhppoted-dll` source code can be found in `src/vendoruhppoted-dll` and is compiled using `go build` to compile a static archive.
- The resulting `uhppoted-dll` static library is then copied to the `target/` directory.
- `bindgen` is used to generate the Rust bindings from the `uhppoted-dll` C header file.
- The Rust bindings and static library are then compiled and linked into a single library.

## Building from source

```bash
git submodule update --init --recursive
cargo build --release
```

```

```
