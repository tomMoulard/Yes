# Makefile for generating React client

# Targets
.PHONY: generate-client test

generate-client:
	pnpm run generate-client

start:
	pnpm run start:dev

lint:
	# do `npx eslint --inspect-config` for more informations.
	tsc --noEmit
	pnpm run lint

lint-fix:
	npx eslint --fix

test:
	pnpm run test

# npm install -g npm-check-updates
update-deps:
	ncu --upgrade
	pnpm install
