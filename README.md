# first-rust-api

Uma API backend em Rust para aprendizado prático da linguagem.

## 🦀 Stack

- **[axum](https://github.com/tokio-rs/axum)** — Web framework
- **[tokio](https://tokio.rs/)** — Runtime assíncrono
- **[sqlx](https://github.com/launchbadge/sqlx)** — Banco de dados PostgreSQL
- **[jsonwebtoken](https://github.com/Keats/jsonwebtoken)** — Autenticação JWT
- **[argon2](https://github.com/RustCrypto/password-hashes)** — Hash de senhas
- **[serde](https://serde.rs/)** — Serialização JSON

## 📂 Estrutura do projeto

```
src/
├── main.rs              # Ponto de entrada
├── config.rs            # Variáveis de ambiente
├── error.rs             # Erros centralizados
├── state.rs             # Estado compartilhado (AppState)
├── modules/
│   ├── auth/            # Registro e login
│   │   ├── dto.rs       # Request/Response types
│   │   ├── handler.rs   # HTTP handlers
│   │   ├── mod.rs       # Router
│   │   └── service.rs   # Lógica de negócio
│   └── users/           # Consulta de usuários
│       ├── dto.rs
│       ├── handler.rs
│       ├── mod.rs
│       └── service.rs
├── db/
│   ├── mod.rs
│   └── user_repository.rs  # Queries SQL
├── middleware/
│   ├── mod.rs
│   └── auth.rs          # JWT extractor
└── models/
    ├── mod.rs
    └── user.rs          # Struct do banco
```

## 🚀 Como executar

### Pré-requisitos

- Rust (estável)
- PostgreSQL rodando localmente

### Configuração

1. Copie o arquivo de variáveis de ambiente:
   ```bash
   cp .env.example .env
   ```

2. Edite o `.env` com suas configurações:
   ```env
   DATABASE_URL=postgresql://postgres:postgres@localhost:5432/first_rust_api
   JWT_SECRET=seu-segredo-super-secreto
   JWT_EXPIRES_IN=86400
   SERVER_ADDR=0.0.0.0:3000
   ```

3. Crie o banco de dados:
   ```sql
   CREATE DATABASE first_rust_api;
   ```

4. Execute:
   ```bash
   cargo run
   ```

As migrations serão aplicadas automaticamente ao iniciar.

## 📡 Endpoints

### Health Check
```
GET /health
```

### Autenticação

**Registro:**
```
POST /auth/register
Content-Type: application/json

{
  "email": "gabriel@email.com",
  "password": "12345678"
}
```

**Login:**
```
POST /auth/login
Content-Type: application/json

{
  "email": "gabriel@email.com",
  "password": "12345678"
}
```

Resposta:
```json
{
  "access_token": "eyJ...",
  "token_type": "Bearer"
}
```

### Usuários (rotas protegidas)

Todas as rotas abaixo requerem o header:
```
Authorization: Bearer <token>
```

**Dados do usuário autenticado:**
```
GET /users/me
```

**Buscar usuário por ID:**
```
GET /users/:id
```

## 🧪 Testes

```bash
cargo test
```

## 🔒 Segurança

- Senhas com hash Argon2 (nunca em texto puro)
- Segredo JWT via variável de ambiente
- Validação de e-mail e tamanho mínimo de senha
- Rotas privadas protegidas por extractor JWT

## 📚 Fases do projeto

- [x] **Fase 1** — Base: servidor Axum + GET /health
- [x] **Fase 2** — Configuração + banco de dados + migrations
- [x] **Fase 3** — POST /auth/register
- [x] **Fase 4** — POST /auth/login + JWT
- [x] **Fase 5** — Rotas protegidas com extractor JWT
- [x] **Fase 6** — GET /users/me e GET /users/:id
- [x] **Fase 7** — Centralização de erros + organização em módulos
- [ ] **Fase 8** — PUT /users/me, refresh token, testes de integração, Docker
