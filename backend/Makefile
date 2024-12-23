# Makefile for building and running the backend

# Targets
.PHONY: all build dev migrate-db

all: test lint build

build:
	cargo build --release

dev: start-db
	# go to http://localhost:8080/docs/ for documentation
	cargo run

lint: lint-clippy lint-fmt

lint-clippy:
	cargo clippy -- -D warnings

lint-fmt:
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
	-docker-compose -f ../.devcontainers/docker-compose.yml up -d --remove-orphans

stop-db:
	-docker-compose -f ../.devcontainers/docker-compose.yml down

migrate-db:
	for file in backend/migrations/*.sql; do \
		psql $(PG__URL) -f $$file; \
	done
