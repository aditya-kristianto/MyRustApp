-- Your SQL goes here
CREATE TABLE IF NOT EXISTS project_requests (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    requestor_name VARCHAR(255) NOT NULL,
    requestor_email VARCHAR(255) NOT NULL,
    requestor_directorate VARCHAR(255) NOT NULL,
    project_category VARCHAR(255) NOT NULL,
    project_title VARCHAR(255) NOT NULL,
    project_background VARCHAR(255) NOT NULL,
    brd_link VARCHAR(255) NOT NULL,
    live_date_estimation DATE NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    created_by UUID NOT NULL,
    updated_at TIMESTAMP,
    updated_by UUID NULL,
    deleted_at TIMESTAMP,
    deleted_by UUID NULL
);