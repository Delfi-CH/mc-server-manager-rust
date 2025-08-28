# Compiling

## Additional Dependencies

### Linux-only

- OpenSSl Development libaries
- pkg-config

Install via Package-Manager:

**Debian/Ubuntu**

```bash
    sudo apt update
    sudo apt upgrade
    sudo apt install pkg-config libssl-dev
```

**Fedora/RHEL**

```bash
    sudo dnf update
    sudo dnf install pkgconf-pkg-config openssl-devel
```
**Arch Linux**

```bash
    sudo pacman -Syu pkgconf openssl
```

**Alpine Linux**

```bash
    sudo apk update
    sudo apk upgrade
    sudo apk add pkgconfig openssl-dev
```

**OpenSUSE**

```bash
    sudo zypper update
    sudo zypper refresh
    sudo zypper install pkg-config libopenssl-devel
```

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