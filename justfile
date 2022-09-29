alias b := build
alias c := check
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

_default:
    just --list

build *args:
    cargo build {{ args }}

check *args:
    cargo check {{ args }}

clippy *args:
    cargo clippy {{ args }}

console:
    RUSTFLAGS="--cfg tokio_unstable" cargo run --release --features console

fmt:
    cargo fmt

install:
    RUSTFLAGS="--cfg tokio_unstable" cargo deb
    sudo apt -f install --reinstall ./target/debian/{{ name }}_{{ version }}_{{ arch }}.deb

log:
    sudo journalctl -xeu walls-bot-rs.service

run type="":
    cargo run {{ type }}

start:
    sudo systemctl start walls-bot-rs.service

status:
    sudo systemctl status walls-bot-rs.service

stop:
    sudo systemctl stop walls-bot-rs.service

test:
    cargo nextest run

uninstall:
    sudo apt purge -y {{ name }}

version:
    @echo {{ version }}
