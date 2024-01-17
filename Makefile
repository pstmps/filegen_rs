SHELL := /bin/bash
.PHONY: help

help:
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-15s\033[0m %s\n", $$1, $$2}'

debug: ## Run in debug mode
	export RUST_LOG=info; \
	cargo run -- -r /home/miki/remote_dev/watchfolder -n 100 -s 4 -p

lint: ## Run clippy
	cargo clippy --all-targets --all-features -- -D warnings

build: ## Build the project
	cargo build --release