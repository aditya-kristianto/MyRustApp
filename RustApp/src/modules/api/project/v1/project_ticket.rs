use actix_http::StatusCode;
use actix_web::web::ServiceConfig;
use actix_web::{delete, get, post, put, Result, web};
use actix_web::{Error, HttpRequest, HttpResponse, Responder, ResponseError};
use serde_json::to_string;
use std::collections::HashMap;
use std::sync::Arc;
use crate::DataValue;
use crate::Header;
use crate::Meta;
use crate::QueryParams;
use crate::Response;
use crate::Project;
use tokio_postgres::Client;

/// Create Project Ticket
///
/// To create new project ticket data
#[utoipa::path(
    post,
    context_path = "/v1",
    path = "/projects/tickets",
    params(Header),
    responses(
        (status = 100, description = "Continue", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::CONTINUE.to_string(), "message": "", "data":{"uuid": ""}, "meta":{"page": 0, "limit": 10}})),
        (status = 101, description = "Switching Protocols", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::SWITCHING_PROTOCOLS.to_string(), "message": ""})),
        (status = 103, description = "Early Hints", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": "103 Early Hints".to_string(), "message": ""})),
        (status = 200, description = "OK", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::OK.to_string(), "message": ""})),
        (status = 201, description = "Created", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::CREATED.to_string(), "message": ""})),
        (status = 202, description = "Accepted", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::ACCEPTED.to_string(), "message": ""})),
        (status = 203, description = "Non-Authoritative Information", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::NON_AUTHORITATIVE_INFORMATION.to_string(), "message": ""})),
        (status = 204, description = "No Content", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::NO_CONTENT.to_string(), "message": ""})),
        (status = 205, description = "Reset Content", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::RESET_CONTENT.to_string(), "message": ""})),
        (status = 206, description = "Partial Content", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::PARTIAL_CONTENT.to_string(), "message": ""})),
        (status = 300, description = "Multiple Choices", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::MULTIPLE_CHOICES.to_string(), "message": ""})),
        (status = 301, description = "Moved Permanently", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::MOVED_PERMANENTLY.to_string(), "message": ""})),
        (status = 302, description = "Found", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::FOUND.to_string(), "message": ""})),
        (status = 303, description = "See Other", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::SEE_OTHER.to_string(), "message": ""})),
        (status = 304, description = "Not Modified", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::NOT_MODIFIED.to_string(), "message": ""})),
        (status = 307, description = "Temporary Redirect", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::TEMPORARY_REDIRECT.to_string(), "message": ""})),
        (status = 308, description = "Permanent Redirect", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::PERMANENT_REDIRECT.to_string(), "message": ""})),
        (status = 400, description = "Bad Request", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::BAD_REQUEST.to_string(), "message": ""})),
        (status = 401, description = "Unauthorized", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::UNAUTHORIZED.to_string(), "message": ""})),
        (status = 402, description = "Payment Required", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::PAYMENT_REQUIRED.to_string(), "message": ""})),
        (status = 403, description = "Forbidden", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::FORBIDDEN.to_string(), "message": ""})),
        (status = 404, description = "Not Found", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::NOT_FOUND.to_string(), "message": ""})),
        (status = 405, description = "Method Not Allowed", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::METHOD_NOT_ALLOWED.to_string(), "message": ""})),
        (status = 406, description = "Not Acceptable", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::NOT_ACCEPTABLE.to_string(), "message": ""})),
        (status = 407, description = "Proxy Authentication Required", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::PROXY_AUTHENTICATION_REQUIRED.to_string(), "message": ""})),
        (status = 408, description = "Request Timeout", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::REQUEST_TIMEOUT.to_string(), "message": ""})),
        (status = 409, description = "Conflict", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::CONFLICT.to_string(), "message": ""})),
        (status = 410, description = "Gone", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::GONE.to_string(), "message": ""})),
        (status = 411, description = "Length Required", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::LENGTH_REQUIRED.to_string(), "message": ""})),
        (status = 412, description = "Precondition Failed", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::PRECONDITION_FAILED.to_string(), "message": ""})),
        (status = 413, description = "Payload Too Large", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::PAYLOAD_TOO_LARGE.to_string(), "message": ""})),
        (status = 414, description = "URI Too Long", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::URI_TOO_LONG.to_string(), "message": ""})),
        (status = 415, description = "Unsupported Media Type", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::UNSUPPORTED_MEDIA_TYPE.to_string(), "message": ""})),
        (status = 416, description = "Range Not Satisfiable", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::RANGE_NOT_SATISFIABLE.to_string(), "message": ""})),
        (status = 417, description = "Expectation Failed", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::EXPECTATION_FAILED.to_string(), "message": ""})),
        (status = 418, description = "I'm a teapot", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::IM_A_TEAPOT.to_string(), "message": ""})),
        (status = 422, description = "Unprocessable Entity", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::UNPROCESSABLE_ENTITY.to_string(), "message": ""})),
        (status = 425, description = "Too Early", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": "425 Too Early", "message": ""})),
        (status = 426, description = "Upgrade Required", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::UPGRADE_REQUIRED.to_string(), "message": ""})),
        (status = 428, description = "Precondition Required", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::PRECONDITION_REQUIRED.to_string(), "message": ""})),
        (status = 429, description = "Too Many Requests", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::TOO_MANY_REQUESTS.to_string(), "message": ""})),
        (status = 431, description = "Request Header Fields Too Large", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::REQUEST_HEADER_FIELDS_TOO_LARGE.to_string(), "message": ""})),
        (status = 451, description = "Unavailable For Legal Reasons", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::UNAVAILABLE_FOR_LEGAL_REASONS.to_string(), "message": ""})),
        (status = 500, description = "Internal Server Error", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::INTERNAL_SERVER_ERROR.to_string(), "message": ""})),
        (status = 501, description = "Not Implemented", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::NOT_IMPLEMENTED.to_string(), "message": ""})),
        (status = 502, description = "Bad Gateway", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::BAD_GATEWAY.to_string(), "message": ""})),
        (status = 503, description = "Service Unavailable", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::SERVICE_UNAVAILABLE.to_string(), "message": ""})),
        (status = 504, description = "Gateway Timeout", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::GATEWAY_TIMEOUT.to_string(), "message": ""})),
        (status = 505, description = "HTTP Version Not Supported", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::HTTP_VERSION_NOT_SUPPORTED.to_string(), "message": ""})),
        (status = 506, description = "Variant Also Negotiates", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::VARIANT_ALSO_NEGOTIATES.to_string(), "message": ""})),
        (status = 507, description = "Insufficient Storage", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::INSUFFICIENT_STORAGE.to_string(), "message": ""})),
        (status = 508, description = "Loop Detected", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::LOOP_DETECTED.to_string(), "message": ""})),
        (status = 510, description = "Not Extended", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::NOT_EXTENDED.to_string(), "message": ""})),
        (status = 511, description = "Network Authentication Required", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::NETWORK_AUTHENTICATION_REQUIRED.to_string(), "message": ""}))
    ),
    security(
        (),
        ("my_auth" = ["read:items", "edit:items"]),
        ("token_jwt" = [])
    ),
    tag = "Project Ticket"
)]
#[post("/v1/projects/tickets")]
pub async fn create_project_ticket(
    query: Result<web::Query<QueryParams>, Error>,
    data: web::Data<Arc<Client>>,
) -> Result<impl Responder> {
    let query_params;

    match query {
        Ok(params) => {
            query_params = params.into_inner();
        }
        Err(err) => {
            let resp = Response::new(
                None,
                None,
                Some(format!("Error: {}", err)),
                None,
                StatusCode::BAD_REQUEST.as_u16(),
            );

            return Ok(HttpResponse::BadRequest().json(web::Json(resp)));
        }
    }

    match query_params.validate() {
        Ok(_) => {}
        Err(err) => {
            let resp = Response::new(
                None,
                None,
                Some(err.to_string()),
                None,
                StatusCode::BAD_REQUEST.as_u16(),
            );

            return Ok(HttpResponse::BadRequest().json(web::Json(resp)));
        }
    }
    
    let _field = format!("{:?}", query_params.field);
    let _offset = query_params.offset.unwrap_or(0);
    let _limit = query_params.limit.unwrap_or(0); // You can choose a different default value if needed

    
    let projects = match Project::get_projects(data.clone()).await {
        Ok(summary) => summary,
        Err(err) => {
            let resp = Response::new(
                None,
                None,
                Some(format!("Error fetching projects: {}", err)),
                None,
                StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            );

            return Ok(HttpResponse::InternalServerError().json(web::Json(resp)));
        }
    };

    let projects_json: Vec<String> = projects.iter()
        .map(|e| to_string(e).unwrap()) // Convert each Employee to JSON string
        .collect();

    // Construct response with meta data
    let resp = Response::new(
        Some(HashMap::from([(
            "projects".to_string(),
            DataValue::StringArray(projects_json),
        )])),
        None,
        None,
        Some(Meta {
            // count: rows.len() as u8,
            count: 0,
            limit: 0,
            offset: 0,
        }),
        StatusCode::OK.as_u16(),
    );

    // Return successful response
    Ok(HttpResponse::Ok().json(web::Json(resp)))
}

