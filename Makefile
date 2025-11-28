.PHONY: help dev run test lint fmt clippy check docker-up docker-down docker-build migrate migrate-down clean

# Default target
.DEFAULT_GOAL := help

# Colors for output
CYAN := \033[36m
GREEN := \033[32m
YELLOW := \033[33m
RESET := \033[0m

## help: Show this help message
help:
	@echo "$(CYAN)Afaf Rest Rust - Development Commands$(RESET)"
	@echo ""
	@echo "$(GREEN)Usage:$(RESET)"
	@echo "  make $(YELLOW)<target>$(RESET)"
	@echo ""
	@echo "$(GREEN)Development:$(RESET)"
	@grep -E '^## [a-zA-Z_-]+:' $(MAKEFILE_LIST) | sed 's/## /  /' | sed 's/:/:$(RESET)\t/'

## dev: Start development server with auto-reload (requires cargo-watch)
dev:
	cargo watch -x 'run -- server'

## run: Start the server
run:
	cargo run -- server

## test: Run all tests
test:
	cargo test

## lint: Run clippy and check formatting
lint: clippy fmt-check

## clippy: Run clippy linter
clippy:
	cargo clippy --all-targets -- -D warnings

## fmt: Format code with rustfmt
fmt:
	cargo fmt

## fmt-check: Check code formatting without modifying files
fmt-check:
	cargo fmt --check

## check: Quick compilation check
check:
	cargo check

## build: Build release binary
build:
	cargo build --release

## clean: Clean build artifacts
clean:
	cargo clean

# =============================================================================
# Database Commands
# =============================================================================

## migrate: Run database migrations
migrate:
	sqlx migrate run

## migrate-down: Rollback last migration
migrate-down:
	sqlx migrate revert

## migrate-create: Create a new migration (usage: make migrate-create name=migration_name)
migrate-create:
	sqlx migrate add $(name)

# =============================================================================
# Docker Commands
# =============================================================================

## docker-up: Start Docker development environment
docker-up:
	docker-compose up -d

## docker-down: Stop Docker development environment
docker-down:
	docker-compose down

## docker-build: Build Docker images
docker-build:
	docker-compose build

## docker-logs: Show Docker container logs
docker-logs:
	docker-compose logs -f

## docker-migrate: Run migrations in Docker
docker-migrate:
	docker-compose run --rm migrate

## docker-clean: Stop containers and remove volumes
docker-clean:
	docker-compose down -v

## docker-shell: Open shell in app container
docker-shell:
	docker-compose exec app bash

## docker-db: Open psql shell in database container
docker-db:
	docker-compose exec db psql -U postgres -d rest_rust_db

# =============================================================================
# CI Commands
# =============================================================================

## ci: Run all CI checks (lint, test, build)
ci: lint test build
	@echo "$(GREEN)All CI checks passed!$(RESET)"

## audit: Run security audit on dependencies
audit:
	cargo audit

# =============================================================================
# Documentation Commands
# =============================================================================

## docs: Generate and open documentation
docs:
	cargo doc --open

## docs-build: Generate documentation
docs-build:
	cargo doc --no-deps
