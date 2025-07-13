use actix_http::StatusCode;
use actix_web::delete;
use actix_web::get;
use actix_web::post;
use actix_web::put;
use actix_web::HttpResponse;
use actix_web::web;
use actix_web::web::ServiceConfig;
use mime::APPLICATION_JSON;
use std::sync::Arc;
use tokio_postgres::Client;
use tokio_postgres::Row;
use tokio_postgres::Statement;
use tokio_postgres::types::Type;
use uuid::Uuid;

#[path = "user.rs"] mod model;
#[path = "../../../../pkg/response/response.rs"] mod response;

pub fn configure_v1(config: &mut ServiceConfig) {
    config
        .service(create_user)
        .service(delete_user)
        .service(select_user)
        .service(select_user_by_id)
        .service(update_user);
}

/// Create new user
#[utoipa::path(
post,
path = "/users",
responses(
(status = 100, description = "Continue", content_type = "application/json", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::CONTINUE.to_string(), "message": ""})),
(status = 101, description = "Switching Protocols", content_type = "application/json", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::SWITCHING_PROTOCOLS.to_string(), "message": ""})),
(status = 103, description = "Early Hints", content_type = "application/json", content_type = "application/json", content_type = "application/json", body = Response,
example = json!({"status": "103 Early Hints".to_string(), "message": ""})),
(status = 200, description = "OK", content_type = "application/json", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::OK.to_string(), "message": ""})),
(status = 201, description = "Created", content_type = "application/json", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::CREATED.to_string(), "message": ""})),
(status = 202, description = "Accepted", content_type = "application/json", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::ACCEPTED.to_string(), "message": ""})),
(status = 203, description = "Non-Authoritative Information", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::NON_AUTHORITATIVE_INFORMATION.to_string(), "message": ""})),
(status = 204, description = "No Content", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::NO_CONTENT.to_string(), "message": ""})),
(status = 205, description = "Reset Content", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::RESET_CONTENT.to_string(), "message": ""})),
(status = 206, description = "Partial Content", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::PARTIAL_CONTENT.to_string(), "message": ""})),
(status = 300, description = "Multiple Choices", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::MULTIPLE_CHOICES.to_string(), "message": ""})),
(status = 301, description = "Moved Permanently", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::MOVED_PERMANENTLY.to_string(), "message": ""})),
(status = 302, description = "Found", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::FOUND.to_string(), "message": ""})),
(status = 303, description = "See Other", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::SEE_OTHER.to_string(), "message": ""})),
(status = 304, description = "Not Modified", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::NOT_MODIFIED.to_string(), "message": ""})),
(status = 307, description = "Temporary Redirect", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::TEMPORARY_REDIRECT.to_string(), "message": ""})),
(status = 308, description = "Permanent Redirect", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::PERMANENT_REDIRECT.to_string(), "message": ""})),
(status = 400, description = "Bad Request", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::BAD_REQUEST.to_string(), "message": ""})),
(status = 401, description = "Unauthorized", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::UNAUTHORIZED.to_string(), "message": ""})),
(status = 402, description = "Payment Required", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::PAYMENT_REQUIRED.to_string(), "message": ""})),
(status = 403, description = "Forbidden", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::FORBIDDEN.to_string(), "message": ""})),
(status = 404, description = "Not Found", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::NOT_FOUND.to_string(), "message": ""})),
(status = 405, description = "Method Not Allowed", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::METHOD_NOT_ALLOWED.to_string(), "message": ""})),
(status = 406, description = "Not Acceptable", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::NOT_ACCEPTABLE.to_string(), "message": ""})),
(status = 407, description = "Proxy Authentication Required", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::PROXY_AUTHENTICATION_REQUIRED.to_string(), "message": ""})),
(status = 408, description = "Request Timeout", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::REQUEST_TIMEOUT.to_string(), "message": ""})),
(status = 409, description = "Conflict", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::CONFLICT.to_string(), "message": ""})),
(status = 410, description = "Gone", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::GONE.to_string(), "message": ""})),
(status = 411, description = "Length Required", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::LENGTH_REQUIRED.to_string(), "message": ""})),
(status = 412, description = "Precondition Failed", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::PRECONDITION_FAILED.to_string(), "message": ""})),
(status = 413, description = "Payload Too Large", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::PAYLOAD_TOO_LARGE.to_string(), "message": ""})),
(status = 414, description = "URI Too Long", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::URI_TOO_LONG.to_string(), "message": ""})),
(status = 415, description = "Unsupported Media Type", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::UNSUPPORTED_MEDIA_TYPE.to_string(), "message": ""})),
(status = 416, description = "Range Not Satisfiable", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::RANGE_NOT_SATISFIABLE.to_string(), "message": ""})),
(status = 417, description = "Expectation Failed", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::EXPECTATION_FAILED.to_string(), "message": ""})),
(status = 418, description = "I'm a teapot", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::IM_A_TEAPOT.to_string(), "message": ""})),
(status = 422, description = "Unprocessable Entity", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::UNPROCESSABLE_ENTITY.to_string(), "message": ""})),
(status = 425, description = "Too Early", content_type = "application/json", body = Response,
example = json!({"status": "425 Too Early", "message": ""})),
(status = 426, description = "Upgrade Required", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::UPGRADE_REQUIRED.to_string(), "message": ""})),
(status = 428, description = "Precondition Required", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::PRECONDITION_REQUIRED.to_string(), "message": ""})),
(status = 429, description = "Too Many Requests", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::TOO_MANY_REQUESTS.to_string(), "message": ""})),
(status = 431, description = "Request Header Fields Too Large", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::REQUEST_HEADER_FIELDS_TOO_LARGE.to_string(), "message": ""})),
(status = 451, description = "Unavailable For Legal Reasons", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::UNAVAILABLE_FOR_LEGAL_REASONS.to_string(), "message": ""})),
(status = 500, description = "Internal Server Error", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::INTERNAL_SERVER_ERROR.to_string(), "message": ""})),
(status = 501, description = "Not Implemented", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::NOT_IMPLEMENTED.to_string(), "message": ""})),
(status = 502, description = "Bad Gateway", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::BAD_GATEWAY.to_string(), "message": ""})),
(status = 503, description = "Service Unavailable", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::SERVICE_UNAVAILABLE.to_string(), "message": ""})),
(status = 504, description = "Gateway Timeout", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::GATEWAY_TIMEOUT.to_string(), "message": ""})),
(status = 505, description = "HTTP Version Not Supported", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::HTTP_VERSION_NOT_SUPPORTED.to_string(), "message": ""})),
(status = 506, description = "Variant Also Negotiates", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::VARIANT_ALSO_NEGOTIATES.to_string(), "message": ""})),
(status = 507, description = "Insufficient Storage", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::INSUFFICIENT_STORAGE.to_string(), "message": ""})),
(status = 508, description = "Loop Detected", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::LOOP_DETECTED.to_string(), "message": ""})),
(status = 510, description = "Not Extended", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::NOT_EXTENDED.to_string(), "message": ""})),
(status = 511, description = "Network Authentication Required", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::NETWORK_AUTHENTICATION_REQUIRED.to_string(), "message": ""}))
),
request_body = UserGetRequest,
params(
("Authorization", Header, description = "Authorization Token", example = "Bearer ..."),
("Content-Type", Header, description = "Content Type", example = "application/json"),
("X-API-Key", Header, description = "Current X-API-Key of user", example = "123ASD"),
),
tag = "User"
)]
#[post("/users")]
pub async fn create_user(data: web::Data<Arc<Client>>, _json: web::Json<model::UserPOSTRequest>) -> HttpResponse {
    // Access the database client from app_data
    let db_client = data.get_ref();

    let user_model = model::new("Steven".to_string(), "".to_string(), "".to_string(), "".to_string());
    let result = db_client.execute("INSERT INTO users (user_id, username, email, password_hash, full_name) VALUES ($1, $2, $3, $4, $5)",
                              &[&user_model.get_user_id(), &user_model.get_username(), &user_model.get_email(), &user_model.get_password_hash(), &user_model.get_full_name()]).await;

    match result {
        Ok(affected_rows) => {
            // Query was successful
            println!("Query executed successfully. Affected rows: {}", affected_rows);
            let response = response::JsonResponse::new(
                true,
                StatusCode::OK.as_u16(),
                Some(String::from("".to_string())),
                None);

            // Serialize the struct to a JSON string
            let json_data = match serde_json::to_string(&response) {
                Ok(json_str) => json_str,
                Err(err) => err.to_string(),

            };

            // Valid JSON data
            HttpResponse::Ok()
                .content_type(APPLICATION_JSON.to_string())
                .body(json_data)
        }
        Err(e) => {
            // Query failed
            eprintln!("Query error: {}", e);
            let response = response::JsonResponse{
                success: false,
                code: StatusCode::OK.as_u16(),
                message: Some(String::from(e.to_string())),
                data: None,
            };

            // Serialize the struct to a JSON string
            let json_data = match serde_json::to_string(&response) {
                Ok(json_str) => json_str,
                Err(err) => err.to_string(),

            };

            // Valid JSON data
            HttpResponse::InternalServerError()
                .content_type(APPLICATION_JSON.to_string())
                .body(json_data)
        }
    }
}

