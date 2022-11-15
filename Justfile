PROJECT := "shellymon"

set dotenv-load := false

default:
  @just --list | grep -v "    default"

build *args:
  cargo build {{ args }}

run:
  cargo run --bin shellymon

watch:
  cargo-watch --shell 'just run'

simulator:
  cargo run --bin simulator

simulator-watch:
  cargo-watch --shell 'just simulator'

setup:
    docker-compose up -d

teardown:
    docker-compose down -v

test:
    cargo nextest run --workspace

fmt: _fmt _clippy

_fmt:
  cargo fmt

_clippy:
  cargo clippy --fix --allow-dirty --allow-staged

check: fmt test
    cargo check

reset: teardown setup