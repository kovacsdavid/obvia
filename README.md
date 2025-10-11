# Obvia ERP

A modern Enterprise Resource Planning (ERP) system built with Rust and React. Obvia ERP features a multi-tenant
architecture.

## Overview

Obvia ERP is designed as a full-stack web application:

- **Backend**: RESTful API built with Rust using the Axum web framework
- **Frontend**: React-based single-page application (SPA)
- **Architecture**: Multi-tenant system with PostgreSQL databases
- **Authentication**: JWT-based authentication with Argon2 password hashing

## Technology Stack

### Backend

- **Language**: Rust (Edition 2024)
- **Web Framework**: Axum 0.8.4
- **Database**: PostgreSQL with SQLx 0.8.6
- **Authentication**: JWT (jsonwebtoken), Argon2 for password hashing
- **Async Runtime**: Tokio 1.45.1
- **Configuration**: TOML-based configuration files
- **Testing**: pretty_assertions, mockall

### Frontend

- **Framework**: React 19.1.0
- **Language**: TypeScript 5.8.3
- **Build Tool**: Vite 6.3.5
- **Styling**: TailwindCSS 4.1.8
- **State Management**: Redux Toolkit 2.8.2 with redux-persist
- **Routing**: React Router 7.6.1
- **UI Components**: Radix UI components
- **Testing**: Vitest 3.2.3

## Project Structure

```
obvia/
├── backend/                        # Backend Rust workspace
│   ├── bin/                        # Backend binary (entry point)
│   │   └── src/main.rs             # Main application entry point
│   ├── lib/                        # Backend library code
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
│   └── Dockerfile                  # Frontend container definition
├── CONTRIBUTING.md                 # Contribution guidelines
├── CONTRIBUTORS.md                 # List of contributors
├── DCO                             # Developer Certificate of Origin
├── LICENSE                         # GNU AGPL v3 license
└── Jenkinsfile                     # CI/CD pipeline configuration
```

## Requirements

### Backend

- Rust 1.90.0 or later
- PostgreSQL database
- Cargo (included with Rust)

### Frontend

- Node.js 24 or later
- npm (included with Node.js)

### Optional

- Docker (for containerized deployment)
- Docker Compose (for local development with containers)

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
   Edit `config/default.toml` with your database credentials and settings.

4. **Build the backend**:
   ```bash
   cargo build --release
   ```

5. **Run the backend**:
   ```bash
   cargo run --bin backend_bin
   ```
   The backend will start on `http://0.0.0.0:3000` (or the configured host/port).

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

4. **Build for production**:
   ```bash
   npm run build
   ```

## Environment Variables and Configuration

### Backend Configuration (`backend/config/default.toml`)

The backend uses TOML configuration files located in `backend/config/`. Create `default.toml` from the example file:

**Key Configuration Sections:**

- **Server Settings**:
    - `host`: Server host address (default: `0.0.0.0`)
    - `port`: Server port (default: `3000`)

- **Main Database**:
    - `host`: PostgreSQL host
    - `port`: PostgreSQL port (default: `5432`)
    - `username`: Database username
    - `password`: Database password
    - `database`: Database name
    - `pool_size`: Connection pool size (default: `10`)

- **Default Tenant Database**:
    - Same structure as main database
    - Used for tenant-specific data

- **Authentication**:
    - `jwt_secret`: Secret key for JWT token signing
    - `jwt_issuer`: JWT issuer (default: `obvia`)
    - `jwt_audience`: JWT audience (default: `obvia_users`)
    - `jwt_expiration_mins`: Token expiration time in minutes (default: `480`)

### Frontend Environment Variables

The frontend can use environment variables during build:

- `VITE_GIT_COMMIT_HASH`: Git commit hash (set during Docker build)

## Running Tests

### Backend Tests

Run all backend tests:

```bash
cd backend
cargo test
```

Run tests with output:

```bash
cargo test -- --nocapture
```

### Frontend Tests

Run frontend tests:

```bash
cd frontend
npx vitest
```

## Available Scripts

### Backend Scripts

- `cargo build`: Build the backend in debug mode
- `cargo build --release`: Build optimized production binary
- `cargo run`: Run the backend in development mode
- `cargo test`: Run all tests
- `cargo clippy`: Run linter for code quality checks
- `cargo fmt`: Formats rust files

### Frontend Scripts

- `npm run dev`: Start development server with hot reload
- `npm run build`: Build for production (TypeScript compilation + Vite build)
- `npm run preview`: Preview production build locally
- `npm run lint`: Run ESLint for code quality

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
docker build -t obvia-frontend .
```

### Running Containers

**Backend** (exposes port 3000):

```bash
docker run -p 3000:3000 --env-file backend/config/.env obvia-backend
```

**Frontend** (exposes port 80):

```bash
docker run -p 80:80 obvia-frontend
```

## Database Migrations

The application automatically runs database migrations on startup. Migration files are located in:

- `backend/migrations/main/`: Main database schema
- `backend/migrations/tenant/`: Tenant-specific schema

Migrations are executed using SQLx during the application initialization process.

## Contributing

We welcome contributions! Please read [CONTRIBUTING.md](./CONTRIBUTING.md) for details on:

- How to submit feature requests and bug reports
- Code contribution guidelines
- Developer Certificate of Origin (DCO) requirements
- Code review process

**Important**: All commits must be signed off using `git commit -s` to comply with the DCO.

Contributors are listed in [CONTRIBUTORS.md](./CONTRIBUTORS.md).

## License

This project is licensed under the **GNU Affero General Public License v3.0 (AGPL-3.0)**.

See the [LICENSE](./LICENSE) file for the full license text.

### What this means:

- You can use, modify, and distribute this software
- If you modify and deploy this software (including as a web service), you must make your source code available under
  the same license
- Network use counts as distribution under AGPL (unlike GPL)

## Contact

For questions or support, please contact:

- **Email**: kapcsolat@kovacsdavid.dev

---

**Copyright (C) 2025 Kovács Dávid**

This program is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General Public
License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later
version.
