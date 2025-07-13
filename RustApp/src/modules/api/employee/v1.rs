use actix_http::StatusCode;
use actix_web::web::ServiceConfig;
use actix_web::{delete, get, post, put, Result, web};
use actix_web::{Error, HttpRequest, HttpResponse, Responder, ResponseError};
use serde_json::to_string;
use std::collections::HashMap;
use std::sync::Arc;
use super::default_repository::DataValue;
use super::default_repository::Header;
use super::default_repository::Meta;
use super::default_repository::QueryParams;
use super::default_repository::Response;
use super::repository::Employee::Employee;
use tokio_postgres::Client;

#[path = "./v1/employee.rs"]
pub mod employee;
#[path = "./v1/employee_department.rs"]
pub mod employee_department;
#[path = "./v1/employee_directorate.rs"]
pub mod employee_directorate;
#[path = "./v1/employee_division.rs"]
pub mod employee_division;
#[path = "./v1/employee_position.rs"]
pub mod employee_position;
#[path = "./v1/employee_project.rs"]
pub mod employee_project;

pub fn configure_v1(config: &mut ServiceConfig) {
    config
        .service(employee::create_employee)
        .service(employee::get_employee)
        .service(employee::delete_employee)
        .service(employee::update_employee)
        .service(employee_department::create_employee_department)
        .service(employee_department::get_employee_department)
        .service(employee_department::delete_employee_department)
        .service(employee_department::update_employee_department)
        .service(employee_directorate::create_employee_directorate)
        .service(employee_directorate::get_employee_directorate)
        .service(employee_directorate::delete_employee_directorate)
        .service(employee_directorate::update_employee_directorate)
        .service(employee_division::create_employee_division)
        .service(employee_division::get_employee_division)
        .service(employee_division::delete_employee_division)
        .service(employee_division::update_employee_division)
        .service(employee_position::create_employee_position)
        .service(employee_position::get_employee_position)
        .service(employee_position::delete_employee_position)
        .service(employee_position::update_employee_position)
        .service(employee_project::create_employee_project)
        .service(employee_project::get_employee_project)
        .service(employee_project::delete_employee_project)
        .service(employee_project::update_employee_project);
}