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
### Requirements
- Rust 
- PostgreSQL

### Setup Config
create `.env` file with content from `.env.example`

### Migrate DB
Install the sqlx CLI tool:
```bash
cargo install sqlx-cli
```
Run the database migrations:
bash
Copy code

```bash
sqlx migrate run
```
### Run

```bash
cargo run server
```


## License
This project is licensed under the MIT License. For more details, please refer to the LICENSE file included in the repository.


## 👏 Contributing

I would love your help! Contribute by forking the repo and opening pull requests. 
All pull requests should be submitted to the `master` branch.