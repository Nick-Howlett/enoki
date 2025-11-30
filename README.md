# Enoki

A web application built with React and Rust.

## Tech Stack

### Frontend
- **React** - UI library
- **Vite** - Build tool and dev server
- **TypeScript** - Type safety
- **Material UI** - Component library
- **Tanstack Query** - Data fetching and caching
- **Prettier** - Code formatting

### Backend
- **Rust** - Systems programming language
- **Axum** - Web framework
- **Tokio** - Async runtime
- **SQLx** - Async Postgres database driver with compile-time query checking
- **PostgreSQL** - Database
- **Tower HTTP** - CORS middleware
- **Serde** - JSON serialization/deserialization
- **Tracing** - Structured logging
- **Dotenvy** - Environment variable management

## Getting Started

### Prerequisites
- Node.js (v18 or higher)
- Rust (latest stable)
- npm or yarn
- Docker and Docker Compose (for local Postgres database)
- cargo-watch (for auto-reloading backend during development)
- sqlx-cli (for database migrations)

### Installation

1. Clone the repository
```bash
git clone <repository-url>
cd enoki
```

2. Install Rust development tools
```bash
cargo install cargo-watch
cargo install sqlx-cli --no-default-features --features postgres
```

3. Install frontend dependencies
```bash
cd frontend
npm install
cd ..
```

4. Start the PostgreSQL database
```bash
docker compose up -d
```

5. Set up environment variables (backend)
```bash
cd backend
cp .env.example .env
# Edit .env if needed (defaults should work with Docker setup)
```

6. Run database migrations
```bash
cd backend
sqlx migrate run
cd ..
```

## Development

### Running the Application

You'll need two terminal windows:

**Terminal 1 - Backend:**
```bash
cd backend
cargo watch -x run
```
The backend API will be available at http://localhost:8080

**Terminal 2 - Frontend:**
```bash
cd frontend
npm run dev
```
The frontend will be available at http://localhost:3000

### Without Auto-Reload

If you don't need auto-reload:

**Backend:**
```bash
cd backend
cargo run
```

**Frontend:**
```bash
cd frontend
npm run dev
# (Vite always includes HMR)
```

### Frontend Commands
In `frontend/` directory:
- `npm run dev` - Start development server on port 3000
- `npm run build` - Build for production
- `npm run preview` - Preview production build
- `npm run lint` - Run ESLint
- `npm run format` - Format code with Prettier
- `npm run format:check` - Check code formatting

### Backend Commands
In `backend/` directory:
- `cargo run` - Run the server on port 8080
- `cargo watch -x run` - Run with auto-reload
- `cargo build` - Build the project
- `cargo build --release` - Build optimized release version
- `cargo test` - Run tests
- `cargo fmt` - Format code
- `cargo clippy` - Run linter

### Database Commands
In `backend/` directory:
- `sqlx migrate run` - Run pending migrations
- `sqlx migrate revert` - Rollback the last migration
- `sqlx migrate add <name>` - Create a new migration

### Docker Commands
- `docker compose up -d` - Start Postgres in background
- `docker compose down` - Stop Postgres
- `docker compose logs -f postgres` - View Postgres logs
- `docker compose ps` - Check container status

## API Endpoints

- `GET /api/health` - Health check endpoint (includes database status)
- `GET /api/users` - List all users
- `POST /api/users` - Create a new user
- `GET /api/users/:id` - Get a specific user by ID

## Project Structure

```
enoki/
├── frontend/              # React frontend application
│   ├── src/
│   ├── public/
│   └── package.json
├── backend/               # Rust backend application
│   ├── src/
│   │   ├── main.rs       # Application entry point
│   │   ├── db.rs         # Database connection setup
│   │   ├── models/       # Database models
│   │   └── handlers/     # API request handlers
│   ├── migrations/        # SQL migration files
│   ├── .env.example      # Environment variables template
│   └── Cargo.toml
├── docker-compose.yml     # PostgreSQL container setup
└── README.md
```

## Configuration

### Frontend
- Vite is configured to proxy `/api` requests to `http://localhost:8080`
- Port: 3000 (configured in `vite.config.ts`)
- Prettier settings in `.prettierrc`

### Backend
- Port: 8080 (configured in `.env`)
- Database: PostgreSQL running on port 5432 (via Docker)
- CORS: Configured via `ALLOWED_ORIGIN` environment variable
  - Development default: `http://localhost:3000`
  - Production: Set to your frontend domain (e.g., `https://yourdomain.com`)
  - Allowed methods: GET, POST, PUT, DELETE, OPTIONS
  - Allowed headers: Content-Type, Authorization
- Structured logging with tracing (configured via `RUST_LOG` environment variable)
- Database migrations must be run manually before starting the server

### Database
- PostgreSQL 16 running in Docker container
- Default credentials: `enoki` / `development` (development only!)
- Default database: `enoki_dev`
- Connection string in `backend/.env`
