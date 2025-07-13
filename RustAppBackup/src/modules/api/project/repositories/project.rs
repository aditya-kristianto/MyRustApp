use actix_http::StatusCode;
use actix_web::HttpResponse;
use actix_web::Error;
use actix_web::web;
use chrono::DateTime;
use chrono::Utc;
use serde::Deserialize;
use serde::Serialize;
use std::sync::Arc;
use utoipa::{ToSchema};
use uuid::Uuid;
use crate::Response;
use tokio_postgres::Client;

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct ProjectSatus {
    id: Uuid,
    name: String,
    pub created_at: DateTime<Utc>,
    pub created_by: Uuid,
    pub updated_at: Option<DateTime<Utc>>,
    pub updated_by: Option<Uuid>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub deleted_by: Option<Uuid>,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct Project {
    id: Uuid,
    name: String,
    requestor_id: Uuid,
    project_status_id: Uuid,
    created_at: DateTime<Utc>,
    created_by: Uuid,
    updated_at: Option<DateTime<Utc>>,
    updated_by: Option<Uuid>,
    deleted_at: Option<DateTime<Utc>>,
    deleted_by: Option<Uuid>,
}

impl Project {
    pub fn new(
        id: Uuid,
        name: String,
        requestor_id: Uuid,
        project_status_id: Uuid,
        created_by: Uuid,
    ) -> Self {
        Project { 
            id, 
            name, 
            requestor_id, 
            project_status_id, 
            created_at: Utc::now(),
            created_by,
            updated_at: None,
            updated_by: None,
            deleted_at: None,
            deleted_by: None, 
        }
    }

    // Function to query the database and get projects
    pub async fn get_projects(data: web::Data<Arc<Client>>) -> Result<Vec<Project>, Error> {
        // Access the database client from app_data
        let db_client = data.get_ref();
        let query_str: String = format!(
            "
                SELECT *
                FROM projects;
            "
        );

        let rows = db_client
            .query(&query_str, &[])
            .await
            .map_err(|e| {
                eprintln!("Error executing query: {:?}", e);
    
                let resp = Response::new(
                    None,
                    None,
                    Some("".to_string()),
                    None,
                    StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                );
    
                actix_web::error::InternalError::new(HttpResponse::InternalServerError().json(resp), StatusCode::INTERNAL_SERVER_ERROR)
            })
            .unwrap();

        let data: Vec<Project> = rows
            .iter()
            .map(|row| {
                Project{
                    id: row.get(0), 
                    name: row.get(1), 
                    requestor_id: row.get(2),
                    project_status_id: row.get(3),
                    created_at: row.get(6), 
                    created_by: row.get(7),
                    updated_at: row.get(8),
                    updated_by: row.get(9),
                    deleted_at: row.get(10),
                    deleted_by: row.get(11),
                }
            })
            .collect();
    
        Ok(data)
    }
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct ProjectApproval {
    id: Uuid,
    name: String,
    project_id: Uuid,
    approver_id: Uuid,
    note: String,
    pub created_at: DateTime<Utc>,
    pub created_by: Uuid,
    pub updated_at: Option<DateTime<Utc>>,
    pub updated_by: Option<Uuid>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub deleted_by: Option<Uuid>,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct ProjectTicketApproval {
    id: Uuid,
    project_id: Uuid,
    ticket_id: Uuid,
    approver_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub created_by: Uuid,
    pub updated_at: Option<DateTime<Utc>>,
    pub updated_by: Option<Uuid>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub deleted_by: Option<Uuid>,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct ProjectTicket {
    id: Uuid,
    project_id: Uuid,
    ticket_type: String,
    pub created_at: DateTime<Utc>,
    pub created_by: Uuid,
    pub updated_at: Option<DateTime<Utc>>,
    pub updated_by: Option<Uuid>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub deleted_by: Option<Uuid>,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct ProjectItem {
    id: Uuid,
    project_id: Uuid,
    name: String,
    timeline_start: DateTime<Utc>,
    timeline_end: DateTime<Utc>,
    duration: i32,
    project_status_id: Uuid,
    progress: String,
    bundling_no: Option<i32>,
    pub created_at: DateTime<Utc>,
    pub created_by: Uuid,
    pub updated_at: Option<DateTime<Utc>>,
    pub updated_by: Option<Uuid>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub deleted_by: Option<Uuid>,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct ProjectRequest {
    id: Uuid,
    requestor_name: String,
    requestor_email: String,
    requestor_directorate: String,
    project_category: String,
    project_title: String,
    project_background: String,
    brd_link: String,
    live_date_estimation: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub created_by: Uuid,
    pub updated_at: Option<DateTime<Utc>>,
    pub updated_by: Option<Uuid>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub deleted_by: Option<Uuid>,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct ProjectSurveyQuestion {
    id: Uuid,
    no: i32,
    question: String,
    answer_type: String,
    pub created_at: DateTime<Utc>,
    pub created_by: Uuid,
    pub updated_at: Option<DateTime<Utc>>,
    pub updated_by: Option<Uuid>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub deleted_by: Option<Uuid>,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct ProjectSurveyFeedback {
    id: Uuid,
    project_id: Uuid,
    surveyor_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub created_by: Uuid,
    pub updated_at: Option<DateTime<Utc>>,
    pub updated_by: Option<Uuid>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub deleted_by: Option<Uuid>,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct ProjectSurveyAnswer {
    id: Uuid,
    project_survey_feedback_id: Uuid,
    project_survey_question_id: Uuid,
    answer: String,
    pub created_at: DateTime<Utc>,
    pub created_by: Uuid,
    pub updated_at: Option<DateTime<Utc>>,
    pub updated_by: Option<Uuid>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub deleted_by: Option<Uuid>,
}