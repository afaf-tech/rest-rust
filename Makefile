# Makefile for AFAF REST Rust API

# Default target
.DEFAULT_GOAL := help

# Variables
CARGO := cargo
DATABASE_URL := postgresql://postgres:secret@localhost/afaf_rest_rust

# Colors for output
BOLD := \033[1m
GREEN := \033[32m
YELLOW := \033[33m
RED := \033[31m
NC := \033[0m # No Color

##@ Development Commands

.PHONY: help
help: ## Display this help
	@echo "$(BOLD)AFAF REST Rust API - Development Commands$(NC)"
	@echo ""
	@awk 'BEGIN {FS = ":.*##"; printf "Usage:\n  make $(YELLOW)<target>$(NC)\n"} /^[a-zA-Z_0-9-]+:.*?##/ { printf "  $(YELLOW)%-15s$(NC) %s\n", $$1, $$2 } /^##@/ { printf "\n$(BOLD)%s$(NC)\n", substr($$0, 5) } ' $(MAKEFILE_LIST)

.PHONY: install
install: ## Install dependencies
	@echo "$(GREEN)Installing dependencies...$(NC)"
	$(CARGO) --version
	$(CARGO) install sqlx-cli --no-default-features --features rustls,postgres

##@ Build Commands

.PHONY: check
check: ## Quick compile check
	@echo "$(GREEN)Running cargo check...$(NC)"
	$(CARGO) check

.PHONY: build
build: ## Build the project
	@echo "$(GREEN)Building project...$(NC)"
	$(CARGO) build

.PHONY: build-release
build-release: ## Build for release
	@echo "$(GREEN)Building release version...$(NC)"
	$(CARGO) build --release

.PHONY: clean
clean: ## Clean build artifacts
	@echo "$(GREEN)Cleaning build artifacts...$(NC)"
	$(CARGO) clean

##@ Test Commands

.PHONY: test
test: ## Run all tests
	@echo "$(GREEN)Running tests...$(NC)"
	$(CARGO) test

.PHONY: test-verbose
test-verbose: ## Run tests with verbose output
	@echo "$(GREEN)Running tests (verbose)...$(NC)"
	$(CARGO) test -- --nocapture

.PHONY: test-unit
test-unit: ## Run unit tests only
	@echo "$(GREEN)Running unit tests...$(NC)"
	$(CARGO) test --lib

.PHONY: test-integration
test-integration: ## Run integration tests only
	@echo "$(GREEN)Running integration tests...$(NC)"
	$(CARGO) test --test "*"

.PHONY: test-users
test-users: ## Run user domain tests
	@echo "$(GREEN)Running user domain tests...$(NC)"
	$(CARGO) test users

##@ Database Commands

.PHONY: db-setup
db-setup: ## Set up database (create and migrate)
	@echo "$(GREEN)Setting up database...$(NC)"
	@echo "Creating database..."
	-docker exec postgres_container createdb -U postgres afaf_rest_rust 2>/dev/null || echo "Database might already exist"
	$(MAKE) db-migrate

.PHONY: db-migrate
db-migrate: ## Run database migrations
	@echo "$(GREEN)Running database migrations...$(NC)"
	sqlx migrate run

.PHONY: db-migrate-revert
db-migrate-revert: ## Revert last migration
	@echo "$(YELLOW)Reverting last migration...$(NC)"
	sqlx migrate revert

.PHONY: db-reset
db-reset: ## Reset database (drop, create, migrate)
	@echo "$(YELLOW)Resetting database...$(NC)"
	@echo "Terminating connections..."
	-docker exec postgres_container psql -U postgres -c "SELECT pg_terminate_backend(pg_stat_activity.pid) FROM pg_stat_activity WHERE pg_stat_activity.datname = 'afaf_rest_rust' AND pid <> pg_backend_pid();" 2>/dev/null
	@echo "Dropping database..."
	-docker exec postgres_container dropdb -U postgres afaf_rest_rust 2>/dev/null
	@echo "Creating database..."
	docker exec postgres_container createdb -U postgres afaf_rest_rust
	$(MAKE) db-migrate

.PHONY: db-status
db-status: ## Check migration status
	@echo "$(GREEN)Checking migration status...$(NC)"
	sqlx migrate info

##@ Server Commands

.PHONY: run
run: ## Run the server
	@echo "$(GREEN)Starting server...$(NC)"
	$(CARGO) run server

.PHONY: run-dev
run-dev: ## Run server with auto-reload (requires cargo-watch)
	@echo "$(GREEN)Starting server in development mode...$(NC)"
	@command -v cargo-watch >/dev/null 2>&1 || { echo "$(RED)cargo-watch not found. Install with: cargo install cargo-watch$(NC)"; exit 1; }
	cargo watch -x "run server"

.PHONY: run-cli
run-cli: ## Run CLI with task (usage: make run-cli TASK=task_name)
	@echo "$(GREEN)Running CLI task: $(TASK)$(NC)"
	$(CARGO) run cli --task "$(TASK)"

##@ Code Quality Commands

.PHONY: fmt
fmt: ## Format code
	@echo "$(GREEN)Formatting code...$(NC)"
	$(CARGO) fmt

.PHONY: fmt-check
fmt-check: ## Check code formatting
	@echo "$(GREEN)Checking code formatting...$(NC)"
	$(CARGO) fmt --check

.PHONY: clippy
clippy: ## Run clippy linter
	@echo "$(GREEN)Running clippy...$(NC)"
	$(CARGO) clippy -- -D warnings

.PHONY: fix
fix: ## Fix common issues automatically
	@echo "$(GREEN)Fixing issues automatically...$(NC)"
	$(CARGO) fix --allow-dirty --allow-staged
	$(CARGO) clippy --fix --allow-dirty --allow-staged

##@ Development Workflow

.PHONY: dev-setup
dev-setup: install db-setup ## Complete development setup
	@echo "$(GREEN)Development setup complete!$(NC)"
	@echo "$(YELLOW)Next steps:$(NC)"
	@echo "  1. Update .env with your configuration"
	@echo "  2. Run 'make run' to start the server"
	@echo "  3. Visit http://localhost:9000/swagger-ui/ for API docs"

.PHONY: ci
ci: fmt-check clippy test ## Run CI checks (format, lint, test)
	@echo "$(GREEN)All CI checks passed!$(NC)"

.PHONY: pre-commit
pre-commit: fmt clippy test ## Run pre-commit checks (format, lint, test)
	@echo "$(GREEN)Pre-commit checks completed!$(NC)"

.PHONY: full-check
full-check: clean build test clippy fmt-check ## Full project check
	@echo "$(GREEN)Full project check completed successfully!$(NC)"

##@ Docker Commands

.PHONY: docker-db-start
docker-db-start: ## Start PostgreSQL in Docker
	@echo "$(GREEN)Starting PostgreSQL container...$(NC)"
	@if [ $$(docker ps -q -f name=postgres_container) ]; then \
		echo "$(YELLOW)PostgreSQL container already running$(NC)"; \
	else \
		docker start postgres_container || \
		docker run --name postgres_container -e POSTGRES_PASSWORD=secret -p 5432:5432 -d postgres:13; \
	fi

.PHONY: docker-db-stop
docker-db-stop: ## Stop PostgreSQL container
	@echo "$(YELLOW)Stopping PostgreSQL container...$(NC)"
	-docker stop postgres_container

.PHONY: docker-db-logs
docker-db-logs: ## Show PostgreSQL container logs
	@echo "$(GREEN)PostgreSQL container logs:$(NC)"
	docker logs postgres_container

##@ Documentation

.PHONY: docs
docs: ## Generate and open documentation
	@echo "$(GREEN)Generating documentation...$(NC)"
	$(CARGO) doc --open

.PHONY: swagger
swagger: ## Open Swagger UI (requires server to be running)
	@echo "$(GREEN)Opening Swagger UI...$(NC)"
	@echo "Make sure the server is running with 'make run'"
	@command -v open >/dev/null 2>&1 && open http://localhost:9000/swagger-ui/ || echo "Visit http://localhost:9000/swagger-ui/"

##@ Utility Commands

.PHONY: env-check
env-check: ## Check environment setup
	@echo "$(GREEN)Environment Check:$(NC)"
	@echo "Rust version: $$(rustc --version)"
	@echo "Cargo version: $$(cargo --version)"
	@echo "SQLx CLI: $$(sqlx --version 2>/dev/null || echo '$(RED)Not installed$(NC)')"
	@echo "Docker: $$(docker --version 2>/dev/null || echo '$(RED)Not available$(NC)')"
	@echo "PostgreSQL container: $$(docker ps --format 'table {{.Names}}\t{{.Status}}' | grep postgres_container || echo '$(RED)Not running$(NC)')"
	@echo ""
	@echo "Environment files:"
	@echo "  .env: $$([ -f .env ] && echo '$(GREEN)✓ exists$(NC)' || echo '$(RED)✗ missing$(NC)')"
	@echo "  CLAUDE.md: $$([ -f CLAUDE.md ] && echo '$(GREEN)✓ exists$(NC)' || echo '$(RED)✗ missing$(NC)')"

.PHONY: logs
logs: ## Show application logs
	@echo "$(GREEN)Showing logs from var/log/$(NC)"
	@if [ -d "var/log" ]; then \
		tail -f var/log/afaf_rest_rust.log 2>/dev/null || echo "$(YELLOW)No log files found$(NC)"; \
	else \
		echo "$(YELLOW)Log directory not found$(NC)"; \
	fi
>>>>>>> f3d2c72 (feat: implement comprehensive authentication and authorization system)
