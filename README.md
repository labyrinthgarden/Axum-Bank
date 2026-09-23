# Axum Bank
A simple but professional full-stack Rust banking system:

- **Backend:** Axum + SQLx + PostgreSQL
- **Frontend:** Yew (WASM)
- **Infra:** Docker + Docker Compose

#
![Rust](https://img.shields.io/badge/Rust-CE422B?style=for-the-badge&logo=rust&logoColor=white)
![Axum](https://img.shields.io/badge/Axum-000000?style=for-the-badge&logo=rust&logoColor=white)
![Yew](https://img.shields.io/badge/Yew-4BB8A5?style=for-the-badge&logo=rust&logoColor=white)
![Docker](https://img.shields.io/badge/Docker-2496ED?style=for-the-badge&logo=docker&logoColor=white)
![PostgreSQL](https://img.shields.io/badge/PostgreSQL-4169E1?style=for-the-badge&logo=postgresql&logoColor=white)

## Architecture

- `Axum-Bank/` → backend API service
- `Yew-Client/` → Yew client application
- `docker-compose.yml` → local orchestration (db + backend + frontend)

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
- `GET /api/accounts`
- `POST /api/accounts`
- `POST /api/accounts/{id}/deposit`
- `POST /api/accounts/{id}/withdraw`
- `GET /api/accounts/{id}/transactions`
- `POST /api/transfers`
