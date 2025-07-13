-- Your SQL goes here
CREATE TABLE IF NOT EXISTS project_survey_answers (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    project_survey_feedback_id UUID REFERENCES project_survey_feedbacks(id),
    project_survey_question_id UUID REFERENCES project_survey_questions(id),
    answer VARCHAR(255) NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    created_by UUID NOT NULL,
    updated_at TIMESTAMP,
    updated_by UUID NULL,
    deleted_at TIMESTAMP,
    deleted_by UUID NULL
);