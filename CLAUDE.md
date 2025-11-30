# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Enoki is a full-stack web application with a React frontend, Rust backend, and PostgreSQL database. The frontend runs on port 3000 and proxies API requests to the backend on port 8080. PostgreSQL runs in a Docker container on port 5432.

## Development Commands

### Frontend (in `frontend/` directory)
- `npm run dev` - Start Vite dev server on port 3000
- `npm run build` - Build for production (runs TypeScript compiler then Vite build)
- `npm run format` - Format code with Prettier
- `npm run format:check` - Check formatting without making changes
- `npm run lint` - Run ESLint

### Backend (in `backend/` directory)
- `cargo watch -x run` - Run with auto-reload on file changes
- `cargo run` - Run development server on port 8080 (no auto-reload)
- `cargo build` - Compile the project
- `cargo check` - Type-check without building binaries (faster)
- `cargo test` - Run tests
- `cargo fmt` - Format code with rustfmt
- `cargo clippy` - Run Clippy linter
- `cargo build --release` - Build optimized production binary

### Database (in `backend/` directory)
- `sqlx migrate run` - Run pending migrations
- `sqlx migrate revert` - Rollback the last migration
- `sqlx migrate add <name>` - Create a new migration file

### Docker (from project root)
- `docker compose up -d` - Start PostgreSQL container
- `docker compose down` - Stop PostgreSQL container
- `docker compose logs -f postgres` - View PostgreSQL logs

## Architecture

### Frontend Stack
- **React 19** with TypeScript
- **Vite** as build tool and dev server
- **Material UI (v7)** for components, using `@emotion` for styling
- **Tanstack Query (v5)** for data fetching, caching, and state management
- **Prettier** for code formatting

### Frontend Structure
- All API calls are proxied through Vite dev server (`/api` → `http://localhost:8080`)
- `App.tsx` sets up providers: `QueryClientProvider` wraps `ThemeProvider` which wraps the app
- MUI theme is configured with light mode and custom primary/secondary colors
- React Query DevTools are included (hidden by default, toggle in dev)

### Backend Stack
- **Axum 0.8** web framework
- **Tokio** async runtime with full features
- **SQLx 0.8** - Async PostgreSQL driver with compile-time query verification
- **PostgreSQL 16** - Database (runs in Docker)
- **Tower HTTP** for CORS middleware
- **Serde** for JSON serialization/deserialization
- **Tracing** for structured logging
- **Dotenvy** for .env file loading
- **Anyhow** for error handling
- **UUID** and **Chrono** for common data types

