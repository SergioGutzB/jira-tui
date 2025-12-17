.PHONY: help build run release test clean fmt lint check install dev watch clippy bench doc audit deps update-deps size pre-commit all

# Default target
.DEFAULT_GOAL := help

# Variables
BINARY_NAME := jira-tui
CARGO := cargo
RUST_LOG ?= info

# Color output
CYAN := \033[0;36m
GREEN := \033[0;32m
YELLOW := \033[0;33m
RED := \033[0;31m
RESET := \033[0m

##@ General

help: ## Display this help message
	@echo "$(CYAN)Available targets:$(RESET)"
	@awk 'BEGIN {FS = ":.*##"; printf "\n"} /^[a-zA-Z_-]+:.*?##/ { printf "  $(GREEN)%-15s$(RESET) %s\n", $$1, $$2 } /^##@/ { printf "\n$(YELLOW)%s$(RESET)\n", substr($$0, 5) } ' $(MAKEFILE_LIST)

##@ Development

dev: ## Run in development mode with auto-reload
	$(CARGO) watch -x run

run: ## Run the application in debug mode
	RUST_LOG=$(RUST_LOG) $(CARGO) run

build: ## Build in debug mode
	$(CARGO) build

check: ## Check code without building
	$(CARGO) check --all-targets --all-features

watch: ## Watch for changes and check
	$(CARGO) watch -x check

##@ Testing

test: ## Run all tests
	$(CARGO) test --all-features

test-verbose: ## Run tests with verbose output
	$(CARGO) test --all-features -- --nocapture

test-ignored: ## Run ignored tests
	$(CARGO) test --all-features -- --ignored

bench: ## Run benchmarks
	$(CARGO) bench

##@ Code Quality

fmt: ## Format code
	$(CARGO) fmt --all

fmt-check: ## Check code formatting without modifying
	$(CARGO) fmt --all -- --check

lint: clippy ## Alias for clippy

clippy: ## Run clippy linter
	$(CARGO) clippy --all-targets --all-features -- -D warnings

clippy-fix: ## Auto-fix clippy warnings
	$(CARGO) clippy --all-targets --all-features --fix

audit: ## Check for security vulnerabilities
	$(CARGO) audit

##@ Release

release: ## Build optimized release binary
	$(CARGO) build --release
	@echo "$(GREEN)✓ Release binary built at: target/release/$(BINARY_NAME)$(RESET)"

release-run: ## Run optimized release binary
	$(CARGO) run --release

install: release ## Install binary to ~/.cargo/bin
	$(CARGO) install --path .
	@echo "$(GREEN)✓ Installed $(BINARY_NAME) to ~/.cargo/bin$(RESET)"

size: release ## Show binary size
	@echo "$(CYAN)Binary size:$(RESET)"
	@du -h target/release/$(BINARY_NAME)
	@echo ""
	@ls -lh target/release/$(BINARY_NAME)

##@ Documentation

doc: ## Generate and open documentation
	$(CARGO) doc --open --no-deps

doc-all: ## Generate documentation with dependencies
	$(CARGO) doc --open

##@ Dependencies

deps: ## List all dependencies
	$(CARGO) tree

update-deps: ## Update dependencies
	$(CARGO) update

outdated: ## Check for outdated dependencies
	$(CARGO) outdated

##@ Cleanup

clean: ## Remove build artifacts
	$(CARGO) clean
	@echo "$(GREEN)✓ Cleaned build artifacts$(RESET)"

clean-all: clean ## Remove all generated files including Cargo.lock
	rm -f Cargo.lock
	@echo "$(GREEN)✓ Removed Cargo.lock$(RESET)"

##@ Git & CI

pre-commit: fmt lint test ## Run all checks before commit
	@echo "$(GREEN)✓ All pre-commit checks passed!$(RESET)"

ci: fmt-check clippy test ## Run CI checks locally
	@echo "$(GREEN)✓ CI checks passed!$(RESET)"

##@ Utility

all: clean fmt lint test release ## Clean, format, lint, test, and build release
	@echo "$(GREEN)✓ All tasks completed successfully!$(RESET)"

todo: ## Show TODO comments in code
	@rg "TODO|FIXME|XXX|HACK" --color=always || echo "No TODOs found"

lines: ## Count lines of code
	@echo "$(CYAN)Lines of code:$(RESET)"
	@find src -name "*.rs" | xargs wc -l | tail -1

bloat: release ## Analyze binary size
	@which cargo-bloat > /dev/null || (echo "$(RED)cargo-bloat not installed. Run: cargo install cargo-bloat$(RESET)" && exit 1)
	$(CARGO) bloat --release

profile: ## Profile the application
	@which cargo-flamegraph > /dev/null || (echo "$(RED)cargo-flamegraph not installed. Run: cargo install flamegraph$(RESET)" && exit 1)
	$(CARGO) flamegraph

##@ Release Management

tag: ## Create a git tag (usage: make tag VERSION=0.1.0-beta.1)
	@if [ -z "$(VERSION)" ]; then \
		echo "$(RED)Error: VERSION is required. Usage: make tag VERSION=0.1.0-beta.1$(RESET)"; \
		exit 1; \
	fi
	@echo "$(YELLOW)Creating tag v$(VERSION)...$(RESET)"
	git tag -a v$(VERSION) -m "Release v$(VERSION)"
	@echo "$(GREEN)✓ Tag v$(VERSION) created$(RESET)"
	@echo "$(CYAN)Push with: git push origin v$(VERSION)$(RESET)"

release-check: ## Check if ready for release
	@echo "$(CYAN)Checking release readiness...$(RESET)"
	@make fmt-check
	@make clippy
	@make test
	@make release
	@echo "$(GREEN)✓ Ready for release!$(RESET)"

##@ Docker (if needed in future)

docker-build: ## Build Docker image
	docker build -t $(BINARY_NAME):latest .

docker-run: ## Run Docker container
	docker run -it --rm $(BINARY_NAME):latest
