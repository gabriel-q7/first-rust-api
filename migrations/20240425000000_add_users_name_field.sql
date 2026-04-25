-- Migration: Adição do campo name à tabela users
--
-- Adiciona um campo opcional para o nome do usuário.
-- O campo é nullable para manter compatibilidade com registros existentes.

ALTER TABLE users 
ADD COLUMN name VARCHAR(255) NULL;

-- Comentário na coluna para documentação
COMMENT ON COLUMN users.name IS 'Nome opcional do usuário';