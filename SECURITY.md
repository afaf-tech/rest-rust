# Security Policy

## Supported Versions

The following versions of Afaf Rest Rust are currently being supported with security updates:

| Version | Supported          |
| ------- | ------------------ |
| 0.1.x   | :white_check_mark: |

## Reporting a Vulnerability

We take the security of Afaf Rest Rust seriously. If you believe you have found a security vulnerability, please report it to us as described below.

### How to Report

**Please do not report security vulnerabilities through public GitHub issues.**

Instead, please report them via [GitHub Security Advisories](https://github.com/afaf-tech/rest-rust/security/advisories/new).

When reporting a vulnerability, please include:

1. **Description**: A clear description of the vulnerability
2. **Steps to Reproduce**: Detailed steps to reproduce the issue
3. **Impact**: What an attacker could achieve by exploiting this vulnerability
4. **Affected Versions**: Which versions are affected (if known)
5. **Suggested Fix**: If you have a suggestion for how to fix the issue (optional)

### What to Expect

- **Acknowledgment**: We will acknowledge receipt of your vulnerability report within **48 hours**
- **Initial Assessment**: We will provide an initial assessment of the report within **5 business days**
- **Status Updates**: We will keep you informed about our progress toward a fix
- **Resolution**: We aim to resolve critical vulnerabilities within **30 days**

### Disclosure Policy

- We follow a coordinated disclosure process
- We will work with you to understand and resolve the issue quickly
- We will credit you in the security advisory (unless you prefer to remain anonymous)
- We ask that you give us reasonable time to address the issue before public disclosure

### Safe Harbor

We consider security research conducted in accordance with this policy to be:

- Authorized in view of any applicable anti-hacking laws
- Authorized in view of any relevant anti-circumvention laws
- Lawful, helpful to the overall security of the Internet, and conducted in good faith

You are expected, as always, to comply with all applicable laws.

## Security Best Practices for Users

When deploying Afaf Rest Rust in production:

1. **Environment Variables**: Never commit `.env` files or expose secrets in logs
2. **Database**: Use strong passwords and limit database user permissions
3. **Network**: Run the API behind a reverse proxy (nginx, Caddy) with TLS
4. **Updates**: Keep dependencies up to date with `cargo update`
5. **Monitoring**: Enable logging and monitor for unusual activity

## Security Features

This project includes several security-conscious features:

- **SQLx Compile-time Verification**: SQL queries are verified at compile time
- **UUID Primary Keys**: Non-sequential IDs prevent enumeration attacks
- **Input Validation**: All inputs are validated before processing
- **Structured Logging**: Security events can be logged and monitored

## Dependencies

We regularly audit our dependencies for known vulnerabilities using:

```bash
cargo audit
```

If you discover a vulnerability in one of our dependencies, please report it following the process above.