/// Delete Project Ticket
///
/// To delete project ticket data
#[utoipa::path(
    delete,
    context_path = "/v1",
    path = "/projects/tickets",
    params(Header),
    responses(
        (status = 100, description = "Continue", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::CONTINUE.to_string(), "message": "", "data":{"uuid": ""}, "meta":{"page": 0, "limit": 10}})),
        (status = 101, description = "Switching Protocols", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::SWITCHING_PROTOCOLS.to_string(), "message": ""})),
        (status = 103, description = "Early Hints", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": "103 Early Hints".to_string(), "message": ""})),
        (status = 200, description = "OK", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::OK.to_string(), "message": ""})),
        (status = 201, description = "Created", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::CREATED.to_string(), "message": ""})),
        (status = 202, description = "Accepted", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::ACCEPTED.to_string(), "message": ""})),
        (status = 203, description = "Non-Authoritative Information", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::NON_AUTHORITATIVE_INFORMATION.to_string(), "message": ""})),
        (status = 204, description = "No Content", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::NO_CONTENT.to_string(), "message": ""})),
        (status = 205, description = "Reset Content", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::RESET_CONTENT.to_string(), "message": ""})),
        (status = 206, description = "Partial Content", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::PARTIAL_CONTENT.to_string(), "message": ""})),
        (status = 300, description = "Multiple Choices", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::MULTIPLE_CHOICES.to_string(), "message": ""})),
        (status = 301, description = "Moved Permanently", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::MOVED_PERMANENTLY.to_string(), "message": ""})),
        (status = 302, description = "Found", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::FOUND.to_string(), "message": ""})),
        (status = 303, description = "See Other", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::SEE_OTHER.to_string(), "message": ""})),
        (status = 304, description = "Not Modified", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::NOT_MODIFIED.to_string(), "message": ""})),
        (status = 307, description = "Temporary Redirect", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::TEMPORARY_REDIRECT.to_string(), "message": ""})),
        (status = 308, description = "Permanent Redirect", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::PERMANENT_REDIRECT.to_string(), "message": ""})),
        (status = 400, description = "Bad Request", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::BAD_REQUEST.to_string(), "message": ""})),
        (status = 401, description = "Unauthorized", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::UNAUTHORIZED.to_string(), "message": ""})),
        (status = 402, description = "Payment Required", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::PAYMENT_REQUIRED.to_string(), "message": ""})),
        (status = 403, description = "Forbidden", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::FORBIDDEN.to_string(), "message": ""})),
        (status = 404, description = "Not Found", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::NOT_FOUND.to_string(), "message": ""})),
        (status = 405, description = "Method Not Allowed", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::METHOD_NOT_ALLOWED.to_string(), "message": ""})),
        (status = 406, description = "Not Acceptable", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::NOT_ACCEPTABLE.to_string(), "message": ""})),
        (status = 407, description = "Proxy Authentication Required", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::PROXY_AUTHENTICATION_REQUIRED.to_string(), "message": ""})),
        (status = 408, description = "Request Timeout", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::REQUEST_TIMEOUT.to_string(), "message": ""})),
        (status = 409, description = "Conflict", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::CONFLICT.to_string(), "message": ""})),
        (status = 410, description = "Gone", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::GONE.to_string(), "message": ""})),
        (status = 411, description = "Length Required", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::LENGTH_REQUIRED.to_string(), "message": ""})),
        (status = 412, description = "Precondition Failed", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::PRECONDITION_FAILED.to_string(), "message": ""})),
        (status = 413, description = "Payload Too Large", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::PAYLOAD_TOO_LARGE.to_string(), "message": ""})),
        (status = 414, description = "URI Too Long", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::URI_TOO_LONG.to_string(), "message": ""})),
        (status = 415, description = "Unsupported Media Type", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::UNSUPPORTED_MEDIA_TYPE.to_string(), "message": ""})),
        (status = 416, description = "Range Not Satisfiable", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::RANGE_NOT_SATISFIABLE.to_string(), "message": ""})),
        (status = 417, description = "Expectation Failed", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::EXPECTATION_FAILED.to_string(), "message": ""})),
        (status = 418, description = "I'm a teapot", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::IM_A_TEAPOT.to_string(), "message": ""})),
        (status = 422, description = "Unprocessable Entity", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::UNPROCESSABLE_ENTITY.to_string(), "message": ""})),
        (status = 425, description = "Too Early", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": "425 Too Early", "message": ""})),
        (status = 426, description = "Upgrade Required", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::UPGRADE_REQUIRED.to_string(), "message": ""})),
        (status = 428, description = "Precondition Required", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::PRECONDITION_REQUIRED.to_string(), "message": ""})),
        (status = 429, description = "Too Many Requests", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::TOO_MANY_REQUESTS.to_string(), "message": ""})),
        (status = 431, description = "Request Header Fields Too Large", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::REQUEST_HEADER_FIELDS_TOO_LARGE.to_string(), "message": ""})),
        (status = 451, description = "Unavailable For Legal Reasons", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::UNAVAILABLE_FOR_LEGAL_REASONS.to_string(), "message": ""})),
        (status = 500, description = "Internal Server Error", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::INTERNAL_SERVER_ERROR.to_string(), "message": ""})),
        (status = 501, description = "Not Implemented", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::NOT_IMPLEMENTED.to_string(), "message": ""})),
        (status = 502, description = "Bad Gateway", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::BAD_GATEWAY.to_string(), "message": ""})),
        (status = 503, description = "Service Unavailable", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::SERVICE_UNAVAILABLE.to_string(), "message": ""})),
        (status = 504, description = "Gateway Timeout", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::GATEWAY_TIMEOUT.to_string(), "message": ""})),
        (status = 505, description = "HTTP Version Not Supported", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::HTTP_VERSION_NOT_SUPPORTED.to_string(), "message": ""})),
        (status = 506, description = "Variant Also Negotiates", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::VARIANT_ALSO_NEGOTIATES.to_string(), "message": ""})),
        (status = 507, description = "Insufficient Storage", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::INSUFFICIENT_STORAGE.to_string(), "message": ""})),
        (status = 508, description = "Loop Detected", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::LOOP_DETECTED.to_string(), "message": ""})),
        (status = 510, description = "Not Extended", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::NOT_EXTENDED.to_string(), "message": ""})),
        (status = 511, description = "Network Authentication Required", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::NETWORK_AUTHENTICATION_REQUIRED.to_string(), "message": ""}))
    ),
    security(
        (),
        ("my_auth" = ["read:items", "edit:items"]),
        ("token_jwt" = [])
    ),
    tag = "Project Ticket"
)]
#[delete("/v1/projects/tickets")]
pub async fn delete_project_ticket(
    query: Result<web::Query<QueryParams>, Error>,
    data: web::Data<Arc<Client>>,
) -> Result<impl Responder> {
    let query_params;

    match query {
        Ok(params) => {
            query_params = params.into_inner();
        }
        Err(err) => {
            let resp = Response::new(
                None,
                None,
                Some(format!("Error: {}", err)),
                None,
                StatusCode::BAD_REQUEST.as_u16(),
            );

            return Ok(HttpResponse::BadRequest().json(web::Json(resp)));
        }
    }

    match query_params.validate() {
        Ok(_) => {}
        Err(err) => {
            let resp = Response::new(
                None,
                None,
                Some(err.to_string()),
                None,
                StatusCode::BAD_REQUEST.as_u16(),
            );

            return Ok(HttpResponse::BadRequest().json(web::Json(resp)));
        }
    }
    
    let _field = format!("{:?}", query_params.field);
    let _offset = query_params.offset.unwrap_or(0);
    let _limit = query_params.limit.unwrap_or(0); // You can choose a different default value if needed

    
    let projects = match Project::get_projects(data.clone()).await {
        Ok(summary) => summary,
        Err(err) => {
            let resp = Response::new(
                None,
                None,
                Some(format!("Error fetching projects: {}", err)),
                None,
                StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            );

            return Ok(HttpResponse::InternalServerError().json(web::Json(resp)));
        }
    };

    let projects_json: Vec<String> = projects.iter()
        .map(|e| to_string(e).unwrap()) // Convert each Employee to JSON string
        .collect();

    // Construct response with meta data
    let resp = Response::new(
        Some(HashMap::from([(
            "projects".to_string(),
            DataValue::StringArray(projects_json),
        )])),
        None,
        None,
        Some(Meta {
            // count: rows.len() as u8,
            count: 0,
            limit: 0,
            offset: 0,
        }),
        StatusCode::OK.as_u16(),
    );

    // Return successful response
    Ok(HttpResponse::Ok().json(web::Json(resp)))
}

