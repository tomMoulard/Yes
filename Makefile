# Main Makefile

# Import backend and frontend makefiles
include backend/Makefile

# Targets
.PHONY: all build dev lint test rustfmt

all: test lint build

build:
	$(MAKE) -C backend build

dev:
	$(MAKE) -C backend dev

lint:
	$(MAKE) -C backend lint

test:
	$(MAKE) -C backend test
