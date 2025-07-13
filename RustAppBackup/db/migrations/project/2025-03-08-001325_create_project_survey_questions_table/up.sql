-- Your SQL goes here
CREATE TABLE IF NOT EXISTS project_survey_questions (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    no INT NOT NULL,
    question VARCHAR(255) NOT NULL,
    answer_type VARCHAR(255) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    created_by UUID NOT NULL,
    updated_at TIMESTAMP,
    updated_by UUID NULL,
    deleted_at TIMESTAMP,
    deleted_by UUID NULL
);