/// Update the user with specific user id
#[utoipa::path(
put,
path = "/users/{id}",
responses(
(status = 100, description = "Continue", content_type = "application/json", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::CONTINUE.to_string(), "message": ""})),
(status = 101, description = "Switching Protocols", content_type = "application/json", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::SWITCHING_PROTOCOLS.to_string(), "message": ""})),
(status = 103, description = "Early Hints", content_type = "application/json", content_type = "application/json", content_type = "application/json", body = Response,
example = json!({"status": "103 Early Hints".to_string(), "message": ""})),
(status = 200, description = "OK", content_type = "application/json", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::OK.to_string(), "message": ""})),
(status = 201, description = "Created", content_type = "application/json", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::CREATED.to_string(), "message": ""})),
(status = 202, description = "Accepted", content_type = "application/json", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::ACCEPTED.to_string(), "message": ""})),
(status = 203, description = "Non-Authoritative Information", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::NON_AUTHORITATIVE_INFORMATION.to_string(), "message": ""})),
(status = 204, description = "No Content", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::NO_CONTENT.to_string(), "message": ""})),
(status = 205, description = "Reset Content", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::RESET_CONTENT.to_string(), "message": ""})),
(status = 206, description = "Partial Content", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::PARTIAL_CONTENT.to_string(), "message": ""})),
(status = 300, description = "Multiple Choices", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::MULTIPLE_CHOICES.to_string(), "message": ""})),
(status = 301, description = "Moved Permanently", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::MOVED_PERMANENTLY.to_string(), "message": ""})),
(status = 302, description = "Found", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::FOUND.to_string(), "message": ""})),
(status = 303, description = "See Other", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::SEE_OTHER.to_string(), "message": ""})),
(status = 304, description = "Not Modified", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::NOT_MODIFIED.to_string(), "message": ""})),
(status = 307, description = "Temporary Redirect", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::TEMPORARY_REDIRECT.to_string(), "message": ""})),
(status = 308, description = "Permanent Redirect", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::PERMANENT_REDIRECT.to_string(), "message": ""})),
(status = 400, description = "Bad Request", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::BAD_REQUEST.to_string(), "message": ""})),
(status = 401, description = "Unauthorized", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::UNAUTHORIZED.to_string(), "message": ""})),
(status = 402, description = "Payment Required", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::PAYMENT_REQUIRED.to_string(), "message": ""})),
(status = 403, description = "Forbidden", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::FORBIDDEN.to_string(), "message": ""})),
(status = 404, description = "Not Found", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::NOT_FOUND.to_string(), "message": ""})),
(status = 405, description = "Method Not Allowed", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::METHOD_NOT_ALLOWED.to_string(), "message": ""})),
(status = 406, description = "Not Acceptable", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::NOT_ACCEPTABLE.to_string(), "message": ""})),
(status = 407, description = "Proxy Authentication Required", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::PROXY_AUTHENTICATION_REQUIRED.to_string(), "message": ""})),
(status = 408, description = "Request Timeout", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::REQUEST_TIMEOUT.to_string(), "message": ""})),
(status = 409, description = "Conflict", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::CONFLICT.to_string(), "message": ""})),
(status = 410, description = "Gone", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::GONE.to_string(), "message": ""})),
(status = 411, description = "Length Required", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::LENGTH_REQUIRED.to_string(), "message": ""})),
(status = 412, description = "Precondition Failed", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::PRECONDITION_FAILED.to_string(), "message": ""})),
(status = 413, description = "Payload Too Large", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::PAYLOAD_TOO_LARGE.to_string(), "message": ""})),
(status = 414, description = "URI Too Long", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::URI_TOO_LONG.to_string(), "message": ""})),
(status = 415, description = "Unsupported Media Type", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::UNSUPPORTED_MEDIA_TYPE.to_string(), "message": ""})),
(status = 416, description = "Range Not Satisfiable", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::RANGE_NOT_SATISFIABLE.to_string(), "message": ""})),
(status = 417, description = "Expectation Failed", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::EXPECTATION_FAILED.to_string(), "message": ""})),
(status = 418, description = "I'm a teapot", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::IM_A_TEAPOT.to_string(), "message": ""})),
(status = 422, description = "Unprocessable Entity", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::UNPROCESSABLE_ENTITY.to_string(), "message": ""})),
(status = 425, description = "Too Early", content_type = "application/json", body = Response,
example = json!({"status": "425 Too Early", "message": ""})),
(status = 426, description = "Upgrade Required", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::UPGRADE_REQUIRED.to_string(), "message": ""})),
(status = 428, description = "Precondition Required", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::PRECONDITION_REQUIRED.to_string(), "message": ""})),
(status = 429, description = "Too Many Requests", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::TOO_MANY_REQUESTS.to_string(), "message": ""})),
(status = 431, description = "Request Header Fields Too Large", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::REQUEST_HEADER_FIELDS_TOO_LARGE.to_string(), "message": ""})),
(status = 451, description = "Unavailable For Legal Reasons", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::UNAVAILABLE_FOR_LEGAL_REASONS.to_string(), "message": ""})),
(status = 500, description = "Internal Server Error", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::INTERNAL_SERVER_ERROR.to_string(), "message": ""})),
(status = 501, description = "Not Implemented", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::NOT_IMPLEMENTED.to_string(), "message": ""})),
(status = 502, description = "Bad Gateway", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::BAD_GATEWAY.to_string(), "message": ""})),
(status = 503, description = "Service Unavailable", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::SERVICE_UNAVAILABLE.to_string(), "message": ""})),
(status = 504, description = "Gateway Timeout", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::GATEWAY_TIMEOUT.to_string(), "message": ""})),
(status = 505, description = "HTTP Version Not Supported", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::HTTP_VERSION_NOT_SUPPORTED.to_string(), "message": ""})),
(status = 506, description = "Variant Also Negotiates", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::VARIANT_ALSO_NEGOTIATES.to_string(), "message": ""})),
(status = 507, description = "Insufficient Storage", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::INSUFFICIENT_STORAGE.to_string(), "message": ""})),
(status = 508, description = "Loop Detected", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::LOOP_DETECTED.to_string(), "message": ""})),
(status = 510, description = "Not Extended", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::NOT_EXTENDED.to_string(), "message": ""})),
(status = 511, description = "Network Authentication Required", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::NETWORK_AUTHENTICATION_REQUIRED.to_string(), "message": ""}))
),
request_body = UserGetRequest,
params(
("Authorization", Header, description = "Authorization Token", example = "Bearer ..."),
("Content-Type", Header, description = "Content Type", example = "application/json"),
("X-API-Key", Header, description = "Current X-API-Key of user", example = "123ASD"),
),
tag = "User"
)]
#[put("/users")]
pub async fn update_user(data: web::Data<Arc<Client>>, _json: web::Json<model::UserPUTRequest>) -> HttpResponse {
    // Access the database client from app_data
    let db_client = data.get_ref();

    let user_model = model::new("Steven".to_string(), "".to_string(), "".to_string(), "".to_string());
    let result = db_client.execute("UPDATE users SET username = $1, email = $2, password_hash = $3, full_name = $4 WHERE user_id = $5",
                                   &[&user_model.get_username(), &user_model.get_email(), &user_model.get_password_hash(), &user_model.get_full_name(), &user_model.get_user_id()]).await;

    match result {
        Ok(affected_rows) => {
            // Query was successful
            println!("Query executed successfully. Affected rows: {}", affected_rows);
            let response = response::JsonResponse{
                success: true,
                code: StatusCode::OK.as_u16(),
                message: Some(String::from("".to_string())),
                data: None,
            };

            // Serialize the struct to a JSON string
            let json_data = match serde_json::to_string(&response) {
                Ok(json_str) => json_str,
                Err(err) => err.to_string(),

            };

            // Valid JSON data
            HttpResponse::Ok()
                .content_type(APPLICATION_JSON.to_string())
                .body(json_data)
        }
        Err(e) => {
            // Query failed
            eprintln!("Query error: {}", e);
            let response = response::JsonResponse{
                success: false,
                code: StatusCode::OK.as_u16(),
                message: Some(String::from(e.to_string())),
                data: None,
            };

            // Serialize the struct to a JSON string
            let json_data = match serde_json::to_string(&response) {
                Ok(json_str) => json_str,
                Err(err) => err.to_string(),

            };

            // Valid JSON data
            HttpResponse::InternalServerError()
                .content_type(APPLICATION_JSON.to_string())
                .body(json_data)
        }
    }
}

