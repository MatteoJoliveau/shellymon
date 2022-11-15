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

fmt:
  cargo fmt
  cargo clippy --fix --allow-dirty --allow-staged

lint: fmt
  cargo fmt --all -- --check
  cargo clippy -- -D warnings

check: fmt test
    cargo check

reset: teardown setup