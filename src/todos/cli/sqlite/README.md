<div align="center">
  <h1>Toor</h1>

<img src='docs/logo.svg' width=80px />

Find project root.

<a href="https://github.com/azzamsa/toor/actions/workflows/ci.yml">
    <img src="https://github.com/azzamsa/toor/actions/workflows/ci.yml/badge.svg" alt="Build status" />
  </a>

<a href="https://crates.io/crates/toor">
    <img src="https://img.shields.io/crates/v/toor.svg">
  </a>

<a href=" https://docs.rs/toor/">
    <img src="https://docs.rs/toor/badge.svg">
  </a>

<a href="https://azzamsa.com/support/">
    <img alt="Sponsor me" src="https://img.shields.io/badge/Sponsor%20Me-%F0%9F%92%96-ff69b4">
  </a>

<p><p/>

</div>

---

Say goodbye to the tedious dance of `cd ..; cd..; cd.. (10x)` or convoluted aliases like `..`, `....`, `........`.
Embrace simplicity with just one command -- introducing `toor`. Bind it to your favorite shell, and voila!
A single `r` keystroke transports you straight to the root directory. Effortlessly elevate your command line experience with a touch of magic.

## Features

- Fancy error message and colorful output.
- Cross-platform and single binary.

## Usage

```bash
🦄 toor --help

rust on master is 📦 v0.1.0 via 🦀 v1.74.0
🦄 toor
/home/user/playground/rust

~/playground
🦄 toor
Error: toor::no_project_root (link)

  × Project root is not found.
  help: Make sure the project root exists.
```

### Integration With Other Tools

#### Fish Shell

```fish
#
# toor
function r # root
    set project_root (toor 2>/dev/null)

    if test -n "$project_root"
        # If successful, change to the project root directory
        cd "$project_root"
        echo "Changed to project root: $project_root"
    else
        # If not successful, stay in current directory
        echo "Project root not found. I dont' go anywhere 📍"
    end
end
```

## Installation

### From Binaries

The [release page](https://github.com/azzamsa/toor/releases) includes
pre-compiled binaries for GNU/Linux, macOS, and Windows.

### From Source

Using [cargo-binstall](https://github.com/cargo-bins/cargo-binstall)

```bash
$ cargo binstall toor
```

Using Rust's package manager [cargo](https://github.com/rust-lang/cargo):

```bash
$ cargo install toor
```

## Development

```bash
git clone https://github.com/azzamsa/toor

# Build
cd toor
cargo build

# Run unit tests and integration tests
cargo test

# Install
cargo install --path .
```

## Contributing

To learn more read [the development guide](docs/dev/README.md)

## Origin of The Name

The term "toor" is a whimsical variation of "root".

## Credits

- [bbatsov's `projectile-project-root`](https://github.com/bbatsov/projectile)
- [Noto Emoji](https://github.com/googlefonts/noto-emoji)