/// Delete the user with specific user id
#[utoipa::path(
delete,
path = "/users/{id}",
responses(
(status = 100, description = "Continue", content_type = "application/json", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::CONTINUE.to_string(), "message": ""})),
(status = 101, description = "Switching Protocols", content_type = "application/json", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::SWITCHING_PROTOCOLS.to_string(), "message": ""})),
(status = 103, description = "Early Hints", content_type = "application/json", content_type = "application/json", content_type = "application/json", body = Response,
example = json!({"status": "103 Early Hints".to_string(), "message": ""})),
(status = 200, description = "OK", content_type = "application/json", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::OK.to_string(), "message": ""})),
(status = 201, description = "Created", content_type = "application/json", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::CREATED.to_string(), "message": ""})),
(status = 202, description = "Accepted", content_type = "application/json", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::ACCEPTED.to_string(), "message": ""})),
(status = 203, description = "Non-Authoritative Information", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::NON_AUTHORITATIVE_INFORMATION.to_string(), "message": ""})),
(status = 204, description = "No Content", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::NO_CONTENT.to_string(), "message": ""})),
(status = 205, description = "Reset Content", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::RESET_CONTENT.to_string(), "message": ""})),
(status = 206, description = "Partial Content", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::PARTIAL_CONTENT.to_string(), "message": ""})),
(status = 300, description = "Multiple Choices", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::MULTIPLE_CHOICES.to_string(), "message": ""})),
(status = 301, description = "Moved Permanently", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::MOVED_PERMANENTLY.to_string(), "message": ""})),
(status = 302, description = "Found", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::FOUND.to_string(), "message": ""})),
(status = 303, description = "See Other", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::SEE_OTHER.to_string(), "message": ""})),
(status = 304, description = "Not Modified", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::NOT_MODIFIED.to_string(), "message": ""})),
(status = 307, description = "Temporary Redirect", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::TEMPORARY_REDIRECT.to_string(), "message": ""})),
(status = 308, description = "Permanent Redirect", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::PERMANENT_REDIRECT.to_string(), "message": ""})),
(status = 400, description = "Bad Request", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::BAD_REQUEST.to_string(), "message": ""})),
(status = 401, description = "Unauthorized", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::UNAUTHORIZED.to_string(), "message": ""})),
(status = 402, description = "Payment Required", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::PAYMENT_REQUIRED.to_string(), "message": ""})),
(status = 403, description = "Forbidden", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::FORBIDDEN.to_string(), "message": ""})),
(status = 404, description = "Not Found", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::NOT_FOUND.to_string(), "message": ""})),
(status = 405, description = "Method Not Allowed", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::METHOD_NOT_ALLOWED.to_string(), "message": ""})),
(status = 406, description = "Not Acceptable", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::NOT_ACCEPTABLE.to_string(), "message": ""})),
(status = 407, description = "Proxy Authentication Required", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::PROXY_AUTHENTICATION_REQUIRED.to_string(), "message": ""})),
(status = 408, description = "Request Timeout", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::REQUEST_TIMEOUT.to_string(), "message": ""})),
(status = 409, description = "Conflict", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::CONFLICT.to_string(), "message": ""})),
(status = 410, description = "Gone", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::GONE.to_string(), "message": ""})),
(status = 411, description = "Length Required", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::LENGTH_REQUIRED.to_string(), "message": ""})),
(status = 412, description = "Precondition Failed", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::PRECONDITION_FAILED.to_string(), "message": ""})),
(status = 413, description = "Payload Too Large", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::PAYLOAD_TOO_LARGE.to_string(), "message": ""})),
(status = 414, description = "URI Too Long", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::URI_TOO_LONG.to_string(), "message": ""})),
(status = 415, description = "Unsupported Media Type", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::UNSUPPORTED_MEDIA_TYPE.to_string(), "message": ""})),
(status = 416, description = "Range Not Satisfiable", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::RANGE_NOT_SATISFIABLE.to_string(), "message": ""})),
(status = 417, description = "Expectation Failed", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::EXPECTATION_FAILED.to_string(), "message": ""})),
(status = 418, description = "I'm a teapot", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::IM_A_TEAPOT.to_string(), "message": ""})),
(status = 422, description = "Unprocessable Entity", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::UNPROCESSABLE_ENTITY.to_string(), "message": ""})),
(status = 425, description = "Too Early", content_type = "application/json", body = Response,
example = json!({"status": "425 Too Early", "message": ""})),
(status = 426, description = "Upgrade Required", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::UPGRADE_REQUIRED.to_string(), "message": ""})),
(status = 428, description = "Precondition Required", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::PRECONDITION_REQUIRED.to_string(), "message": ""})),
(status = 429, description = "Too Many Requests", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::TOO_MANY_REQUESTS.to_string(), "message": ""})),
(status = 431, description = "Request Header Fields Too Large", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::REQUEST_HEADER_FIELDS_TOO_LARGE.to_string(), "message": ""})),
(status = 451, description = "Unavailable For Legal Reasons", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::UNAVAILABLE_FOR_LEGAL_REASONS.to_string(), "message": ""})),
(status = 500, description = "Internal Server Error", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::INTERNAL_SERVER_ERROR.to_string(), "message": ""})),
(status = 501, description = "Not Implemented", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::NOT_IMPLEMENTED.to_string(), "message": ""})),
(status = 502, description = "Bad Gateway", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::BAD_GATEWAY.to_string(), "message": ""})),
(status = 503, description = "Service Unavailable", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::SERVICE_UNAVAILABLE.to_string(), "message": ""})),
(status = 504, description = "Gateway Timeout", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::GATEWAY_TIMEOUT.to_string(), "message": ""})),
(status = 505, description = "HTTP Version Not Supported", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::HTTP_VERSION_NOT_SUPPORTED.to_string(), "message": ""})),
(status = 506, description = "Variant Also Negotiates", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::VARIANT_ALSO_NEGOTIATES.to_string(), "message": ""})),
(status = 507, description = "Insufficient Storage", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::INSUFFICIENT_STORAGE.to_string(), "message": ""})),
(status = 508, description = "Loop Detected", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::LOOP_DETECTED.to_string(), "message": ""})),
(status = 510, description = "Not Extended", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::NOT_EXTENDED.to_string(), "message": ""})),
(status = 511, description = "Network Authentication Required", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::NETWORK_AUTHENTICATION_REQUIRED.to_string(), "message": ""}))
),
params(
("id" = Uuid, Path, description = "User ID", example = "184e71b7-7f6f-43be-9f79-6e97d5d72d87"),
("Authorization", Header, description = "Authorization Token", example = "Bearer ..."),
("Content-Type", Header, description = "Content Type", example = "application/json"),
("X-API-Key", Header, description = "Current X-API-Key of user", example = "123ASD"),
),
tag = "User"
)]
#[delete("/users/{id}")]
pub async fn delete_user(data: web::Data<Arc<Client>>, path_params: web::Path<Uuid>) -> HttpResponse {
    let user_id: Uuid = path_params.into_inner();
    let db_client = data.get_ref();
    let select_query = "SELECT deleted_at FROM users WHERE user_id = $1";
    let select_statement: Statement = match db_client.prepare_typed(select_query, &[Type::UUID]).await {
        Ok(statement) => statement,
        Err(err) => {
            eprintln!("Error preparing statement: {}", err);
            // return Ok(HttpResponse::InternalServerError().finish());
            return HttpResponse::InternalServerError()
                .content_type(APPLICATION_JSON.to_string())
                .body(err.to_string())
        }
    };
    let rows = match db_client.query(&select_statement, &[&user_id]).await {
        Ok(result) => result,
        Err(err) => {
            eprintln!("Error executing query: {}", err);
            return HttpResponse::InternalServerError()
                .content_type(APPLICATION_JSON.to_string())
                .body(err.to_string())
        }
    };
    println!("rows {}", rows.len());
    if rows.len() >= 1 {
        return HttpResponse::BadRequest()
                .content_type(APPLICATION_JSON.to_string())
                .body("")
    }

    // let search_result = db_client.execute("SELECT deleted_at FROM users WHERE user_id = $1",
    //                                &[&user_id]).await;
    // println!("{}", search_result.to_string());

    let delete_result = db_client.execute("UPDATE users SET deleted_at = now(), deleted_by = $1 WHERE user_id = $2",
                                   &[&user_id, &user_id]).await;

    match delete_result {
        Ok(affected_rows) => {
            // Query was successful
            println!("Query executed successfully. Affected rows: {}", affected_rows);
            let response = response::JsonResponse{
                success: true,
                code: StatusCode::OK.as_u16(),
                message: Some(String::from("".to_string())),
                data: None,
            };

            // Serialize the struct to a JSON string
            let json_data = match serde_json::to_string(&response) {
                Ok(json_str) => json_str,
                Err(err) => err.to_string(),

            };

            // Valid JSON data
            HttpResponse::Ok()
                .content_type(APPLICATION_JSON.to_string())
                .body(json_data)
        }
        Err(e) => {
            // Query failed
            eprintln!("Query error: {}", e);
            let response = response::JsonResponse{
                success: false,
                code: StatusCode::OK.as_u16(),
                message: Some(String::from(e.to_string())),
                data: None,
            };

            // Serialize the struct to a JSON string
            let json_data = match serde_json::to_string(&response) {
                Ok(json_str) => json_str,
                Err(err) => err.to_string(),

            };

            // Valid JSON data
            HttpResponse::InternalServerError()
                .content_type(APPLICATION_JSON.to_string())
                .body(json_data)
        }
    }
}

