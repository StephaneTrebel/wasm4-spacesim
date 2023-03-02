APP_NAME := $(shell grep "name = " Cargo.toml | cut -d'"' -f2)
SOURCES := $(shell find . -type f -name "*.rs")

.DEFAULT: help

.PHONY: help
help:
	@grep -E '^[///a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | \
		sort | \
		awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

.PHONY: install
install: ## Install dependencies
	@echo no deps yet

.PHONY: Check
Check: ## Check code
	@cargo check

target/release/$(APP_NAME): $(SOURCES) ## Release version of the app
	@cargo build --release

target/wasm32-unknown-unknown/release/$(APP_NAME)_snipped.wasm: target/release/$(APP_NAME) ## Released+Snipped version of the app
	@$(MAKE) -s target/release/$(APP_NAME)
	@wasm-snip --snip-rust-fmt-code --snip-rust-panicking-code target/wasm32-unknown-unknown/release/cart.wasm > target/wasm32-unknown-unknown/release/cart_snipped.wasm

.PHONY: build
build: ## Build application (in release mode)
	@$(MAKE) -s target/release/$(APP_NAME)

.PHONY: build-snipped
build-snipped: ## Build application (in release+snipped mode)
	@$(MAKE) -s target/wasm32-unknown-unknown/release/$(APP_NAME)_snipped.wasm

.PHONY: build-watch
build-watch: ## Automatic execution upon updates (in release mode)
	@find . -type f -name '*.rs' | entr -c -s "$(MAKE) -s build"

.PHONY: build-snipped-watch
build-snipped-watch: ## Automatic execution upon updates (in release mode)
	@find . -type f -name '*.rs' | entr -c -s "$(MAKE) -s build-snipped"

.PHONY: run
run: ## Run a release version (watch mode in w4)
	@w4 run target/wasm32-unknown-unknown/release/cart.wasm

.PHONY: run-snipped
run-snipped: ## Run a release+snipped version (watch mode in w4)
	@w4 run target/wasm32-unknown-unknown/release/cart_snipped.wasm

