-- Migration: Criação da tabela users
--
-- Esta tabela armazena os dados dos usuários cadastrados na aplicação.
-- Utilizamos UUID como chave primária para evitar enumeração de IDs.

CREATE TABLE IF NOT EXISTS users (
  -- UUID gerado pela aplicação (uuid v4)
  id UUID PRIMARY KEY,
  -- E-mail único do usuário
  email TEXT NOT NULL UNIQUE,
  -- Hash Argon2 da senha (nunca armazenar a senha em texto puro!)
  password_hash TEXT NOT NULL,
  -- Timestamps automáticos
  created_at TIMESTAMP NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

-- Índice no e-mail para buscas rápidas no login
CREATE INDEX IF NOT EXISTS idx_users_email ON users(email);
