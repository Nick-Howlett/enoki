# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Enoki is a full-stack web application with a React frontend and Rust backend. The frontend runs on port 3000 and proxies API requests to the backend on port 8080.

## Development Commands

### Frontend (in `frontend/` directory)
- `npm run dev` - Start Vite dev server on port 3000
- `npm run build` - Build for production (runs TypeScript compiler then Vite build)
- `npm run format` - Format code with Prettier
- `npm run format:check` - Check formatting without making changes
- `npm run lint` - Run ESLint

### Backend (in `backend/` directory)
- `cargo run` - Run development server on port 8080
- `cargo build` - Compile the project
- `cargo check` - Type-check without building binaries (faster)
- `cargo test` - Run tests
- `cargo build --release` - Build optimized production binary

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
- **Tower HTTP** for CORS middleware
- **Serde** for JSON serialization/deserialization

### Backend Structure
- Entry point: `src/main.rs`
- Server runs on `127.0.0.1:8080`
- CORS is configured to allow all origins, methods, and headers (development mode)
- Routes are defined in the main Router
- Current endpoints:
  - `GET /api/health` - Returns `{"status": "ok"}`

## Key Conventions

### Frontend
- Use Prettier formatting rules defined in `.prettierrc` (single quotes, 2-space tabs, semicolons)
- All new components should use Material UI components where applicable
- Use Tanstack Query for all data fetching and server state
- TypeScript strict mode is enabled

### Backend
- All API routes should be prefixed with `/api`
- Use Axum's `Json` extractor for request/response bodies
- Derive `Serialize` and `Deserialize` on all API types
- Follow Rust naming conventions (snake_case for functions/variables)

## Adding New Features

### New API Endpoint
1. Define request/response structs with `#[derive(Serialize, Deserialize)]` in `backend/src/main.rs`
2. Create handler function (async, returns `Json<T>`)
3. Add route to the Router in `main()`
4. CORS is already configured globally

### New Frontend Page/Component
1. Create component in `frontend/src/`
2. Import Material UI components from `@mui/material`
3. Use Tanstack Query hooks (`useQuery`, `useMutation`) for API calls
4. Format with `npm run format` before committing

## Notes
- The frontend Vite proxy automatically forwards `/api` requests to the backend, so frontend code should use relative URLs like `/api/health`
- First-time backend compilation will take several minutes as it downloads and compiles all dependencies
