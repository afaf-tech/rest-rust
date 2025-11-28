# Architecture

This document provides an overview of the Afaf Rest Rust architecture for contributors and developers.

## Overview

Afaf Rest Rust follows a **domain-driven design (DDD)** approach with clear separation of concerns. The architecture is designed to be modular, testable, and maintainable.

```
┌─────────────────────────────────────────────────────────────────┐
│                         HTTP Request                             │
└─────────────────────────────────────────────────────────────────┘
                                │
                                ▼
┌─────────────────────────────────────────────────────────────────┐
│                      Actix-web Server                            │
│                     (src/cmd/server.rs)                          │
└─────────────────────────────────────────────────────────────────┘
                                │
                                ▼
┌─────────────────────────────────────────────────────────────────┐
│                        Middleware                                │
│                (src/core/rest/middleware/)                       │
│            - Logging, Auth, Error Handling, etc.                 │
└─────────────────────────────────────────────────────────────────┘
                                │
                                ▼
┌─────────────────────────────────────────────────────────────────┐
│                         Router                                   │
│                  (src/core/rest/router.rs)                       │
│              Maps HTTP routes to handlers                        │
└─────────────────────────────────────────────────────────────────┘
                                │
                                ▼
┌─────────────────────────────────────────────────────────────────┐
│                        Handlers                                  │
│                 (src/core/rest/handler/)                         │
│    - Validate input                                              │
│    - Call repository methods                                     │
│    - Build standardized responses                                │
└─────────────────────────────────────────────────────────────────┘
                                │
                                ▼
┌─────────────────────────────────────────────────────────────────┐
│                      Domain Layer                                │
│                   (src/core/domain/)                             │
│    ┌─────────────────┐    ┌─────────────────┐                   │
│    │   Model         │    │   Repository    │                   │
│    │ (Data structs)  │◄───│ (DB operations) │                   │
│    └─────────────────┘    └─────────────────┘                   │
└─────────────────────────────────────────────────────────────────┘
                                │
                                ▼
┌─────────────────────────────────────────────────────────────────┐
│                      PostgreSQL                                  │
│                   (via SQLx + PgPool)                            │
└─────────────────────────────────────────────────────────────────┘
```

## Directory Structure

```
src/
├── main.rs              # Entry point, CLI argument parsing
├── lib.rs               # Library exports
├── cmd/                 # Command handlers
│   ├── mod.rs
│   ├── server.rs        # REST API server startup
│   └── cli.rs           # CLI commands
├── config/              # Configuration management
│   └── mod.rs           # Environment variable loading
├── core/                # Core application logic
│   ├── mod.rs
│   ├── domain/          # Business domains
│   │   ├── mod.rs
│   │   └── users/       # Example domain
│   │       ├── mod.rs
│   │       ├── model.rs      # User entity
│   │       └── repository.rs # Database operations
│   └── rest/            # HTTP layer
│       ├── mod.rs
│       ├── router.rs    # Route definitions
│       ├── openapi.rs   # Swagger/OpenAPI config
│       ├── handler/     # Request handlers
│       │   ├── mod.rs
│       │   ├── users.rs
│       │   ├── response.rs   # Response builders
│       │   └── validator.rs  # Input validation
│       └── middleware/  # HTTP middleware
└── pkg/                 # Shared utilities
    ├── mod.rs
    └── logger/          # Logging configuration
```

## Key Design Decisions

### 1. Domain Isolation

Each domain (e.g., `users`) is self-contained with its own:
- **Model**: Data structures with SQLx and Serde derives
- **Repository**: Database access methods

This isolation allows:
- Easy addition of new domains without affecting existing code
- Clear ownership and boundaries
- Independent testing

### 2. Repository Pattern

All database access goes through repository structs:

```rust
pub struct UserRepository<'a> {
    pool: &'a PgPool,
}

impl<'a> UserRepository<'a> {
    pub async fn find_all(&self) -> Result<Vec<User>, sqlx::Error> {
        sqlx::query_as::<_, User>("SELECT * FROM users")
            .fetch_all(self.pool)
            .await
    }
}
```

Benefits:
- **Testability**: Repositories can be mocked
- **Compile-time SQL verification**: SQLx validates queries at compile time
- **Centralized data access**: All SQL lives in one place per domain

### 3. Standardized Response Format

All API responses use a consistent JSON structure:

```json
// Success
{
  "meta": { "version": "1.0.0", "app": "afaf-rest-rust" },
  "data": { ... },
  "message": "Operation successful"
}

// Error
{
  "meta": { "version": "1.0.0", "app": "afaf-rest-rust" },
  "error": "bad_request",
  "message": "Invalid email format"
}
```

### 4. Handler Validation

Handlers validate input before calling repositories:

```rust
#[post("/users")]
pub async fn create_user(/* ... */) -> HttpResponse {
    // 1. Validate input
    if !is_valid_email(&payload.email) {
        return HttpResponse::BadRequest()
            .json(build_error_response("bad_request", "Invalid email"));
    }
    
    // 2. Call repository
    let user = repo.create(&payload).await;
    
    // 3. Return response
    HttpResponse::Ok().json(build_success_response(user, "User created"))
}
```

### 5. UUID Primary Keys

All entities use UUIDs as primary keys:
- Prevents enumeration attacks
- Enables distributed ID generation
- No database round-trip needed for ID generation

## Data Flow

1. **HTTP Request** arrives at Actix-web server
2. **Middleware** processes the request (logging, auth, etc.)
3. **Router** matches the path to a handler
4. **Handler**:
   - Extracts and validates input
   - Calls repository methods
   - Builds and returns response
5. **Repository** executes SQL via SQLx
6. **PostgreSQL** returns data
7. **Response** is serialized to JSON and sent

## Adding a New Domain

1. Create domain directory: `src/core/domain/products/`
2. Add `model.rs` with SQLx derives:
   ```rust
   #[derive(Debug, Clone, Serialize, FromRow, ToSchema)]
   pub struct Product {
       pub id: Uuid,
       pub name: String,
       pub price: i64,
   }
   ```
3. Add `repository.rs` with database methods
4. Add handler in `src/core/rest/handler/products.rs`
5. Register routes in `src/core/rest/router.rs`
6. Register schemas in `src/core/rest/openapi.rs`
7. Create migration: `sqlx migrate add create_products_table`

## Testing Strategy

- **Unit tests**: In-module tests for pure functions
- **Integration tests**: In `tests/` directory with real PostgreSQL
- **Repository tests**: Use unique UUIDs to avoid conflicts

Run all tests:
```bash
cargo test
```

## Dependencies

| Crate | Purpose |
|-------|---------|
| `actix-web` | HTTP server framework |
| `sqlx` | Async PostgreSQL driver with compile-time verification |
| `serde` | JSON serialization/deserialization |
| `tokio` | Async runtime |
| `utoipa` | OpenAPI/Swagger documentation |
| `uuid` | UUID generation |
| `clap` | CLI argument parsing |
| `fern` | Logging |
