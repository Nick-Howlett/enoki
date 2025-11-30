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
- **Tower HTTP** - CORS middleware
- **Serde** - Serialization/deserialization

## Getting Started

### Prerequisites
- Node.js (v18 or higher)
- Rust (latest stable)
- npm or yarn

### Installation

1. Clone the repository
```bash
git clone <repository-url>
cd enoki
```

2. Install frontend dependencies
```bash
cd frontend
npm install
```

3. Build backend dependencies (first run will download and compile)
```bash
cd ../backend
cargo build
```

## Development

### Running the Frontend
```bash
cd frontend
npm run dev
```
The frontend will be available at http://localhost:3000

### Running the Backend
```bash
cd backend
cargo run
```
The backend API will be available at http://localhost:8080

### Frontend Commands
- `npm run dev` - Start development server on port 3000
- `npm run build` - Build for production
- `npm run preview` - Preview production build
- `npm run lint` - Run ESLint
- `npm run format` - Format code with Prettier
- `npm run format:check` - Check code formatting

### Backend Commands
- `cargo run` - Run the server on port 8080
- `cargo build` - Build the project
- `cargo build --release` - Build optimized release version
- `cargo test` - Run tests

## API Endpoints

- `GET /api/health` - Health check endpoint

## Project Structure

```
enoki/
├── frontend/           # React frontend application
│   ├── src/
│   ├── public/
│   └── package.json
├── backend/            # Rust backend application
│   ├── src/
│   └── Cargo.toml
└── README.md
```

## Configuration

### Frontend
- Vite is configured to proxy `/api` requests to `http://localhost:8080`
- Port: 3000 (configured in `vite.config.ts`)
- Prettier settings in `.prettierrc`

### Backend
- Port: 8080 (configured in `src/main.rs`)
- CORS is enabled for all origins in development
