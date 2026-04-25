-- Initialize test database for development environment
-- This script runs automatically when the PostgreSQL container starts

-- Create test database for integration tests
CREATE DATABASE test_first_rust_api;

-- Grant permissions to postgres user
GRANT ALL PRIVILEGES ON DATABASE test_first_rust_api TO postgres;