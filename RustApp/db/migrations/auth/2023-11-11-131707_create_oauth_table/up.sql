-- Your SQL goes here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE IF NOT EXISTS users (
    user_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    username VARCHAR(50) NOT NULL,
    email VARCHAR(255) NOT NULL,
    password_hash VARCHAR(60) NOT NULL,
    full_name VARCHAR(100) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    created_by UUID REFERENCES users(user_id),
    updated_at TIMESTAMP,
    updated_by UUID REFERENCES users(user_id),
    deleted_at TIMESTAMP,
    deleted_by UUID REFERENCES users(user_id)
);

CREATE TABLE IF NOT EXISTS clients (
    client_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    client_secret VARCHAR(255) NOT NULL,
    client_name VARCHAR(255) NOT NULL,
    redirect_uri VARCHAR(255) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    created_by UUID REFERENCES users(user_id),
    updated_at TIMESTAMP,
    updated_by UUID REFERENCES users(user_id),
    deleted_at TIMESTAMP,
    deleted_by UUID REFERENCES users(user_id)
);

CREATE TABLE IF NOT EXISTS access_tokens (
    token_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    client_id UUID NOT NULL REFERENCES clients(client_id),
    user_id UUID NOT NULL REFERENCES users(user_id),
    token_value VARCHAR(255) NOT NULL,
    scope TEXT NOT NULL,
    expires_at TIMESTAMP NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    created_by UUID REFERENCES users(user_id),
    updated_at TIMESTAMP,
    updated_by UUID REFERENCES users(user_id),
    deleted_at TIMESTAMP,
    deleted_by UUID REFERENCES users(user_id)
);

CREATE TABLE IF NOT EXISTS authorization_codes (
    code_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    client_id UUID NOT NULL REFERENCES clients(client_id),
    user_id UUID NOT NULL REFERENCES users(user_id),
    code_value VARCHAR(255) NOT NULL,
    redirect_uri VARCHAR(255) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    created_by UUID REFERENCES users(user_id),
    updated_at TIMESTAMP,
    updated_by UUID REFERENCES users(user_id),
    deleted_at TIMESTAMP,
    deleted_by UUID REFERENCES users(user_id)
);

CREATE TABLE IF NOT EXISTS refresh_tokens (
    refresh_token_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    client_id UUID NOT NULL REFERENCES clients(client_id),
    user_id UUID NOT NULL REFERENCES users(user_id),
    refresh_token_value VARCHAR(255) NOT NULL,
    expires_at TIMESTAMP NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    created_by UUID REFERENCES users(user_id),
    updated_at TIMESTAMP,
    updated_by UUID REFERENCES users(user_id),
    deleted_at TIMESTAMP,
    deleted_by UUID REFERENCES users(user_id)
);