-- Your SQL goes here
CREATE TABLE IF NOT EXISTS employee_projects (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    employee_id UUID REFERENCES employees(id),
    project_id UUID NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    created_by UUID NOT NULL,
    updated_at TIMESTAMP,
    updated_by UUID NULL,
    deleted_at TIMESTAMP,
    deleted_by UUID NULL
);