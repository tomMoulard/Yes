# Makefile for generating React client

# Targets
.PHONY: generate-client test

generate-client:
	pnpm run generate-client

start:
	pnpm run start:dev

lint:
	# do `npx eslint --inspect-config` for more informations.
	npx tsc --noEmit
	pnpm run lint

lint-fix:
	npx eslint --fix

test:
	pnpm run test --coverage .

# npm install -g npm-check-updates
update-deps:
	ncu --upgrade
	pnpm install

# Performance optimization tasks
build:
	pnpm run build

analyze:
	webpack --config webpack.config.js --profile --json > stats.json
