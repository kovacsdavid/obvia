# Obvia ERP

A modern Enterprise Resource Planning (ERP) system built with Rust and React. Obvia ERP features a multi-tenant
architecture designed for scalability and security.

## Overview

Obvia ERP is designed as a full-stack web application:

- **Backend**: RESTful API built with Rust using the Axum web framework
- **Frontend**: React-based single-page application (SPA) with TypeScript
- **Architecture**: Multi-tenant system with PostgreSQL databases
- **Authentication**: JWT-based authentication with Argon2 password hashing
- **Security**: No unsafe code allowed (enforced at compile time)

## Technology Stack

### Backend

- **Language**: Rust (Edition 2024)
- **Web Framework**: Axum 0.8.4
- **Database**: PostgreSQL with SQLx 0.8.6
- **Authentication**: JWT (jsonwebtoken 10.0.0), Argon2 0.5.3 for password hashing
- **Async Runtime**: Tokio 1.45.1
- **Configuration**: TOML-based configuration files (config crate 0.15.11)
- **Error Handling**: thiserror 2.0.12, anyhow 1.0.98
- **Testing**: pretty_assertions 1.4.1, mockall 0.13.1

### Frontend

- **Framework**: React 19.1.0
- **Language**: TypeScript 5.9.3
- **Build Tool**: Vite 7.1.9
- **Styling**: TailwindCSS 4.1.8
- **State Management**: Redux Toolkit 2.8.2
- **Routing**: React Router 7.6.1
- **UI Components**: Radix UI components
- **Icons**: Lucide React 0.545.0
- **Testing**: Vitest 3.2.3
- **Linting**: ESLint 9.25.0 with TypeScript ESLint 8.30.1

## Project Structure

```
obvia/
├── backend/                        # Backend Rust workspace
│   ├── bin/                        # Backend binary (entry point)
│   │   ├── src/main.rs             # Main application entry point
│   │   └── Cargo.toml              # Binary package manifest
│   ├── lib/                        # Backend library code
│   │   ├── src/
│   │   │   ├── common/             # Common utilities and types
│   │   │   ├── manager/            # System-wide operations (auth, users, tenants)
│   │   │   ├── tenant/             # Tenant-specific business logic
│   │   │   └── lib.rs              # Library entry point
│   │   └── Cargo.toml              # Library package manifest
│   ├── config/                     # Configuration files
│   │   └── default.toml.example    # Example configuration
│   ├── migrations/                 # Database migrations
│   │   ├── main/                   # Main database migrations
│   │   └── tenant/                 # Tenant database migrations
│   ├── Cargo.toml                  # Rust workspace manifest
│   └── Dockerfile                  # Backend container definition
├── frontend/                       # Frontend React application
│   ├── src/                        # Frontend source code
│   ├── public/                     # Static assets
│   ├── package.json                # Node.js dependencies and scripts
│   ├── vite.config.ts              # Vite build configuration
│   ├── tsconfig.json               # TypeScript configuration
│   ├── eslint.config.js            # ESLint configuration
│   ├── httpd.conf                  # Apache HTTP server configuration
│   └── Dockerfile                  # Frontend container definition
├── CONTRIBUTING.md                 # Contribution guidelines
├── CONTRIBUTORS.md                 # List of contributors
├── DCO                             # Developer Certificate of Origin
├── LICENSE                         # GNU AGPL v3 license
├── Jenkinsfile                     # CI/CD pipeline configuration
└── README.md                       # This file
```

## Requirements

### Backend

- **Rust**: 1.90.0 or later
- **PostgreSQL**: Database server (version 12 or later recommended)
- **Cargo**: Package manager (included with Rust)

### Frontend

- **Node.js**: 24 or later
- **npm**: Package manager (included with Node.js)

### Optional

- **Docker**: For containerized deployment
- **Docker Compose**: For local development with containers

## Setup and Installation

### Backend Setup

1. **Install Rust** (if not already installed):
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **Navigate to the backend directory**:
   ```bash
   cd backend
   ```

