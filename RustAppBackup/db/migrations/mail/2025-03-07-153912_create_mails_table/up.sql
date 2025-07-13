-- Your SQL goes here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE IF NOT EXISTS mail_histories (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    project_id UUID NOT NULL,
    project_status_id UUID NOT NULL,
    sender_id UUID NOT NULL,
    "to" VARCHAR(150) NOT NULL,
    cc VARCHAR(255) NULL,
    body VARCHAR(255) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    created_by UUID NOT NULL,
    updated_at TIMESTAMP,
    updated_by UUID NULL,
    deleted_at TIMESTAMP,
    deleted_by UUID NULL
);