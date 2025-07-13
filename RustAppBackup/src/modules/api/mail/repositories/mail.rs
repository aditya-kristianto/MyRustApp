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
pub struct MailHistory {
    id: Uuid,
    project_id: Uuid,
    project_status_id: Uuid,
    sender_id: Uuid,
    to: String,
    cc: String,
    created_at: DateTime<Utc>,
    created_by: Uuid,
    updated_at: Option<DateTime<Utc>>,
    updated_by: Option<Uuid>,
    deleted_at: Option<DateTime<Utc>>,
    deleted_by: Option<Uuid>,
}

impl MailHistory {
    pub fn new(
        id: Uuid,
        project_id: Uuid,
        project_status_id: Uuid,
        sender_id: Uuid,
        to: String,
        cc: String,
        created_by: Uuid,
    ) -> Self {
        MailHistory { 
            id, 
            project_id, 
            project_status_id, 
            sender_id, 
            to, 
            cc,
            created_at: Utc::now(), 
            created_by,
            updated_at: None,
            updated_by: None,
            deleted_at: None,
            deleted_by: None, 
        }
    }

    // Function to query the database and get mail histories
    pub async fn get_mail_histories(data: web::Data<Arc<Client>>) -> Result<Vec<MailHistory>, Error> {
        // Access the database client from app_data
        let db_client = data.get_ref();
        let query_str: String = format!(
            "
                SELECT *
                FROM mail_histories;
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

        let data: Vec<MailHistory> = rows
            .iter()
            .map(|row| {
                MailHistory{
                    id: row.get(0), 
                    project_id: row.get(1), 
                    project_status_id: row.get(2), 
                    sender_id: row.get(3), 
                    to: row.get(4), 
                    cc: row.get(5),
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