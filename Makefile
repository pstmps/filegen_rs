SHELL := /bin/bash
.PHONY: help

help:
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-15s\033[0m %s\n", $$1, $$2}'

clean: ## Clean the project using cargo
	cargo clean

debug: ## Run in debug mode
	export RUST_LOG=info; \
	cargo run -- -r /home/miki/remote_dev/watchfolder -n 100 -s 4 -p

lint: ## Run clippy
	cargo clippy --all-targets --all-features -- -D warnings

build: ## Build the project
	cargo build --release

bump: ## Bump the version number
	@echo "Current version is $(shell cargo pkgid | cut -d# -f2)"
	@read -p "Enter new version number: " version; \
	sed -E "s/^version = .*/version = \"$$version\"/" Cargo.toml > Cargo.toml.tmp && rm Cargo.toml && rm Cargo.lock &&  mv Cargo.toml.tmp Cargo.toml \
	&& cargo check && \
	echo "Updated Cargo.toml to version $$version"


run: ## Run in debug mode
	export RUST_LOG=info; \
	cargo run -- -r /home/miki/remote_dev/watchfolder -n 100 -s 4 -p