/// Get Project Ticket
///
/// To get projects ticket list
#[utoipa::path(
    get,
    context_path = "/v1",
    path = "/projects/tickets",
    params(Header),
    responses(
        (status = 100, description = "Continue", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::CONTINUE.to_string(), "message": "", "data":{"uuid": ""}, "meta":{"page": 0, "limit": 10}})),
        (status = 101, description = "Switching Protocols", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::SWITCHING_PROTOCOLS.to_string(), "message": ""})),
        (status = 103, description = "Early Hints", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": "103 Early Hints".to_string(), "message": ""})),
        (status = 200, description = "OK", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::OK.to_string(), "message": ""})),
        (status = 201, description = "Created", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::CREATED.to_string(), "message": ""})),
        (status = 202, description = "Accepted", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::ACCEPTED.to_string(), "message": ""})),
        (status = 203, description = "Non-Authoritative Information", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::NON_AUTHORITATIVE_INFORMATION.to_string(), "message": ""})),
        (status = 204, description = "No Content", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::NO_CONTENT.to_string(), "message": ""})),
        (status = 205, description = "Reset Content", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::RESET_CONTENT.to_string(), "message": ""})),
        (status = 206, description = "Partial Content", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::PARTIAL_CONTENT.to_string(), "message": ""})),
        (status = 300, description = "Multiple Choices", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::MULTIPLE_CHOICES.to_string(), "message": ""})),
        (status = 301, description = "Moved Permanently", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::MOVED_PERMANENTLY.to_string(), "message": ""})),
        (status = 302, description = "Found", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::FOUND.to_string(), "message": ""})),
        (status = 303, description = "See Other", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::SEE_OTHER.to_string(), "message": ""})),
        (status = 304, description = "Not Modified", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::NOT_MODIFIED.to_string(), "message": ""})),
        (status = 307, description = "Temporary Redirect", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::TEMPORARY_REDIRECT.to_string(), "message": ""})),
        (status = 308, description = "Permanent Redirect", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::PERMANENT_REDIRECT.to_string(), "message": ""})),
        (status = 400, description = "Bad Request", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::BAD_REQUEST.to_string(), "message": ""})),
        (status = 401, description = "Unauthorized", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::UNAUTHORIZED.to_string(), "message": ""})),
        (status = 402, description = "Payment Required", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::PAYMENT_REQUIRED.to_string(), "message": ""})),
        (status = 403, description = "Forbidden", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::FORBIDDEN.to_string(), "message": ""})),
        (status = 404, description = "Not Found", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::NOT_FOUND.to_string(), "message": ""})),
        (status = 405, description = "Method Not Allowed", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::METHOD_NOT_ALLOWED.to_string(), "message": ""})),
        (status = 406, description = "Not Acceptable", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::NOT_ACCEPTABLE.to_string(), "message": ""})),
        (status = 407, description = "Proxy Authentication Required", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::PROXY_AUTHENTICATION_REQUIRED.to_string(), "message": ""})),
        (status = 408, description = "Request Timeout", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::REQUEST_TIMEOUT.to_string(), "message": ""})),
        (status = 409, description = "Conflict", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::CONFLICT.to_string(), "message": ""})),
        (status = 410, description = "Gone", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::GONE.to_string(), "message": ""})),
        (status = 411, description = "Length Required", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::LENGTH_REQUIRED.to_string(), "message": ""})),
        (status = 412, description = "Precondition Failed", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::PRECONDITION_FAILED.to_string(), "message": ""})),
        (status = 413, description = "Payload Too Large", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::PAYLOAD_TOO_LARGE.to_string(), "message": ""})),
        (status = 414, description = "URI Too Long", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::URI_TOO_LONG.to_string(), "message": ""})),
        (status = 415, description = "Unsupported Media Type", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::UNSUPPORTED_MEDIA_TYPE.to_string(), "message": ""})),
        (status = 416, description = "Range Not Satisfiable", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::RANGE_NOT_SATISFIABLE.to_string(), "message": ""})),
        (status = 417, description = "Expectation Failed", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::EXPECTATION_FAILED.to_string(), "message": ""})),
        (status = 418, description = "I'm a teapot", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::IM_A_TEAPOT.to_string(), "message": ""})),
        (status = 422, description = "Unprocessable Entity", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::UNPROCESSABLE_ENTITY.to_string(), "message": ""})),
        (status = 425, description = "Too Early", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": "425 Too Early", "message": ""})),
        (status = 426, description = "Upgrade Required", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::UPGRADE_REQUIRED.to_string(), "message": ""})),
        (status = 428, description = "Precondition Required", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::PRECONDITION_REQUIRED.to_string(), "message": ""})),
        (status = 429, description = "Too Many Requests", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::TOO_MANY_REQUESTS.to_string(), "message": ""})),
        (status = 431, description = "Request Header Fields Too Large", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::REQUEST_HEADER_FIELDS_TOO_LARGE.to_string(), "message": ""})),
        (status = 451, description = "Unavailable For Legal Reasons", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::UNAVAILABLE_FOR_LEGAL_REASONS.to_string(), "message": ""})),
        (status = 500, description = "Internal Server Error", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::INTERNAL_SERVER_ERROR.to_string(), "message": ""})),
        (status = 501, description = "Not Implemented", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::NOT_IMPLEMENTED.to_string(), "message": ""})),
        (status = 502, description = "Bad Gateway", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::BAD_GATEWAY.to_string(), "message": ""})),
        (status = 503, description = "Service Unavailable", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::SERVICE_UNAVAILABLE.to_string(), "message": ""})),
        (status = 504, description = "Gateway Timeout", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::GATEWAY_TIMEOUT.to_string(), "message": ""})),
        (status = 505, description = "HTTP Version Not Supported", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::HTTP_VERSION_NOT_SUPPORTED.to_string(), "message": ""})),
        (status = 506, description = "Variant Also Negotiates", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::VARIANT_ALSO_NEGOTIATES.to_string(), "message": ""})),
        (status = 507, description = "Insufficient Storage", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::INSUFFICIENT_STORAGE.to_string(), "message": ""})),
        (status = 508, description = "Loop Detected", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::LOOP_DETECTED.to_string(), "message": ""})),
        (status = 510, description = "Not Extended", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::NOT_EXTENDED.to_string(), "message": ""})),
        (status = 511, description = "Network Authentication Required", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::NETWORK_AUTHENTICATION_REQUIRED.to_string(), "message": ""}))
    ),
    security(
        (),
        ("my_auth" = ["read:items", "edit:items"]),
        ("token_jwt" = [])
    ),
    tag = "Project Ticket"
)]
#[get("/v1/projects/tickets")]
pub async fn get_project_ticket(
    query: Result<web::Query<QueryParams>, Error>,
    data: web::Data<Arc<Client>>,
) -> Result<impl Responder> {
    let query_params;

    match query {
        Ok(params) => {
            query_params = params.into_inner();
        }
        Err(err) => {
            let resp = Response::new(
                None,
                None,
                Some(format!("Error: {}", err)),
                None,
                StatusCode::BAD_REQUEST.as_u16(),
            );

            return Ok(HttpResponse::BadRequest().json(web::Json(resp)));
        }
    }

    match query_params.validate() {
        Ok(_) => {}
        Err(err) => {
            let resp = Response::new(
                None,
                None,
                Some(err.to_string()),
                None,
                StatusCode::BAD_REQUEST.as_u16(),
            );

            return Ok(HttpResponse::BadRequest().json(web::Json(resp)));
        }
    }
    
    let _field = format!("{:?}", query_params.field);
    let _offset = query_params.offset.unwrap_or(0);
    let _limit = query_params.limit.unwrap_or(0); // You can choose a different default value if needed

    
    let projects = match Project::get_projects(data.clone()).await {
        Ok(summary) => summary,
        Err(err) => {
            let resp = Response::new(
                None,
                None,
                Some(format!("Error fetching projects: {}", err)),
                None,
                StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            );

            return Ok(HttpResponse::InternalServerError().json(web::Json(resp)));
        }
    };

    let projects_json: Vec<String> = projects.iter()
        .map(|e| to_string(e).unwrap()) // Convert each Employee to JSON string
        .collect();

    // Construct response with meta data
    let resp = Response::new(
        Some(HashMap::from([(
            "projects".to_string(),
            DataValue::StringArray(projects_json),
        )])),
        None,
        None,
        Some(Meta {
            // count: rows.len() as u8,
            count: 0,
            limit: 0,
            offset: 0,
        }),
        StatusCode::OK.as_u16(),
    );

    // Return successful response
    Ok(HttpResponse::Ok().json(web::Json(resp)))
}