### Backend Structure
- Entry point: `src/main.rs` - Orchestrates application startup (minimal, delegates to modules)
- Telemetry: `src/telemetry.rs` - Tracing/logging initialization
- Database: `src/db.rs` - Connection pool setup and database configuration
- Routes: `src/routes.rs` - Router creation, CORS configuration, health check endpoint
- Models: `src/models/` - Database models with SQLx queries (e.g., `models/user.rs`)
- Handlers: `src/handlers/` - API request handlers (e.g., `handlers/user.rs`)
- Migrations: `migrations/` - SQL migration files (timestamped)
- Configuration: `.env` file for environment variables (DATABASE_URL, RUST_LOG, ALLOWED_ORIGIN, etc.)
- Server runs on `127.0.0.1:8080`
- CORS is configured via `ALLOWED_ORIGIN` environment variable (defaults to http://localhost:3000)
  - Allowed methods: GET, POST, PUT, DELETE, OPTIONS
  - Allowed headers: Content-Type, Authorization
  - Credentials: enabled
- Database connection pool is shared via Axum state (`AppState`)
- Structured logging via tracing (configured with RUST_LOG env var)

### Current API Endpoints
- `GET /api/health` - Health check with database status
- `GET /api/users` - List all users
- `POST /api/users` - Create new user (body: `{"email": "...", "name": "..."}`)
- `GET /api/users/:id` - Get user by UUID

## Key Conventions

### Frontend
- Use Prettier formatting rules defined in `.prettierrc` (single quotes, 2-space tabs, semicolons)
- All new components should use Material UI components where applicable
- Use Tanstack Query for all data fetching and server state
- TypeScript strict mode is enabled

### Backend

**Module Organization (Rust 2018+ Edition):**
- Use file-based modules, NOT `mod.rs` files
- For submodules: create `src/models.rs` (declares modules) and `src/models/user.rs` (implementation)
- Example structure:
  ```
  src/
    models.rs       ← pub mod user;
    models/
      user.rs       ← implementation
  ```

**Code Style:**
- All API routes should be prefixed with `/api`
- Use Axum's `Json` extractor for request/response bodies
- Use Axum's `State` extractor to access database pool
- Derive `Serialize` and `Deserialize` on all API types
- Derive `FromRow` on database models (from SQLx)
- Use `sqlx::query_as!` or `sqlx::query_as` for type-safe database queries
- Follow Rust naming conventions (snake_case for functions/variables)
- Use `tracing::info!`, `tracing::error!`, etc. for logging (not `println!`)
- Handle errors properly - return appropriate HTTP status codes

**Security:**
- CORS is environment-based, not permissive by default
- Set `ALLOWED_ORIGIN` in `.env` to control which origins can access the API
- For production, set `ALLOWED_ORIGIN=https://yourdomain.com`
- Never use `Any` for CORS settings in production

**Comments:**
- Keep comments to a minimum
- Only add comments for non-standard usage or technically complex code
- Do NOT add obvious comments like `// Create app state` before `let state = AppState { db: db_pool };`
- If code seems to need a comment, consider refactoring into a well-named helper function instead
- Self-documenting code with clear names is preferred over commented code

## Adding New Features

### New Database Table
1. Create migration in `backend/` directory: `sqlx migrate add create_table_name`
2. Edit the generated SQL file in `backend/migrations/`
3. Run migration: `sqlx migrate run` (in `backend/` directory)
4. Create model in `backend/src/models/table_name.rs`
   - Derive `FromRow`, `Serialize`, `Deserialize` on the main struct
   - Add query methods as `impl` block on the model
5. Add module declaration to `backend/src/models.rs`: `pub mod table_name;`

### New API Endpoint
1. Create model in `src/models/` if it doesn't exist
2. Create handler in `src/handlers/` (use `State<AppState>` to access database)
3. Define request/response structs with `#[derive(Serialize, Deserialize)]`
4. Add route to the Router in `main()`
5. CORS is already configured globally

### New Frontend Page/Component
1. Create component in `frontend/src/`
2. Import Material UI components from `@mui/material`
3. Use Tanstack Query hooks (`useQuery`, `useMutation`) for API calls
4. Format with `npm run format` before committing

## Development Workflow

### Initial Setup
1. Start PostgreSQL: `docker compose up -d`
2. Copy environment file: `cd backend && cp .env.example .env`
3. Install tools: `cargo install cargo-watch sqlx-cli --no-default-features --features postgres`
4. Run database migrations: `cd backend && sqlx migrate run`

### Daily Development
**You'll need two terminal windows:**

Terminal 1 - Backend (in `backend/` directory):
```bash
cargo watch -x run
```

Terminal 2 - Frontend (in `frontend/` directory):
```bash
npm run dev
```

**Notes:**
- The frontend Vite proxy automatically forwards `/api` requests to the backend, so frontend code should use relative URLs like `/api/health`
- Database must be running (start with `docker compose up -d` if not already running)
- Both servers auto-reload on file changes (Vite HMR for frontend, cargo-watch recompilation for backend)

### Database Development
- Create migrations for all schema changes (don't modify the database directly)
- Migration files are timestamped and run in order
- Always run migrations manually: `sqlx migrate run` in `backend/` directory
- SQLx performs compile-time verification of SQL queries against the database schema
- If you get SQLx compilation errors after schema changes, ensure migrations have run
- Use `tracing::info!` to log SQL queries during development

### Important Notes
- First-time backend compilation will take several minutes as it downloads and compiles all dependencies
- Environment variables are loaded from `backend/.env`
- PostgreSQL data persists in a Docker volume (survives container restarts)
- To reset database: `docker compose down -v` (warning: deletes all data)
