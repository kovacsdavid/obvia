# Obvia ERP Development Guidelines

This document provides project-specific development guidelines for the Obvia ERP system.

---

## 1. Build and Configuration

### Project Structure

The backend is a **single Cargo package** located in the `backend/` directory:
- `backend/src/main.rs` - The main application entry point
- `backend/src/common/` - Common utilities and types
- `backend/src/manager/` - System-wide operations (authentication, users, tenants)
- `backend/src/tenant/` - Tenant-specific business logic

The package uses **Rust Edition 2024**.

### Building the Project

From the `backend/` directory:

```bash
# Development build
cargo build

# Release build (optimized)
cargo build --release

# Run the application
cargo run
```

### Configuration Files

The backend uses TOML configuration files located in `backend/config/`:

1. Copy the example configuration:
   ```bash
   cp config/default.toml.example config/default.toml
   ```

2. Edit `config/default.toml` with your database credentials and settings.

Key configuration sections include:
- Server settings (host, port)
- Main database configuration
- Default tenant database configuration
- Authentication settings (JWT secret, issuer, audience, expiration)

### Code Quality Tools

Run these commands from the `backend/` directory:

```bash
# Format code
cargo fmt

# Lint code
cargo clippy

# Check for compilation errors without building
cargo check
```

**Important**: The package is configured to **deny unsafe code** via Clippy metadata in `backend/Cargo.toml`:

```toml
[package.metadata]
clippy = { deny = ["unsafe_code"] }
```

Additionally, `backend/src/main.rs` contains:

```rust
#![forbid(unsafe_code)]
```

This means **no unsafe code is allowed** in the codebase.

---

## 2. Testing

### Test Organization

Tests in this project are **co-located with the source code** using inline test modules:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_something() {
        // test code
    }
}
```

This pattern is used throughout the codebase (see examples in `backend/src/manager/auth/dto/claims.rs`, `backend/src/manager/auth/repository.rs`, etc.).

### Running Tests

From the `backend/` directory:

```bash
# Run all tests
cargo test

# Run tests with output (show println! statements)
cargo test -- --nocapture

# Run specific test module
cargo test <module_name>

# Run a specific test function
cargo test <test_function_name>
```

**Example test run output:**

```
$ cargo test test_example
   Compiling obvia_backend v0.19.0 (/path/to/backend)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 9.51s
     Running unittests src/main.rs (target/debug/deps/obvia_backend-37eadbf8e46e7ac8)
running 5 tests
test test_example::tests::test_add_mixed_numbers ... ok
test test_example::tests::test_add_negative_numbers ... ok
test test_example::tests::test_add_positive_numbers ... ok
test test_example::tests::test_greet_empty_string ... ok
test test_example::tests::test_greet_simple_name ... ok
test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 493 filtered out
```

### Test Dependencies

The project uses these testing libraries (defined in `backend/Cargo.toml`):

```toml
[dev-dependencies]
pretty_assertions = "1.4.1"
mockall = "0.13.1"
```

- **pretty_assertions**: Provides better assertion failure messages with colored diffs
- **mockall**: Enables mocking of traits for unit testing

### Using pretty_assertions

Replace standard assertions with pretty_assertions for better error messages:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;  // Import from pretty_assertions
    
    #[test]
    fn test_example() {
        assert_eq!(actual_value, expected_value);
    }
}
```

### Mocking with mockall

Traits can be automatically mocked using the `#[cfg_attr(test, automock)]` attribute:

```rust
#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, automock)]
#[async_trait]
pub trait SomeRepository: Send + Sync + 'static {
    async fn some_method(&self, param: &Type) -> Result<(), Error>;
}
```

During tests, mockall generates a `MockSomeRepository` that can be used:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_with_mock() {
        let mut mock_repo = MockSomeRepository::new();
        mock_repo.expect_some_method()
            .returning(|_| Ok(()));
        
        // Use mock_repo in your test
    }
}
```

### Adding New Tests

1. Add a `#[cfg(test)]` module at the bottom of your source file
2. Import necessary items with `use super::*;`
3. Import `pretty_assertions::assert_eq` if needed
4. Write test functions with `#[test]` attribute
5. For async tests, use `#[tokio::test]` instead