3. **Configure the application**:
   ```bash
   cp config/default.toml.example config/default.toml
   ```
   Edit `config/default.toml` with your database credentials and settings. See
   the [Configuration](#backend-configuration-backendconfigdefaulttoml) section for details.

4. **Build the backend**:
   ```bash
   # Development build
   cargo build
   
   # Production build (optimized)
   cargo build --release
   ```

5. **Run the backend**:
   ```bash
   # Development mode
   cargo run --bin obvia_backend
   
   # Production mode (after release build)
   ./target/release/obvia_backend
   ```

   The backend will start on `http://0.0.0.0:3000` (or the configured host/port).
   Database migrations will be automatically executed on startup.

### Frontend Setup

1. **Navigate to the frontend directory**:
   ```bash
   cd frontend
   ```

2. **Install dependencies**:
   ```bash
   npm install
   ```

3. **Run the development server**:
   ```bash
   npm run dev
   ```

   The frontend will be available at `http://localhost:5173` (default Vite port).

   **Note**: The development server is configured to proxy API requests from `/api` to `http://localhost:3000` (the
   backend server). Ensure the backend is running before making API calls.

4. **Build for production**:
   ```bash
   npm run build
   ```

   The production build will be output to the `dist/` directory.

## Environment Variables and Configuration

### Backend Configuration (`backend/config/default.toml`)

The backend uses TOML configuration files located in `backend/config/`. Create `default.toml` from the example file and
customize it for your environment.

**Key Configuration Sections:**

#### Server Settings

```toml
[server]
host = "0.0.0.0"        # Server host address
port = 3000             # Server port
```

#### Main Database

```toml
[main_database]
host = "localhost"      # PostgreSQL host
port = 5432             # PostgreSQL port
username = "db_user"    # Database username
password = "db_pass"    # Database password
database = "obvia_main" # Main database name
pool_size = 10          # Connection pool size
```

#### Default Tenant Database

```toml
[default_tenant_database]
host = "localhost"          # PostgreSQL host
port = 5432                 # PostgreSQL port
username = "db_user"        # Database username
password = "db_pass"        # Database password
database = "obvia_tenant"   # Tenant database name
pool_size = 10              # Connection pool size
```

#### Authentication

```toml
[auth]
jwt_secret = "your_super_secret_jwt_key"    # Secret key for JWT signing (change this!)
jwt_issuer = "obvia"                        # JWT issuer
jwt_audience = "obvia_users"                # JWT audience
jwt_expiration_mins = "480"                 # Token expiration (8 hours)
```

**Important**: Always change the `jwt_secret` to a strong, random value in production environments.

### Frontend Environment Variables

The frontend can use environment variables during the build process:

- `VITE_GIT_COMMIT_HASH`: Git commit hash (automatically set during Docker build)

Additional environment variables can be added following
the [Vite environment variable conventions](https://vitejs.dev/guide/env-and-mode.html) (prefix with `VITE_`).

### Development Proxy Configuration

The frontend development server (`npm run dev`) is configured to proxy API requests:

- All requests to `/api/*` are forwarded to `http://localhost:3000`
- This is configured in `frontend/vite.config.ts`

## Running Tests

### Backend Tests

The backend uses inline test modules (co-located with source code) and includes unit tests with mocking support.

Run all backend tests:
```bash
cd backend
cargo test
```

Run tests with output (shows `println!` statements):
```bash
cargo test -- --nocapture
```

Run specific test module or function:

```bash
cargo test <module_or_function_name>
```

Run tests with verbose output:

```bash
cargo test --verbose
```

### Frontend Tests

Run tests in watch mode (default for Vitest):
```bash
cd frontend
npx vitest
```

Run tests once and exit:

```bash
cd frontend
npx vitest run
```

## Available Scripts

### Backend Scripts

Run these commands from the `backend/` directory:

- **`cargo build`**: Build the backend in debug mode
- **`cargo build --release`**: Build optimized production binary
- **`cargo run --bin obvia_backend`**: Run the backend in development mode
- **`cargo test`**: Run all tests
- **`cargo clippy`**: Run linter for code quality checks
- **`cargo fmt`**: Format Rust source files
- **`cargo check`**: Check for compilation errors without building

### Frontend Scripts

Run these commands from the `frontend/` directory:

- **`npm run dev`**: Start development server with hot reload (port 5173)
- **`npm run build`**: Build for production (TypeScript compilation + Vite build)
- **`npm run preview`**: Preview production build locally
- **`npm run lint`**: Run ESLint for code quality
- **`npx vitest`**: Run tests with Vitest

## Docker Deployment

### Building Docker Images

**Backend:**
```bash
cd backend
docker build -t obvia-backend .
```

**Frontend:**
```bash
cd frontend
docker build --build-arg VITE_GIT_COMMIT_HASH=$(git rev-parse --short HEAD) -t obvia-frontend .
```

### Running Containers

**Backend** (exposes port 3000):
```bash
docker run -v /path/to/config:/opt/config -p 3000:3000 obvia-backend 
```

**Frontend** (exposes port 80):

```bash
docker run -p 80:80 obvia-frontend
```

The frontend container uses Apache HTTP Server (httpd 2.4) to serve the static build.

### Docker Platform Support

Both Dockerfiles support multi-platform builds:

- **Backend**: Supports `linux/amd64` and `linux/arm64` with cross-compilation

## Database Migrations

The application automatically runs database migrations on startup using SQLx. Migration files are located in:

- **`backend/migrations/main/`**: Main database schema (system-wide data)
- **`backend/migrations/tenant/`**: Tenant-specific schema

Migrations are executed in order based on their timestamp prefix. The migration system tracks which migrations have been
applied to avoid re-running them.

## Architecture

### Multi-Tenant Architecture

Obvia ERP implements a multi-tenant architecture with separate database schemas:

- **Manager Module** (`backend/lib/src/manager/`): Handles system-wide operations including:
    - Authentication and authorization
    - User management
    - Tenant management

- **Tenant Module** (`backend/lib/src/tenant/`): Contains tenant-specific business logic:
    - Products
    - Services
    - Customers
    - Taxes
    - And other tenant-isolated data

### Security

- **No Unsafe Code**: The workspace is configured to deny unsafe code via Clippy metadata and `#![forbid(unsafe_code)]`
- **Password Hashing**: Uses Argon2 for secure password hashing
- **JWT Authentication**: Stateless authentication with configurable expiration
- **SQL Injection Protection**: All database queries use parameterized queries via SQLx

## Contributing

We welcome contributions! Please read [CONTRIBUTING.md](./CONTRIBUTING.md) for details on:

- How to submit feature requests and bug reports
- Code contribution guidelines
- Developer Certificate of Origin (DCO) requirements
- Code review process
- Testing requirements

**Important**: All commits must be signed off using `git commit -s` to comply with the DCO.

Contributors are listed in [CONTRIBUTORS.md](./CONTRIBUTORS.md).

## License

This project is licensed under the **GNU Affero General Public License v3.0 (AGPL-3.0)**.

See the [LICENSE](./LICENSE) file for the full license text.

### What this means:

- ✅ You can use, modify, and distribute this software
- ✅ You can use it for commercial purposes
- ⚠️ If you modify and deploy this software (including as a web service), you **must** make your source code available
  under the same license
- ⚠️ Network use counts as distribution under AGPL (unlike GPL)
- ⚠️ You must include the original copyright notice and license text

## Contact

For questions or support, please contact:

- **Email**: kapcsolat@kovacsdavid.dev

---

**Copyright (C) 2025 Kovács Dávid**

This program is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General Public
License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later
version.

This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied
warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU Affero General Public License for more
details.

You should have received a copy of the GNU Affero General Public License along with this program. If not,
see <https://www.gnu.org/licenses/>.