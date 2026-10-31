# SigmaOS Accerciser

This `no_std` crate exposes an `Accerciser` model with a name and an in-memory
active flag. `activate()` and `deactivate()` only toggle that flag. The crate
does not inspect an accessibility tree, connect to AT-SPI, or provide a user
interface.

The inline tests exercise the model state only; they do not demonstrate
accessibility inspection or assistive technology integration.

## Build

```sh
cargo check --lib
```

## License

Apache-2.0. See [`Cargo.toml`](Cargo.toml).
