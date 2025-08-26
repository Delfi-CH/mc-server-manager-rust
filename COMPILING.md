# Compiling

Since this software is written in Rust. and doesnt require OS-specific libaries, it can compile where you can compile Rust.

## Compiling with cargo

Install cargo from [here](https://doc.rust-lang.org/stable/cargo/getting-started/installation.html)

In your shell / command processor execute

```
git clone https://github.com/Delfi-CH/mc-server-management.git

cd mc-server-management

cargo build --bin daemon --release
cargo build --bin cli --release
cargo build --bin webapp-backend --release
cargo build --bin install --release
cargo build --bin update --release
```

You will get a executable in ./target/release.