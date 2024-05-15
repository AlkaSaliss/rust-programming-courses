SHELL := /bin/bash
.PHONY: help

help:
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

clean: ## Clean build artifacts using cargo
	cargo clean
build: ## Build math-utils library using cargo
	cargo build
lint: ## Lint math-utils library using cargo
	@rustup component add clippy 2> /dev/null
	cargo clippy
test: ## Run math-utils tests using cargo
	cargo test
format: ## Format math-utils library using cargo
	@rustup component add rustfmt 2> /dev/null
	cargo fmt
doc: ## Generate math-utils documentation using cargo
	cargo doc
bump: ## Create a new bump version
	@echo "Current version: $(shell cargo pkgid | cut -d# -f2)"
	@read -p "Enter new version: " version;
	## TODO: finish
