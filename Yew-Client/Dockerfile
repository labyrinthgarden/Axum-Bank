FROM rust:1.88-slim AS builder
ENV TRUNK_WASM_BINDGEN_VERSION=0.2.128

RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config \
    libssl-dev \
    curl \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

RUN rustup target add wasm32-unknown-unknown
RUN cargo install trunk --locked

WORKDIR /app
COPY Cargo.toml Trunk.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
#RUN cargo build --target wasm32-unknown-unknown --release

COPY . .
RUN trunk build --release

FROM nginx:1.27-alpine
COPY --from=builder /app/dist /usr/share/nginx/html

RUN cat > /etc/nginx/conf.d/default.conf <<'EOF'
server {
    listen 80;
    server_name localhost;

    root /usr/share/nginx/html;
    index index.html;

    location /api/ {
        resolver 127.0.0.11 valid=10s ipv6=off;
        set $backend_upstream "backend:3000";
        proxy_pass http://backend:3000/api/;
        proxy_http_version 1.1;
        proxy_set_header Host $host;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
    }

    location / {
        try_files $uri $uri/ /index.html;
    }

    location ~* \.wasm$ {
        add_header Content-Type application/wasm;
    }
}
EOF

EXPOSE 80
