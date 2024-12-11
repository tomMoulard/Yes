# Main Makefile

# Import backend and frontend makefiles
include backend/Makefile
include frontend/Makefile

# Variables
MAIN_DIR := .

# Targets
.PHONY: all build dev lint test

all: build

build:
	$(MAKE) -C backend build
	$(MAKE) -C frontend build

dev:
	$(MAKE) -C backend dev &
	$(MAKE) -C frontend dev

lint:
	cargo fmt --all

test:
	cargo test --all
