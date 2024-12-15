# Main Makefile

# Targets
.PHONY: all build dev lint test rustfmt generate

all: test lint build

build:
	$(MAKE) -C backend build

dev:
	$(MAKE) -C backend dev

lint:
	$(MAKE) -C backend lint

test:
	$(MAKE) -C backend test

generate:
	$(MAKE) -C backend openapi
