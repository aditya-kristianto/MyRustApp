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
use super::repository::Project::Project;
use tokio_postgres::Client;

#[path = "./v1/project.rs"]
pub mod project;
#[path = "./v1/project_approval.rs"]
pub mod project_approval;
#[path = "./v1/project_item.rs"]
pub mod project_item;
#[path = "./v1/project_request.rs"]
pub mod project_request;
#[path = "./v1/project_status.rs"]
pub mod project_status;
#[path = "./v1/project_survey_answer.rs"]
pub mod project_survey_answer;
#[path = "./v1/project_survey_feedback.rs"]
pub mod project_survey_feedback;
#[path = "./v1/project_survey_question.rs"]
pub mod project_survey_question;
#[path = "./v1/project_ticket.rs"]
pub mod project_ticket;
#[path = "./v1/project_ticket_approval.rs"]
pub mod project_ticket_approval;

pub fn configure_v1(config: &mut ServiceConfig) {
    config
        .service(project::create_project)
        .service(project::delete_project)
        .service(project::get_projects)
        .service(project::update_project)
        .service(project_approval::create_project_approval)
        .service(project_approval::delete_project_approval)
        .service(project_approval::get_project_approval)
        .service(project_approval::update_project_approval)
        .service(project_item::create_project_item)
        .service(project_item::delete_project_item)
        .service(project_item::get_project_item)
        .service(project_item::update_project_item)
        .service(project_request::create_project_request)
        .service(project_request::delete_project_request)
        .service(project_request::get_project_request)
        .service(project_request::update_project_request)
        .service(project_status::create_project_status)
        .service(project_status::delete_project_status)
        .service(project_status::get_project_status)
        .service(project_status::update_project_status)
        .service(project_survey_answer::create_project_survey_answer)
        .service(project_survey_answer::delete_project_survey_answer)
        .service(project_survey_answer::get_project_survey_answer)
        .service(project_survey_answer::create_project_survey_answer)
        .service(project_survey_question::create_project_survey_question)
        .service(project_survey_question::delete_project_survey_question)
        .service(project_survey_question::get_project_survey_question)
        .service(project_survey_question::create_project_survey_question)
        .service(project_ticket::create_project_ticket)
        .service(project_ticket::delete_project_ticket)
        .service(project_ticket::get_project_ticket)
        .service(project_ticket::update_project_ticket)
        .service(project_ticket_approval::create_project_ticket_approval)
        .service(project_ticket_approval::delete_project_ticket_approval)
        .service(project_ticket_approval::get_project_ticket_approval)
        .service(project_ticket_approval::update_project_ticket_approval);
}