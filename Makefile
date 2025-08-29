SHELL := /bin/bash
.PHONY: help

help:
	@grep -E '^[a-zA-Z_-]+:' Makefile | sed 's/:.*#/#/' | column -t -s '#'

clean: ## Clean the project using cargo. Also runs format.
	@rustup component add rustfmt 2> /dev/null
	cargo clean
	cargo fmt -- --check

build: ## Build the project using cargo
	cargo build

test: ## Run tests using cargo
	cargo test

fmt: ## Format the code
	@rustup component add rustfmt 2> /dev/null
	cargo fmt -- --check

lint: ## Check code style and check for errors and warnings. Also checks formatting.
	@rustup component add clippy 2> /dev/null
	cargo fmt -- --check
	cargo clippy -- -D warnings

version: ## Increment the version number - not recommended. Prefer using cargo release.
	@echo "Current version is $(shell cargo pkgid | sed 's/.*#//')"
	@read -p "Enter new version: " version; \
	updated_version=$$(cargo pkgid | cut -d# -f2 | sed -E "s/[0-9]+\.[0-9]+\.[0-9]+/$$version/"); \
	sed -i '' -E "s/^version = .*/version = \"$$updated_version\"/" Cargo.toml
	@echo "New version is $$(cargo pkgid | cut -d# -f2)"