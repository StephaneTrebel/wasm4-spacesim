APP_NAME := $(shell grep "name = " Cargo.toml | cut -d'"' -f2)
SOURCES := $(shell find . -type f -name "*.rs")
TARGET_APP := target/release/$(APP_NAME)
RELEASE_DIRECTORY :=target/wasm32-unknown-unknown/release
TARGET_CART := $(RELEASE_DIRECTORY)/$(APP_NAME).wasm
TARGET_SNIPPED_CART := $(RELEASE_DIRECTORY)/$(APP_NAME)_snipped.wasm
TARGET_SNIPPED_OPTIMIZED_CART := $(RELEASE_DIRECTORY)/$(APP_NAME)_snipped_optimized.wasm

.DEFAULT: help

.PHONY: help
help:
	@grep -E '^[///a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | \
		sort | \
		awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

.PHONY: install
install: ## Install dependencies
	@echo no deps yet

.PHONY: check
check: ## Check code
	@cargo check

$(TARGET_APP): $(SOURCES) ## Release version of the app
	@cargo build --release

$(TARGET_SNIPPED_OPTIMIZED_CART): $(TARGET_APP) ## Released+Snipped version of the app
	@$(MAKE) -s $(TARGET_APP)
	wasm-snip --snip-rust-fmt-code --snip-rust-panicking-code $(TARGET_CART) > $(TARGET_SNIPPED_CART)
	wasm-strip $(TARGET_SNIPPED_CART)
	wasm-opt $(TARGET_SNIPPED_CART) -Oz --zero-filled-memory --strip-producers --dce --output $(TARGET_SNIPPED_OPTIMIZED_CART)

.PHONY: build
build: ## Build application (in release mode)
	@$(MAKE) -s $(TARGET_APP)

.PHONY: build-snipped-optimized
build-snipped-optimized: ## Build application (in release+snipped mode)
	@$(MAKE) -s $(TARGET_SNIPPED_OPTIMIZED_CART)

.PHONY: build-watch
build-watch: ## Automatic execution upon updates (in release mode)
	@find . -type f -name '*.rs' | entr -c -s "$(MAKE) -s build"

.PHONY: build-snipped-optimized-watch
build-snipped-optimized-watch: ## Automatic execution upon updates (in release mode)
	@find . -type f -name '*.rs' | entr -c -s "$(MAKE) -s build-snipped-optimized"

.PHONY: run
run: ## Run a release version (watch mode in w4)
	@w4 run $(TARGET_CART)

.PHONY: run-snipped-optimized
run-snipped-optimized: ## Run a release+snipped version (watch mode in w4)
	@w4 run $(TARGET_SNIPPED_OPTIMIZED_CART)

.PHONY: prepare-distribution
prepare-distribution: $(TARGET_SNIPPED_OPTIMIZED_CART)
	@w4 bundle $(TARGET_SNIPPED_OPTIMIZED_CART) --title "SpaceRPG" --html dist/index.html
