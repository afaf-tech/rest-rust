# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Community health files (CONTRIBUTING.md, CODE_OF_CONDUCT.md, SECURITY.md)
- GitHub issue templates (bug report, feature request)
- Pull request template with checklist
- Docker development environment (Dockerfile, docker-compose.yml)
- Makefile for common development commands
- GitHub Actions CI/CD workflows
- Enhanced .env.example with documentation

### Changed
- Updated README.md with badges and improved documentation

## [0.1.0] - 2024-01-14

### Added
- Initial project structure with domain-driven architecture
- Actix-web REST API server with OpenAPI/Swagger documentation
- User domain with CRUD operations
- PostgreSQL database integration with SQLx
- Database migrations support
- CLI command support via Clap
- Structured logging with Fern
- Input validation for email and other fields
- Standardized JSON response format
- Environment configuration via dotenv

### Security
- UUID primary keys for all entities
- SQL injection prevention via SQLx prepared statements
- Compile-time SQL query verification

[Unreleased]: https://github.com/afaf-tech/rest-rust/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/afaf-tech/rest-rust/releases/tag/v0.1.0