**Example:**

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    
    #[test]
    fn test_my_function() {
        let result = my_function(input);
        assert_eq!(result, expected);
    }
    
    #[tokio::test]
    async fn test_async_function() {
        let result = async_function().await;
        assert!(result.is_ok());
    }
}
```

---

## 3. Architecture and Design Patterns

### Multi-Tenant Architecture

The application is structured around a multi-tenant architecture:

- **Manager Module** (`backend/src/manager/`): Handles system-wide operations including authentication, user management, and tenant management
- **Tenant Module** (`backend/src/tenant/`): Contains tenant-specific business logic (products, taxes, services, customers, etc.)

### ValueObject Pattern

**This is a critical pattern used throughout the codebase.**

The project extensively uses a ValueObject pattern for type-safe domain modeling with validation. This pattern wraps primitive types in strongly-typed wrappers that enforce validation rules.

**Key components:**

1. **ValueObject struct** (`backend/src/common/types/value_object.rs`): A generic wrapper around domain types
2. **ValueObjectable trait**: Defines validation and value extraction methods

**Implementation pattern:**

```rust
use crate::common::types::value_object::{ValueObject, ValueObjectable};
use std::fmt::Display;

// 1. Define your domain type
pub struct Email(pub String);

// 2. Implement Display for your type
impl Display for Email {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

// 3. Implement ValueObjectable with validation logic
impl ValueObjectable for Email {
    type DataType = String;
    
    fn validate(&self) -> Result<(), String> {
        if self.0.contains('@') && !self.0.is_empty() {
            Ok(())
        } else {
            Err("Invalid email format".to_string())
        }
    }
    
    fn get_value(&self) -> &Self::DataType {
        &self.0
    }
}

// 4. Optionally implement custom Deserialize for ValueObject<Email>
impl<'de> Deserialize<'de> for ValueObject<Email> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        ValueObject::new(Email(s)).map_err(serde::de::Error::custom)
    }
}
```

**Usage:**

```rust
// Creating a ValueObject with validation
let email = ValueObject::new(Email("user@example.com".to_string()))?;

// Accessing the inner value
let email_string = email.extract().get_value();

// In structs
pub struct User {
    pub email: ValueObject<Email>,
    pub name: ValueObject<Name>,
}
```

**Benefits:**
- Compile-time type safety
- Validation enforced at creation
- Self-documenting code
- Prevention of primitive obsession

**Examples in codebase:**
- `backend/src/manager/tenants/types/db_name.rs`
- `backend/src/manager/tenants/types/db_host.rs`
- `backend/src/tenant/taxes/types/` (various types)
- `backend/src/tenant/services/types/` (various types)

### Optional String Validation Macro

The codebase includes a macro for validating optional string fields:

```rust
// From backend/src/tenant/taxes/dto.rs
let optional_field = validate_optional_string!(FieldType(value.field), error.field);
```

This macro handles the common pattern of validating an optional string and converting it to an `Option<ValueObject<T>>`.

### Error Handling Pattern

The project uses custom error types with `thiserror` and form error responses:

```rust
use thiserror::Error;
use serde::{Serialize, Deserialize};
use axum::response::{IntoResponse, Response};

#[derive(Debug, Serialize, Default)]
pub struct MyFormError {
    pub field1: Option<String>,
    pub field2: Option<String>,
}

impl FormErrorResponse for MyFormError {}

impl IntoResponse for MyFormError {
    fn into_response(self) -> Response {
        self.get_error_response()
    }
}
```

### Database Migrations

Database migrations are located in:
- `backend/migrations/main/` - Main database schema
- `backend/migrations/tenant/` - Tenant-specific schema

Migrations are automatically executed on application startup using SQLx.

### Async/Await with Tokio

The project uses Tokio as the async runtime with full features enabled:

```toml
tokio = { version = "1.45.1", features = ["full"] }
```

All async code should use `async/await` syntax and the `#[tokio::test]` attribute for async tests.

---

## 4. Code Style and Conventions

### Module Organization

- Use `#![allow(clippy::module_inception)]` when you have a module with the same name as a submodule
- Use `pub(crate)` for internal module visibility
- Group related functionality into submodules

### Documentation

All public items should have documentation comments:

```rust
/// Brief description of the function.
///
/// # Parameters
/// - `param1` - Description of param1
///
/// # Returns
/// - Description of return value
///
/// # Errors
/// - Description of when errors occur
pub fn my_function(param1: Type) -> Result<ReturnType, Error> {
    // implementation
}
```

### Naming Conventions

