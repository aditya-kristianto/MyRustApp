-- Your SQL goes here
CREATE TABLE IF NOT EXISTS project_ticket_approvals (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    project_id UUID REFERENCES projects(id),
    ticket_id UUID NOT NULL,
    approver_id UUID NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    created_by UUID NOT NULL,
    updated_at TIMESTAMP,
    updated_by UUID NULL,
    deleted_at TIMESTAMP,
    deleted_by UUID NULL
);