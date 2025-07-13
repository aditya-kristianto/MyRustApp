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
pub struct EmployeeDivision {
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
pub struct EmployeeDepartment {
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
pub struct EmployeeDirectorate {
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
pub struct EmployeePosition {
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
pub struct Employee {
    id: Uuid,
    nik: String,
    name: String,
    email: String,
    position_id: Uuid,
    department_id: Uuid,
    division_id: Uuid,
    directorate_id: Uuid,
    created_at: DateTime<Utc>,
    created_by: Uuid,
    updated_at: Option<DateTime<Utc>>,
    updated_by: Option<Uuid>,
    deleted_at: Option<DateTime<Utc>>,
    deleted_by: Option<Uuid>,
}

impl Employee {
    pub fn new(
        id: Uuid,
        nik: String,
        name: String,
        email: String,
        position_id: Uuid,
        department_id: Uuid,
        division_id: Uuid,
        directorate_id: Uuid,
        created_by: Uuid,
    ) -> Self {
        Employee { 
            id, 
            nik, 
            name, 
            email, 
            position_id, 
            department_id, 
            division_id, 
            directorate_id, 
            created_at: Utc::now(), 
            created_by,
            updated_at: None,
            updated_by: None,
            deleted_at: None,
            deleted_by: None, 
        }
    }

    // Function to query the database and get employee
    pub async fn get_employee(data: web::Data<Arc<Client>>) -> Result<Vec<Employee>, Error> {
        // Access the database client from app_data
        let db_client = data.get_ref();
        let query_str: String = format!(
            "
                SELECT *
                FROM employees;
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

        let data: Vec<Employee> = rows
            .iter()
            .map(|row| {
                Employee{
                    id: row.get(0), 
                    nik: row.get(1), 
                    name: row.get(2), 
                    email: row.get(3), 
                    position_id: row.get(4), 
                    department_id: row.get(5), 
                    division_id: row.get(6), 
                    directorate_id: row.get(7), 
                    created_at: row.get(8),
                    created_by: row.get(9),
                    updated_at: row.get(10),
                    updated_by: row.get(11),
                    deleted_at: row.get(12),
                    deleted_by: row.get(13),
                }
            })
            .collect();
    
        Ok(data)
    }
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct EmployeeProject {
    id: Uuid,
    employee_id: Uuid,
    project_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub created_by: Uuid,
    pub updated_at: Option<DateTime<Utc>>,
    pub updated_by: Option<Uuid>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub deleted_by: Option<Uuid>,
}