- Types use `PascalCase`
- Functions and variables use `snake_case`
- Constants use `SCREAMING_SNAKE_CASE`
- Modules use `snake_case`

### File Headers

All source files must include the AGPL license header:

```rust
/*
 * This file is part of the Obvia ERP.
 *
 * Copyright (C) 2025 Kovács Dávid <kapcsolat@kovacsdavid.dev>
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published
 * by the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */
```

### Commit Conventions

All commits **must be signed off** using the Developer Certificate of Origin (DCO):

```bash
git commit -s -m "Your commit message"
```

This adds a `Signed-off-by: Your Name <your.email@example.com>` line to the commit message.

**Pull requests with unsigned commits will not be merged.**

---

## 5. Dependencies Overview

### Backend Core Dependencies

- **axum** (0.8.4): Web framework with macros feature
- **axum-extra** (0.10.1): Cookie handling and typed headers
- **tokio** (1.45.1): Async runtime (full features)
- **serde** (1.0.219): Serialization/deserialization
- **sqlx** (0.8.6): Async PostgreSQL driver with UUID, Chrono, BigDecimal support
- **uuid** (1.17.0): UUID generation (v4) and serialization
- **chrono** (0.4.41): Date/time handling with serde support
- **jsonwebtoken** (10.0.0): JWT token handling
- **argon2** (0.5.3): Password hashing
- **config** (0.15.11): TOML configuration file parsing
- **thiserror** (2.0.12): Error type derivation
- **anyhow** (1.0.98): Error handling utilities
- **regex** (1.11.1): Regular expressions
- **bigdecimal** (0.4.8): Precise decimal arithmetic
- **tower** and **tower-http**: Middleware and HTTP utilities
- **async-trait** (0.1.88): Async trait support

---

## 6. Common Development Tasks

### Adding a New Domain Type with ValueObject

1. Create a new file in the appropriate types directory
2. Define your struct wrapping the primitive type
3. Implement `Display` trait
4. Implement `ValueObjectable` trait with validation
5. Optionally implement custom `Deserialize` for `ValueObject<YourType>`
6. Add tests in a `#[cfg(test)]` module
7. Export the type in `mod.rs`

### Adding a New API Endpoint

1. Define DTOs in a `dto.rs` file
2. Define repository trait in `repository.rs` (with `#[cfg_attr(test, automock)]`)
3. Implement repository for database operations
4. Define service layer in `service.rs`
5. Define handler functions in `handler.rs` or `routes.rs`
6. Register routes in the module's router
7. Add tests for each layer

### Debugging Tips

- Use `tracing` for logging (already configured)
- Use `-- --nocapture` with `cargo test` to see println! output
- Use `RUST_BACKTRACE=1` environment variable for stack traces
- Use `cargo check` for faster feedback during development
- Use `cargo clippy` regularly to catch common issues

---

## 7. Database Interaction

### SQLx Query Pattern

Use parameterized queries with SQLx:

```rust
sqlx::query!(
    r#"
    INSERT INTO table_name (column1, column2)
    VALUES ($1, $2)
    "#,
    value1,
    value2
)
.execute(&pool)
.await?;
```

SQLx provides compile-time checking of SQL queries when a database is available.

### Extracting ValueObject Values for Database

When binding ValueObject values to queries:

```rust
.bind(value_object.extract().get_value())
```

---

## 8. Frontend (Brief Overview)

The frontend is a separate React application in the `frontend/` directory:

- **Framework**: React 19.1.0 with TypeScript 5.9.3
- **Build Tool**: Vite 7.1.9
- **Styling**: TailwindCSS 4.1.8
- **State Management**: Redux Toolkit 2.8.2
- **Testing**: Vitest 3.2.3

Run tests: `npx vitest` (from `frontend/` directory)

---

## Summary

This document covers the essential project-specific information for developing the Obvia ERP system. Key takeaways:

1. **Rust Edition 2024** with single package structure
2. **No unsafe code allowed** (enforced via clippy and main.rs)
3. **Tests are co-located** with source code using `#[cfg(test)]` modules
4. **ValueObject pattern is critical** for domain modeling with validation
5. **All commits must be signed off** with DCO (`git commit -s`)
6. **Multi-tenant architecture** with manager and tenant modules
7. Use **pretty_assertions** for better test output
8. Use **mockall** for mocking traits in tests

For more general information, see the main `README.md` and `CONTRIBUTING.md` files.