# Makefile for building and running the backend



# Targets
.PHONY: all build dev

all: test lint build

build:
	cargo build --release

dev: start-db
	cargo run

lint:
	cargo fmt --all -- --check

apply-lint:
	cargo fmt --all -- --emit files

test:
	cargo test --all

# cargo install -f --no-default-features --features upgrade
update-deps:
	cargo upgrade -i allow && cargo update

# Generate OpenAPI documentation
openapi:
	cargo run --bin bidding_app -- generate-openapi > openapi.json

start-db:
	-docker-compose -f ../.devcontainers/docker-compose.yml up -d

stop-db:
	-docker-compose -f ../.devcontainers/docker-compose.yml down

install-cargo:
	curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
	source ${HOME}/.cargo/env
