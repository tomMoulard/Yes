# Makefile for building and running the backend

# Targets
.PHONY: all build dev

all: test lint build

build:
	cargo build --release

dev:
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

