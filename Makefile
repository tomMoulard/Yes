# Main Makefile

# Import backend and frontend makefiles
include backend/Makefile
include frontend/Makefile

# Targets
.PHONY: all build dev lint test rustfmt

all: test lint build

build:
	$(MAKE) -C backend build
	$(MAKE) -C frontend build

dev:
	$(MAKE) -C backend dev &
	$(MAKE) -C frontend dev

lint:
	$(MAKE) -C backend lint
	$(MAKE) -C frontend lint

test:
	$(MAKE) -C backend test
	$(MAKE) -C frontend test
