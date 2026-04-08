//! config.rs — Carrega e armazena as configurações da aplicação a partir de variáveis de ambiente.
//!
//! Utilizamos `dotenvy` para ler o arquivo `.env` e `std::env::var` para acessar as variáveis.

use std::env;

/// Contém todas as configurações necessárias para rodar a aplicação.
#[derive(Clone, Debug)]
pub struct Config {
    /// URL de conexão com o banco de dados PostgreSQL.
    pub database_url: String,
    /// Segredo usado para assinar e verificar tokens JWT.
    pub jwt_secret: String,
    /// Duração do token JWT em segundos (padrão: 86400 = 24h).
    pub jwt_expires_in: u64,
    /// Endereço e porta onde o servidor vai escutar.
    pub server_addr: String,
}

impl Config {
    /// Lê as variáveis de ambiente e retorna a configuração da aplicação.
    ///
    /// # Panics
    /// Entra em pânico se alguma variável obrigatória não estiver definida.
    pub fn from_env() -> Self {
        // Tenta carregar o arquivo `.env` (ignora erro se não existir).
        dotenvy::dotenv().ok();

        Self {
            database_url: env::var("DATABASE_URL")
                .expect("DATABASE_URL deve estar definida no .env"),
            jwt_secret: env::var("JWT_SECRET")
                .expect("JWT_SECRET deve estar definida no .env"),
            jwt_expires_in: env::var("JWT_EXPIRES_IN")
                .unwrap_or_else(|_| "86400".to_string())
                .parse()
                .expect("JWT_EXPIRES_IN deve ser um número"),
            server_addr: env::var("SERVER_ADDR")
                .unwrap_or_else(|_| "0.0.0.0:3000".to_string()),
        }
    }
}
