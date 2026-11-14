# Axum Bank
Banking infrastructure server built with Rust, Axum and PostgreSQL, following hexagonal architecture and repository patterns.

![Yew](https://img.shields.io/badge/Yew-4BB8A5?style=for-the-badge&logo=rust&logoColor=white)
![Iced](https://img.shields.io/badge/Iced-3645FF?style=for-the-badge&logo=rust&logoColor=white)
![Axum](https://img.shields.io/badge/Axum-000000?style=for-the-badge&logo=rust&logoColor=white)
![Rust](https://img.shields.io/badge/Rust-CE422B?style=for-the-badge&logo=rust&logoColor=white)
![PostgreSQL](https://img.shields.io/badge/PostgreSQL-4169E1?style=for-the-badge&logo=postgresql&logoColor=white)
![Docker](https://img.shields.io/badge/Docker-2496ED?style=for-the-badge&logo=docker&logoColor=white)

# Previews

## Architecture

- **Frontend:** Yew (WASM) `axum-bank-client/` → Yew client application
- **Backend:** Axum + SQLx + PostgreSQL `axum-bank-server/` → Axum backend API service
- **Infra:** Docker + Docker Compose `docker-compose.yml` → local orchestration (frontend + backend + db)

## Run with Docker (recommended)

```bash
cp .env.example .env
docker compose up --build
```

Services:

- Frontend: `http://localhost:8080`
- Backend API: `http://localhost:3000`
- PostgreSQL: `localhost:5432`

## API endpoints

- `GET /api/health`