/// Update Project Ticket
///
/// To update project ticket data
#[utoipa::path(
    put,
    context_path = "/v1",
    path = "/projects/tickets",
    params(Header),
    responses(
        (status = 100, description = "Continue", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::CONTINUE.to_string(), "message": "", "data":{"uuid": ""}, "meta":{"page": 0, "limit": 10}})),
        (status = 101, description = "Switching Protocols", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::SWITCHING_PROTOCOLS.to_string(), "message": ""})),
        (status = 103, description = "Early Hints", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": "103 Early Hints".to_string(), "message": ""})),
        (status = 200, description = "OK", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::OK.to_string(), "message": ""})),
        (status = 201, description = "Created", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::CREATED.to_string(), "message": ""})),
        (status = 202, description = "Accepted", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::ACCEPTED.to_string(), "message": ""})),
        (status = 203, description = "Non-Authoritative Information", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::NON_AUTHORITATIVE_INFORMATION.to_string(), "message": ""})),
        (status = 204, description = "No Content", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::NO_CONTENT.to_string(), "message": ""})),
        (status = 205, description = "Reset Content", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::RESET_CONTENT.to_string(), "message": ""})),
        (status = 206, description = "Partial Content", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::PARTIAL_CONTENT.to_string(), "message": ""})),
        (status = 300, description = "Multiple Choices", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::MULTIPLE_CHOICES.to_string(), "message": ""})),
        (status = 301, description = "Moved Permanently", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::MOVED_PERMANENTLY.to_string(), "message": ""})),
        (status = 302, description = "Found", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::FOUND.to_string(), "message": ""})),
        (status = 303, description = "See Other", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::SEE_OTHER.to_string(), "message": ""})),
        (status = 304, description = "Not Modified", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::NOT_MODIFIED.to_string(), "message": ""})),
        (status = 307, description = "Temporary Redirect", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::TEMPORARY_REDIRECT.to_string(), "message": ""})),
        (status = 308, description = "Permanent Redirect", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::PERMANENT_REDIRECT.to_string(), "message": ""})),
        (status = 400, description = "Bad Request", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::BAD_REQUEST.to_string(), "message": ""})),
        (status = 401, description = "Unauthorized", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::UNAUTHORIZED.to_string(), "message": ""})),
        (status = 402, description = "Payment Required", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::PAYMENT_REQUIRED.to_string(), "message": ""})),
        (status = 403, description = "Forbidden", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::FORBIDDEN.to_string(), "message": ""})),
        (status = 404, description = "Not Found", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::NOT_FOUND.to_string(), "message": ""})),
        (status = 405, description = "Method Not Allowed", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::METHOD_NOT_ALLOWED.to_string(), "message": ""})),
        (status = 406, description = "Not Acceptable", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::NOT_ACCEPTABLE.to_string(), "message": ""})),
        (status = 407, description = "Proxy Authentication Required", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::PROXY_AUTHENTICATION_REQUIRED.to_string(), "message": ""})),
        (status = 408, description = "Request Timeout", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::REQUEST_TIMEOUT.to_string(), "message": ""})),
        (status = 409, description = "Conflict", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::CONFLICT.to_string(), "message": ""})),
        (status = 410, description = "Gone", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::GONE.to_string(), "message": ""})),
        (status = 411, description = "Length Required", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::LENGTH_REQUIRED.to_string(), "message": ""})),
        (status = 412, description = "Precondition Failed", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::PRECONDITION_FAILED.to_string(), "message": ""})),
        (status = 413, description = "Payload Too Large", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::PAYLOAD_TOO_LARGE.to_string(), "message": ""})),
        (status = 414, description = "URI Too Long", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::URI_TOO_LONG.to_string(), "message": ""})),
        (status = 415, description = "Unsupported Media Type", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::UNSUPPORTED_MEDIA_TYPE.to_string(), "message": ""})),
        (status = 416, description = "Range Not Satisfiable", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::RANGE_NOT_SATISFIABLE.to_string(), "message": ""})),
        (status = 417, description = "Expectation Failed", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::EXPECTATION_FAILED.to_string(), "message": ""})),
        (status = 418, description = "I'm a teapot", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::IM_A_TEAPOT.to_string(), "message": ""})),
        (status = 422, description = "Unprocessable Entity", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::UNPROCESSABLE_ENTITY.to_string(), "message": ""})),
        (status = 425, description = "Too Early", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": "425 Too Early", "message": ""})),
        (status = 426, description = "Upgrade Required", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::UPGRADE_REQUIRED.to_string(), "message": ""})),
        (status = 428, description = "Precondition Required", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::PRECONDITION_REQUIRED.to_string(), "message": ""})),
        (status = 429, description = "Too Many Requests", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::TOO_MANY_REQUESTS.to_string(), "message": ""})),
        (status = 431, description = "Request Header Fields Too Large", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::REQUEST_HEADER_FIELDS_TOO_LARGE.to_string(), "message": ""})),
        (status = 451, description = "Unavailable For Legal Reasons", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::UNAVAILABLE_FOR_LEGAL_REASONS.to_string(), "message": ""})),
        (status = 500, description = "Internal Server Error", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::INTERNAL_SERVER_ERROR.to_string(), "message": ""})),
        (status = 501, description = "Not Implemented", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::NOT_IMPLEMENTED.to_string(), "message": ""})),
        (status = 502, description = "Bad Gateway", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::BAD_GATEWAY.to_string(), "message": ""})),
        (status = 503, description = "Service Unavailable", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::SERVICE_UNAVAILABLE.to_string(), "message": ""})),
        (status = 504, description = "Gateway Timeout", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::GATEWAY_TIMEOUT.to_string(), "message": ""})),
        (status = 505, description = "HTTP Version Not Supported", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::HTTP_VERSION_NOT_SUPPORTED.to_string(), "message": ""})),
        (status = 506, description = "Variant Also Negotiates", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::VARIANT_ALSO_NEGOTIATES.to_string(), "message": ""})),
        (status = 507, description = "Insufficient Storage", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::INSUFFICIENT_STORAGE.to_string(), "message": ""})),
        (status = 508, description = "Loop Detected", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::LOOP_DETECTED.to_string(), "message": ""})),
        (status = 510, description = "Not Extended", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::NOT_EXTENDED.to_string(), "message": ""})),
        (status = 511, description = "Network Authentication Required", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::NETWORK_AUTHENTICATION_REQUIRED.to_string(), "message": ""}))
    ),
    security(
        (),
        ("my_auth" = ["read:items", "edit:items"]),
        ("token_jwt" = [])
    ),
    tag = "Project Ticket"
)]
#[put("/v1/projects/tickets")]
pub async fn update_project_ticket(
    query: Result<web::Query<QueryParams>, Error>,
    data: web::Data<Arc<Client>>,
) -> Result<impl Responder> {
    let query_params;

    match query {
        Ok(params) => {
            query_params = params.into_inner();
        }
        Err(err) => {
            let resp = Response::new(
                None,
                None,
                Some(format!("Error: {}", err)),
                None,
                StatusCode::BAD_REQUEST.as_u16(),
            );

            return Ok(HttpResponse::BadRequest().json(web::Json(resp)));
        }
    }

    match query_params.validate() {
        Ok(_) => {}
        Err(err) => {
            let resp = Response::new(
                None,
                None,
                Some(err.to_string()),
                None,
                StatusCode::BAD_REQUEST.as_u16(),
            );

            return Ok(HttpResponse::BadRequest().json(web::Json(resp)));
        }
    }
    
    let _field = format!("{:?}", query_params.field);
    let _offset = query_params.offset.unwrap_or(0);
    let _limit = query_params.limit.unwrap_or(0); // You can choose a different default value if needed

    
    let projects = match Project::get_projects(data.clone()).await {
        Ok(summary) => summary,
        Err(err) => {
            let resp = Response::new(
                None,
                None,
                Some(format!("Error fetching projects: {}", err)),
                None,
                StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            );

            return Ok(HttpResponse::InternalServerError().json(web::Json(resp)));
        }
    };

    let projects_json: Vec<String> = projects.iter()
        .map(|e| to_string(e).unwrap()) // Convert each Employee to JSON string
        .collect();

    // Construct response with meta data
    let resp = Response::new(
        Some(HashMap::from([(
            "projects".to_string(),
            DataValue::StringArray(projects_json),
        )])),
        None,
        None,
        Some(Meta {
            // count: rows.len() as u8,
            count: 0,
            limit: 0,
            offset: 0,
        }),
        StatusCode::OK.as_u16(),
    );

    // Return successful response
    Ok(HttpResponse::Ok().json(web::Json(resp)))
}