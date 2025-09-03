# Rest Rust
 Afaf Rest Rust is an opinionated, domain-based RESTful project structure template designed to promote clean architecture principles and scalable development in Rust. This template provides developers with a solid foundation for building REST APIs while maintaining modularity and domain separation.

## Project Structure
    .
    ├── src/
    │   ├── cmd
    │   ├── config
    │   ├── core/
    │   │   ├── domain
    │   │   └── rest/
    │   │       ├── middleware
    │   │       └── handler
    │   └── pkg
    |── migrations
    └── .env


## Setup

### Prerequisites
- **Rust** (latest stable version)
- **PostgreSQL** server (version 12 or higher)
  - Ensure PostgreSQL is installed and running on your system
  - Create a database for the application (e.g., `rest_rust_db`)

### Database Configuration

1. **Create environment file**: Copy `.env.example` to `.env`:
   ```bash
   cp .env.example .env
   ```

2. **Configure database connection**: Edit the `.env` file and set your PostgreSQL connection URL:
   ```bash
   DATABASE_URL=postgresql://username:password@localhost/rest_rust_db
   REST_URL=localhost:9000
   LOG_DIR=var/log
   LOG_NAME=afaf_rest_rust
   ```
   
   Replace `username`, `password`, and `rest_rust_db` with your PostgreSQL credentials and database name.

### Database Migration

1. **Install SQLx CLI tool**:
   ```bash
   cargo install sqlx-cli --no-default-features --features rustls,postgres
   ```

2. **Run database migrations**:
   ```bash
   sqlx migrate run
   ```

   This will create the necessary tables in your PostgreSQL database.

### Troubleshooting Database Setup

**Common Issues and Solutions:**

- **"Connection refused" error**: 
  - Ensure PostgreSQL server is running: `brew services start postgresql` (macOS) or `sudo systemctl start postgresql` (Linux)
  - Verify the database exists: `createdb rest_rust_db`

- **"Permission denied" error**:
  - Check your PostgreSQL user permissions
  - Ensure the user specified in `DATABASE_URL` has create/write access to the database

- **Migration fails with "relation does not exist"**:
  - Ensure you're connected to the correct database
  - Check that `DATABASE_URL` in `.env` matches your PostgreSQL setup

- **"No such file or directory" when running sqlx**:
  - Install sqlx-cli with the correct features as shown above
  - Restart your terminal after installation
### Run

```bash
cargo run server
```


## License
This project is licensed under the MIT License. For more details, please refer to the LICENSE file included in the repository.


## 👏 Contributing

I would love your help! Contribute by forking the repo and opening pull requests. 
All pull requests should be submitted to the `master` branch.