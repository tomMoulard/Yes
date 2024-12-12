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

test:
	cargo test --all
