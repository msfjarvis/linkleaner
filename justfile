alias b := build
alias i := install
alias f := fmt
alias r := run
alias t := test
alias u := uninstall
alias v := version

name := `dasel select -f Cargo.toml -s package.name`
version := `dasel select -f Cargo.toml -s package.version`
arch := `dpkg --print-architecture`

set positional-arguments := true
set dotenv-load := true

build type="":
    cargo build {{ type }}

console:
    RUSTFLAGS="--cfg tokio_unstable" cargo run --release --features console

fmt:
    cargo fmt

install:
    cargo deb
    sudo apt -f install ./target/debian/{{ name }}_{{ version }}_{{ arch }}.deb

run type="":
    cargo run {{ type }}

test:
    cargo nextest run

uninstall:
    sudo apt purge -y {{ name }}

version:
    @echo {{ version }}