/// Get all users
#[utoipa::path(
get,
path = "/users/{id}",
responses(
(status = 100, description = "Continue", content_type = "application/json", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::CONTINUE.to_string(), "message": ""})),
(status = 101, description = "Switching Protocols", content_type = "application/json", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::SWITCHING_PROTOCOLS.to_string(), "message": ""})),
(status = 103, description = "Early Hints", content_type = "application/json", content_type = "application/json", content_type = "application/json", body = Response,
example = json!({"status": "103 Early Hints".to_string(), "message": ""})),
(status = 200, description = "OK", content_type = "application/json", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::OK.to_string(), "message": ""})),
(status = 201, description = "Created", content_type = "application/json", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::CREATED.to_string(), "message": ""})),
(status = 202, description = "Accepted", content_type = "application/json", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::ACCEPTED.to_string(), "message": ""})),
(status = 203, description = "Non-Authoritative Information", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::NON_AUTHORITATIVE_INFORMATION.to_string(), "message": ""})),
(status = 204, description = "No Content", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::NO_CONTENT.to_string(), "message": ""})),
(status = 205, description = "Reset Content", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::RESET_CONTENT.to_string(), "message": ""})),
(status = 206, description = "Partial Content", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::PARTIAL_CONTENT.to_string(), "message": ""})),
(status = 300, description = "Multiple Choices", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::MULTIPLE_CHOICES.to_string(), "message": ""})),
(status = 301, description = "Moved Permanently", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::MOVED_PERMANENTLY.to_string(), "message": ""})),
(status = 302, description = "Found", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::FOUND.to_string(), "message": ""})),
(status = 303, description = "See Other", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::SEE_OTHER.to_string(), "message": ""})),
(status = 304, description = "Not Modified", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::NOT_MODIFIED.to_string(), "message": ""})),
(status = 307, description = "Temporary Redirect", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::TEMPORARY_REDIRECT.to_string(), "message": ""})),
(status = 308, description = "Permanent Redirect", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::PERMANENT_REDIRECT.to_string(), "message": ""})),
(status = 400, description = "Bad Request", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::BAD_REQUEST.to_string(), "message": ""})),
(status = 401, description = "Unauthorized", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::UNAUTHORIZED.to_string(), "message": ""})),
(status = 402, description = "Payment Required", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::PAYMENT_REQUIRED.to_string(), "message": ""})),
(status = 403, description = "Forbidden", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::FORBIDDEN.to_string(), "message": ""})),
(status = 404, description = "Not Found", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::NOT_FOUND.to_string(), "message": ""})),
(status = 405, description = "Method Not Allowed", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::METHOD_NOT_ALLOWED.to_string(), "message": ""})),
(status = 406, description = "Not Acceptable", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::NOT_ACCEPTABLE.to_string(), "message": ""})),
(status = 407, description = "Proxy Authentication Required", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::PROXY_AUTHENTICATION_REQUIRED.to_string(), "message": ""})),
(status = 408, description = "Request Timeout", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::REQUEST_TIMEOUT.to_string(), "message": ""})),
(status = 409, description = "Conflict", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::CONFLICT.to_string(), "message": ""})),
(status = 410, description = "Gone", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::GONE.to_string(), "message": ""})),
(status = 411, description = "Length Required", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::LENGTH_REQUIRED.to_string(), "message": ""})),
(status = 412, description = "Precondition Failed", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::PRECONDITION_FAILED.to_string(), "message": ""})),
(status = 413, description = "Payload Too Large", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::PAYLOAD_TOO_LARGE.to_string(), "message": ""})),
(status = 414, description = "URI Too Long", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::URI_TOO_LONG.to_string(), "message": ""})),
(status = 415, description = "Unsupported Media Type", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::UNSUPPORTED_MEDIA_TYPE.to_string(), "message": ""})),
(status = 416, description = "Range Not Satisfiable", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::RANGE_NOT_SATISFIABLE.to_string(), "message": ""})),
(status = 417, description = "Expectation Failed", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::EXPECTATION_FAILED.to_string(), "message": ""})),
(status = 418, description = "I'm a teapot", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::IM_A_TEAPOT.to_string(), "message": ""})),
(status = 422, description = "Unprocessable Entity", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::UNPROCESSABLE_ENTITY.to_string(), "message": ""})),
(status = 425, description = "Too Early", content_type = "application/json", body = Response,
example = json!({"status": "425 Too Early", "message": ""})),
(status = 426, description = "Upgrade Required", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::UPGRADE_REQUIRED.to_string(), "message": ""})),
(status = 428, description = "Precondition Required", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::PRECONDITION_REQUIRED.to_string(), "message": ""})),
(status = 429, description = "Too Many Requests", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::TOO_MANY_REQUESTS.to_string(), "message": ""})),
(status = 431, description = "Request Header Fields Too Large", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::REQUEST_HEADER_FIELDS_TOO_LARGE.to_string(), "message": ""})),
(status = 451, description = "Unavailable For Legal Reasons", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::UNAVAILABLE_FOR_LEGAL_REASONS.to_string(), "message": ""})),
(status = 500, description = "Internal Server Error", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::INTERNAL_SERVER_ERROR.to_string(), "message": ""})),
(status = 501, description = "Not Implemented", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::NOT_IMPLEMENTED.to_string(), "message": ""})),
(status = 502, description = "Bad Gateway", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::BAD_GATEWAY.to_string(), "message": ""})),
(status = 503, description = "Service Unavailable", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::SERVICE_UNAVAILABLE.to_string(), "message": ""})),
(status = 504, description = "Gateway Timeout", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::GATEWAY_TIMEOUT.to_string(), "message": ""})),
(status = 505, description = "HTTP Version Not Supported", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::HTTP_VERSION_NOT_SUPPORTED.to_string(), "message": ""})),
(status = 506, description = "Variant Also Negotiates", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::VARIANT_ALSO_NEGOTIATES.to_string(), "message": ""})),
(status = 507, description = "Insufficient Storage", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::INSUFFICIENT_STORAGE.to_string(), "message": ""})),
(status = 508, description = "Loop Detected", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::LOOP_DETECTED.to_string(), "message": ""})),
(status = 510, description = "Not Extended", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::NOT_EXTENDED.to_string(), "message": ""})),
(status = 511, description = "Network Authentication Required", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::NETWORK_AUTHENTICATION_REQUIRED.to_string(), "message": ""}))
),
request_body = UserGetRequest,
params(
("Authorization", Header, description = "Authorization Token", example = "Bearer ..."),
("X-API-Key", Header, description = "Current X-API-Key of user", example = "123ASD"),
),
tag = "User"
)]
#[get("/users")]
pub async fn select_user(data: web::Data<Arc<Client>>) -> HttpResponse {
    let db_client = data.get_ref();
    // Define your SQL query
    let query = "SELECT user_id, username, email, full_name FROM users ORDER BY user_id ASC";

    // Execute the query and collect the results
    let rows = db_client.query(query, &[]).await.expect("Failed to execute query");

    let mut data = Vec::new();
    let _total_items = rows.len() as u16;

    // Process the rows
    for row in rows {
        let user_id: Uuid = row.get("user_id");
        let username: String = row.get("username");
        let email: String = row.get("email");
        let full_name: String = row.get("full_name");

        data.push(model::new_user(user_id, username, email, full_name));
        println!("loop");
    }
    // Process the rows
    // for row in rows {
    //     process_row(&row);
    //     let user_id: Uuid = row.get("user_id");
    //     let username: String = row.get("username");
    //     let email: String = row.get("email");
    //     let full_name: String = row.get("full_name");
    //     println!("User ID: {}, Username: {}, Email: {}", user_id, username, email);
    //
    //     model::new_user(user_id, username, email, full_name);
    // }

    let response = response::JsonResponse{
        success: true,
        code: StatusCode::OK.as_u16(),
        message: Some(String::from("".to_string())),
        data: Some(serde_json::to_string(&data).unwrap()),
    };

    HttpResponse::Ok().json(response)
}

