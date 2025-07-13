-- Your SQL goes here
CREATE TABLE IF NOT EXISTS project_items (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    project_id UUID REFERENCES projects(id),
    name VARCHAR(255) NOT NULL,
    timeline_start DATE NOT NULL,
    timeline_end DATE NOT NULL,
    duration INT NOT NULL,
    project_status_id UUID REFERENCES project_statuses(id),
    progress VARCHAR(255) NOT NULL,
    bundling_no INT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    created_by UUID NOT NULL,
    updated_at TIMESTAMP,
    updated_by UUID NULL,
    deleted_at TIMESTAMP,
    deleted_by UUID NULL
);