/// Get the user with specific user id
#[utoipa::path(
    get,
    path = "/users/{id}",
    responses(
    (status = 100, description = "Continue", content_type = "application/json", content_type = "application/json", body = Response,
    example = json!({"status": StatusCode::CONTINUE.to_string(), "message": ""})),
    (status = 101, description = "Switching Protocols", content_type = "application/json", content_type = "application/json", body = Response,
    example = json!({"status": StatusCode::SWITCHING_PROTOCOLS.to_string(), "message": ""})),
    (status = 103, description = "Early Hints", content_type = "application/json", content_type = "application/json", content_type = "application/json", body = Response,
    example = json!({"status": "103 Early Hints".to_string(), "message": ""})),
    (status = 200, description = "OK", content_type = "application/json", content_type = "application/json", body = Response,
    example = json!({"status": StatusCode::OK.to_string(), "message": ""})),
    (status = 201, description = "Created", content_type = "application/json", content_type = "application/json", body = Response,
    example = json!({"status": StatusCode::CREATED.to_string(), "message": ""})),
    (status = 202, description = "Accepted", content_type = "application/json", content_type = "application/json", body = Response,
    example = json!({"status": StatusCode::ACCEPTED.to_string(), "message": ""})),
    (status = 203, description = "Non-Authoritative Information", content_type = "application/json", body = Response,
    example = json!({"status": StatusCode::NON_AUTHORITATIVE_INFORMATION.to_string(), "message": ""})),
    (status = 204, description = "No Content", content_type = "application/json", body = Response,
    example = json!({"status": StatusCode::NO_CONTENT.to_string(), "message": ""})),
    (status = 205, description = "Reset Content", content_type = "application/json", body = Response,
    example = json!({"status": StatusCode::RESET_CONTENT.to_string(), "message": ""})),
    (status = 206, description = "Partial Content", content_type = "application/json", body = Response,
    example = json!({"status": StatusCode::PARTIAL_CONTENT.to_string(), "message": ""})),
    (status = 300, description = "Multiple Choices", content_type = "application/json", body = Response,
    example = json!({"status": StatusCode::MULTIPLE_CHOICES.to_string(), "message": ""})),
    (status = 301, description = "Moved Permanently", content_type = "application/json", body = Response,
    example = json!({"status": StatusCode::MOVED_PERMANENTLY.to_string(), "message": ""})),
    (status = 302, description = "Found", content_type = "application/json", body = Response,
    example = json!({"status": StatusCode::FOUND.to_string(), "message": ""})),
    (status = 303, description = "See Other", content_type = "application/json", body = Response,
    example = json!({"status": StatusCode::SEE_OTHER.to_string(), "message": ""})),
    (status = 304, description = "Not Modified", content_type = "application/json", body = Response,
    example = json!({"status": StatusCode::NOT_MODIFIED.to_string(), "message": ""})),
    (status = 307, description = "Temporary Redirect", content_type = "application/json", body = Response,
    example = json!({"status": StatusCode::TEMPORARY_REDIRECT.to_string(), "message": ""})),
    (status = 308, description = "Permanent Redirect", content_type = "application/json", body = Response,
    example = json!({"status": StatusCode::PERMANENT_REDIRECT.to_string(), "message": ""})),
    (status = 400, description = "Bad Request", content_type = "application/json", body = Response,
    example = json!({"status": StatusCode::BAD_REQUEST.to_string(), "message": ""})),
    (status = 401, description = "Unauthorized", content_type = "application/json", body = Response,
    example = json!({"status": StatusCode::UNAUTHORIZED.to_string(), "message": ""})),
    (status = 402, description = "Payment Required", content_type = "application/json", body = Response,
    example = json!({"status": StatusCode::PAYMENT_REQUIRED.to_string(), "message": ""})),
    (status = 403, description = "Forbidden", content_type = "application/json", body = Response,
    example = json!({"status": StatusCode::FORBIDDEN.to_string(), "message": ""})),
    (status = 404, description = "Not Found", content_type = "application/json", body = Response,
    example = json!({"status": StatusCode::NOT_FOUND.to_string(), "message": ""})),
    (status = 405, description = "Method Not Allowed", content_type = "application/json", body = Response,
    example = json!({"status": StatusCode::METHOD_NOT_ALLOWED.to_string(), "message": ""})),
    (status = 406, description = "Not Acceptable", content_type = "application/json", body = Response,
    example = json!({"status": StatusCode::NOT_ACCEPTABLE.to_string(), "message": ""})),
    (status = 407, description = "Proxy Authentication Required", content_type = "application/json", body = Response,
    example = json!({"status": StatusCode::PROXY_AUTHENTICATION_REQUIRED.to_string(), "message": ""})),
    (status = 408, description = "Request Timeout", content_type = "application/json", body = Response,
    example = json!({"status": StatusCode::REQUEST_TIMEOUT.to_string(), "message": ""})),
    (status = 409, description = "Conflict", content_type = "application/json", body = Response,
    example = json!({"status": StatusCode::CONFLICT.to_string(), "message": ""})),
    (status = 410, description = "Gone", content_type = "application/json", body = Response,
    example = json!({"status": StatusCode::GONE.to_string(), "message": ""})),
    (status = 411, description = "Length Required", content_type = "application/json", body = Response,
    example = json!({"status": StatusCode::LENGTH_REQUIRED.to_string(), "message": ""})),
    (status = 412, description = "Precondition Failed", content_type = "application/json", body = Response,
    example = json!({"status": StatusCode::PRECONDITION_FAILED.to_string(), "message": ""})),
    (status = 413, description = "Payload Too Large", content_type = "application/json", body = Response,
    example = json!({"status": StatusCode::PAYLOAD_TOO_LARGE.to_string(), "message": ""})),
    (status = 414, description = "URI Too Long", content_type = "application/json", body = Response,
    example = json!({"status": StatusCode::URI_TOO_LONG.to_string(), "message": ""})),
    (status = 415, description = "Unsupported Media Type", content_type = "application/json", body = Response,
    example = json!({"status": StatusCode::UNSUPPORTED_MEDIA_TYPE.to_string(), "message": ""})),
    (status = 416, description = "Range Not Satisfiable", content_type = "application/json", body = Response,
    example = json!({"status": StatusCode::RANGE_NOT_SATISFIABLE.to_string(), "message": ""})),
    (status = 417, description = "Expectation Failed", content_type = "application/json", body = Response,
    example = json!({"status": StatusCode::EXPECTATION_FAILED.to_string(), "message": ""})),
    (status = 418, description = "I'm a teapot", content_type = "application/json", body = Response,
    example = json!({"status": StatusCode::IM_A_TEAPOT.to_string(), "message": ""})),
    (status = 422, description = "Unprocessable Entity", content_type = "application/json", body = Response,
    example = json!({"status": StatusCode::UNPROCESSABLE_ENTITY.to_string(), "message": ""})),
    (status = 425, description = "Too Early", content_type = "application/json", body = Response,
    example = json!({"status": "425 Too Early", "message": ""})),
    (status = 426, description = "Upgrade Required", content_type = "application/json", body = Response,
    example = json!({"status": StatusCode::UPGRADE_REQUIRED.to_string(), "message": ""})),
    (status = 428, description = "Precondition Required", content_type = "application/json", body = Response,
    example = json!({"status": StatusCode::PRECONDITION_REQUIRED.to_string(), "message": ""})),
    (status = 429, description = "Too Many Requests", content_type = "application/json", body = Response,
    example = json!({"status": StatusCode::TOO_MANY_REQUESTS.to_string(), "message": ""})),
    (status = 431, description = "Request Header Fields Too Large", content_type = "application/json", body = Response,
    example = json!({"status": StatusCode::REQUEST_HEADER_FIELDS_TOO_LARGE.to_string(), "message": ""})),
    (status = 451, description = "Unavailable For Legal Reasons", content_type = "application/json", body = Response,
    example = json!({"status": StatusCode::UNAVAILABLE_FOR_LEGAL_REASONS.to_string(), "message": ""})),
    (status = 500, description = "Internal Server Error", content_type = "application/json", body = Response,
    example = json!({"status": StatusCode::INTERNAL_SERVER_ERROR.to_string(), "message": ""})),
    (status = 501, description = "Not Implemented", content_type = "application/json", body = Response,
    example = json!({"status": StatusCode::NOT_IMPLEMENTED.to_string(), "message": ""})),
    (status = 502, description = "Bad Gateway", content_type = "application/json", body = Response,
    example = json!({"status": StatusCode::BAD_GATEWAY.to_string(), "message": ""})),
    (status = 503, description = "Service Unavailable", content_type = "application/json", body = Response,
    example = json!({"status": StatusCode::SERVICE_UNAVAILABLE.to_string(), "message": ""})),
    (status = 504, description = "Gateway Timeout", content_type = "application/json", body = Response,
    example = json!({"status": StatusCode::GATEWAY_TIMEOUT.to_string(), "message": ""})),
    (status = 505, description = "HTTP Version Not Supported", content_type = "application/json", body = Response,
    example = json!({"status": StatusCode::HTTP_VERSION_NOT_SUPPORTED.to_string(), "message": ""})),
    (status = 506, description = "Variant Also Negotiates", content_type = "application/json", body = Response,
    example = json!({"status": StatusCode::VARIANT_ALSO_NEGOTIATES.to_string(), "message": ""})),
    (status = 507, description = "Insufficient Storage", content_type = "application/json", body = Response,
    example = json!({"status": StatusCode::INSUFFICIENT_STORAGE.to_string(), "message": ""})),
    (status = 508, description = "Loop Detected", content_type = "application/json", body = Response,
    example = json!({"status": StatusCode::LOOP_DETECTED.to_string(), "message": ""})),
    (status = 510, description = "Not Extended", content_type = "application/json", body = Response,
    example = json!({"status": StatusCode::NOT_EXTENDED.to_string(), "message": ""})),
    (status = 511, description = "Network Authentication Required", content_type = "application/json", body = Response,
    example = json!({"status": StatusCode::NETWORK_AUTHENTICATION_REQUIRED.to_string(), "message": ""}))
    ),
    request_body = UserGetRequest,
    params(
    ("id" = String, Path, description = "User ID", example = "123"),
    ("Authorization", Header, description = "Authorization Token", example = "Bearer ..."),
    ("X-API-Key", Header, description = "Current X-API-Key of user", example = "123ASD"),
    ),
    tag = "User"
    )]
    #[get("/users/{id}")]
    pub async fn select_user_by_id(data: web::Data<Arc<Client>>) -> HttpResponse {
        let db_client = data.get_ref();
        // Define your SQL query
        let query = "SELECT user_id, username, email, full_name FROM users ORDER BY user_id ASC";
    
        // Execute the query and collect the results
        let rows = db_client.query(query, &[]).await.expect("Failed to execute query");
    
        let mut data = Vec::new();
        let _total_items = rows.len() as u16;
    
        // Process the rows
        for row in rows {
            let user_id: Uuid = row.get("user_id");
            let username: String = row.get("username");
            let email: String = row.get("email");
            let full_name: String = row.get("full_name");
    
            data.push(model::new_user(user_id, username, email, full_name));
            println!("loop");
        }
        // Process the rows
        // for row in rows {
        //     process_row(&row);
        //     let user_id: Uuid = row.get("user_id");
        //     let username: String = row.get("username");
        //     let email: String = row.get("email");
        //     let full_name: String = row.get("full_name");
        //     println!("User ID: {}, Username: {}, Email: {}", user_id, username, email);
        //
        //     model::new_user(user_id, username, email, full_name);
        // }
    
        let response = response::JsonResponse{
            success: true,
            code: StatusCode::OK.as_u16(),
            message: Some(String::from("".to_string())),
            data: Some(serde_json::to_string(&data).unwrap()),
        };
    
        HttpResponse::Ok().json(response)
    }

// Example function to process a row and extract data
fn _process_row(row: &Row) {
    // Extract data from the row based on column names
    let _user_id: Uuid = row.get("user_id");
    let _username: String = row.get("username");
    let _email: String = row.get("email");

    // println!("user_id : {:?}", user_id);
    // Process the extracted data as needed
    // println!("User ID: {}, Username: {}, Email: {}", user_id, username, email);
}