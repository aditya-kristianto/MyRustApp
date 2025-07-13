use actix::{Actor, ActorContext, StreamHandler};
use actix_http::StatusCode;
use actix_multipart::Multipart;
use actix_web::web::ServiceConfig;
use actix_web::{delete, get, post, Result, web};
use actix_web::{Error, HttpRequest, HttpResponse, Responder, ResponseError};
use actix_web_actors::ws;
use calamine::{open_workbook, DataType, Reader, Xlsx};
use chrono::format::ParseError as ChronoParseError;
use futures_util::stream::StreamExt;
use regex::Regex;
use serde::Serialize;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::fs::File;
use std::io::Write;
use std::primitive::i32;
use std::sync::Arc;
use std::{fmt, fs};
use super::repository::BearishStock;
use super::repository::BullishStock;
use super::repository::CustomNaiveDate;
use super::repository::FrequencyStock;
use super::repository::MACDStock;
use super::repository::QueryParams;
// use super::repository::StockInfo;
use super::repository::StockSMA;
use super::repository::DataValue;
use super::repository::Header;
use super::repository::Meta;
use super::repository::Response;
use super::repository::RSIStock;
use super::repository::Stock::Stock;
use super::repository::SummaryStock;
use tokio_postgres::Client;

#[derive(Serialize)]
struct Message {
    msg_type: String,
    content: String,
}

struct MyWebSocketActor;

impl Actor for MyWebSocketActor {
    type Context = ws::WebsocketContext<Self>;

    // Called when the WebSocket connection is established
    fn started(&mut self, ctx: &mut Self::Context) {
        // Example JSON message
        let response = Message {
            msg_type: "greeting".to_string(),
            content: "Welcome to the WebSocket server!".to_string(),
        };

        // Serialize the struct to JSON and send it
        if let Ok(json) = serde_json::to_string(&response) {
            ctx.text(json);
        }
    }
}

pub fn configure_v1(config: &mut ServiceConfig) {
    config
        .route("/ws", web::get().to(websocket_handler))
        .service(add_pemantauan_khusus_stock)
        .service(add_stock)
        .service(delete_stock)
        .service(get_bearish_stock)
        .service(get_bullish_stock)
        .service(get_macd_stock)
        .service(get_pemantauan_khusus_stock)
        .service(get_rsi_stock)
        .service(get_sma_stock)
        .service(get_stocks)
        .service(get_stock_rank_by_frequency)
        .service(get_stock_summary);
}

// Define a custom error type for parsing errors
#[derive(Debug)]
pub enum ParseDateError {
    NotEnough,
    Impossible,
    OutOfRange,
    #[allow(dead_code)]
    Chrono(ChronoParseError),
}

// Implement the ResponseError trait for your custom error type
impl ResponseError for ParseDateError {
    fn error_response(&self) -> HttpResponse {
        // You can return an appropriate HTTP response based on the error
        match self {
            ParseDateError::NotEnough => HttpResponse::InternalServerError()
                .body("Failed to parse, date not enough components to parse"),
            ParseDateError::Impossible => {
                HttpResponse::InternalServerError().body("Failed to parse, date is impossible")
            }
            ParseDateError::OutOfRange => {
                HttpResponse::InternalServerError().body("Failed to parse, date is out of range")
            }
            ParseDateError::Chrono(_e) => {
                HttpResponse::InternalServerError().body("Failed to parse date")
            }
        }
    }
}

// Implement the Display trait for your custom error type for better error messages
impl fmt::Display for ParseDateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseDateError::NotEnough => write!(f, "Not enough components to parse"),
            ParseDateError::Impossible => write!(f, "Parsing date is impossible"),
            ParseDateError::OutOfRange => write!(f, "Date is out of range"),
            ParseDateError::Chrono(e) => write!(f, "Chrono parsing error: {}", e),
        }
    }
}

// Handle WebSocket messages
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWebSocketActor {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Text(text)) => {
                println!("Received: {}", text);

                // Respond with a JSON message
                let response = Message {
                    msg_type: "echo".to_string(),
                    content: text.to_string(),
                };

                if let Ok(json) = serde_json::to_string(&response) {
                    ctx.text(json);
                }
            }
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Close(reason)) => {
                println!("Client disconnected: {:?}", reason);
                ctx.stop();
            }
            _ => (),
        }
    }
}

// Define a placeholder path for the WebSocket
#[utoipa::path(
    get,
    path = "/ws",
    responses(
        (status = 101, description = "Switching Protocols"),
        (status = 400, description = "Bad Request if WebSocket handshake fails")
    ),
    tag = "WebSocket"
)]
async fn websocket_handler(req: HttpRequest, stream: web::Payload) -> HttpResponse {
    ws::start(MyWebSocketActor {}, &req, stream).unwrap_or_else(|e| {
        eprintln!("WebSocket error: {:?}", e);
        HttpResponse::InternalServerError().finish()
    })


}


/// Get Stock
///
/// To get stock list
#[utoipa::path(
    get,
    context_path = "/v1",
    path = "/stocks",
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
    tag = "Stock"
)]
#[get("/v1/stocks")]
pub async fn get_stocks(
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

    // Fetch stock summaries
    let stocks = match Stock::get_stocks(data.clone()).await {
        Ok(summary) => summary,
        Err(err) => {
            let resp = Response::new(
                None,
                None,
                Some(format!("Error fetching stocks: {}", err)),
                None,
                StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            );

            return Ok(HttpResponse::InternalServerError().json(web::Json(resp)));
        }
    };

    // Construct response with meta data
    let resp = Response::new(
        Some(HashMap::from([(
            "stocks".to_string(),
            DataValue::StocksArray(stocks),
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

/// Get Bearish Stock
///
/// To get bearish stock
#[utoipa::path(
    get,
    context_path = "/v1",
    path = "/stock/bearish",
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
    tag = "Stock - Analytics"
)]
#[get("/v1/stock/bearish")]
pub async fn get_bearish_stock(
    query: Result<web::Query<QueryParams>, Error>,
    data: web::Data<Arc<Client>>,
) -> HttpResponse {
    let offset: &i32 = match &query {
        Ok(query) => query.offset.as_ref().unwrap_or(&0),
        Err(_) => &0,
    };
    
    let limit: &i32 = match &query {
        Ok(query) => query.limit.as_ref().unwrap_or(&0),
        Err(_) => &0,
    };

    // Access the database client from app_data
    let db_client = data.get_ref();
    let query_str = format!(
        "
        WITH last_trade_date AS (
            SELECT MAX(tanggal_perdagangan_terakhir) AS last_date
            FROM transactions
        ),
        bearish_data AS (
            SELECT 
                kode_saham,
                nama_perusahaan,
                open_price,
                penutupan,
                tanggal_perdagangan_terakhir,
                (penutupan - open_price) AS bearish_value, -- Absolute bearish value
                ROUND(((penutupan - open_price) * 100.0 / open_price), 2)::DOUBLE PRECISION AS bearish_percentage -- Percentage bearish value
            FROM transactions
            WHERE tanggal_perdagangan_terakhir = (SELECT last_date FROM last_trade_date)
            AND (penutupan - open_price) < 0 -- Only select bearish data
        )
        SELECT 
            RANK() OVER (ORDER BY bearish_percentage ASC) AS rank, -- Rank by bearish percentage
            *
        FROM bearish_data
        ORDER BY bearish_percentage ASC;
        "
    );
    
    // Execute a SELECT query
    let rows = db_client
        .query(&query_str, &[])
        .await
        .map_err(|e| {
            eprintln!("Error executing query: {:?}", e);
            
            let resp = Response {
                data: None,
                error: None,
                message: Some("".to_string()),
                meta: None,
                status: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            };

            actix_web::error::InternalError::new(HttpResponse::InternalServerError().json(resp), StatusCode::INTERNAL_SERVER_ERROR)
        })
        .unwrap();

    let bearish_stocks: Vec<BearishStock::BearishStock> = rows
        .iter()
        .map(|row| BearishStock::BearishStock::new(
            row.get("rank"),
            row.get("kode_saham"),
            row.get("nama_perusahaan"),
            row.get("open_price"),
            row.get("penutupan"),
            row
                .get::<&str, chrono::NaiveDate>("tanggal_perdagangan_terakhir")
                .to_string(),
            row.get("bearish_value"),
            row.get("bearish_percentage")
        ))
        .collect();

    let resp = Response {
        data: Some(HashMap::from([(
            "stocks".to_string(),
            DataValue::BearishStockArray(bearish_stocks),
        )])),
        error: None,
        message: None,
        meta: Some(Meta {
            count: rows.len() as u8,
            limit: *limit as u8,
            offset: *offset as u8,
        }),
        status: StatusCode::OK.as_u16(),
    };

    HttpResponse::Ok().json(web::Json(resp))
}

/// Get Bullish Stock
///
/// To get bullish stock
#[utoipa::path(
    get,
    context_path = "/v1",
    path = "/stock/bullish",
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
    tag = "Stock - Analytics"
)]
#[get("/v1/stock/bullish")]
pub async fn get_bullish_stock(
    query: Result<web::Query<QueryParams>, Error>,
    data: web::Data<Arc<Client>>,
) -> HttpResponse {
    let offset: &i32 = match &query {
        Ok(query) => query.offset.as_ref().unwrap_or(&0),
        Err(_) => &0,
    };
    
    let limit: &i32 = match &query {
        Ok(query) => query.limit.as_ref().unwrap_or(&0),
        Err(_) => &0,
    };

    // Access the database client from app_data
    let db_client = data.get_ref();
    let query_str = format!(
        "
        WITH last_trade_date AS (
            SELECT MAX(tanggal_perdagangan_terakhir) AS last_date
            FROM transactions
        ),
        bullish_data AS (
            SELECT 
                kode_saham,
                nama_perusahaan,
                open_price,
                penutupan,
                tanggal_perdagangan_terakhir,
                (penutupan - open_price) AS bullish_value, -- Absolute bullish value
                ROUND(((penutupan - open_price) * 100.0 / open_price), 2)::DOUBLE PRECISION AS bullish_percentage -- Percentage bullish value
            FROM transactions
            WHERE tanggal_perdagangan_terakhir = (SELECT last_date FROM last_trade_date)
            AND open_price > 0 -- Filter out rows where open_price is zero
            AND (penutupan - open_price) > 0 -- Only select bullish data
        )
        SELECT 
            RANK() OVER (ORDER BY bullish_percentage DESC) AS rank, -- Rank by bullish percentage
            *
        FROM bullish_data
        ORDER BY bullish_percentage DESC;
        ",
    );
    
    // Execute a SELECT query
    let rows = db_client
        .query(&query_str, &[])
        .await
        .map_err(|e| {
            eprintln!("Error executing query: {:?}", e);
            
            let resp = Response {
                data: None,
                error: None,
                message: Some("".to_string()),
                meta: None,
                status: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            };

            actix_web::error::InternalError::new(HttpResponse::InternalServerError().json(resp), StatusCode::INTERNAL_SERVER_ERROR)
        })
        .unwrap();

    let bullish_stocks: Vec<BullishStock::BullishStock> = rows
        .iter()
        .map(|row| BullishStock::BullishStock::new(
            row.get("rank"),
            row.get("kode_saham"),
            row.get("nama_perusahaan"),
            row.get("open_price"),
            row.get("penutupan"),
            row
                .get::<&str, chrono::NaiveDate>("tanggal_perdagangan_terakhir")
                .to_string(),
            row.get("bullish_value"),
            row.get("bullish_percentage")
        ))
        .collect();

    let resp = Response {
        data: Some(HashMap::from([(
            "stocks".to_string(),
            DataValue::BullishStockArray(bullish_stocks),
        )])),
        error: None,
        message: None,
        meta: Some(Meta {
            count: rows.len() as u8,
            limit: *limit as u8,
            offset: *offset as u8,
        }),
        status: StatusCode::OK.as_u16(),
    };

    HttpResponse::Ok().json(web::Json(resp))
}

/// Get Stock MACD
///
/// To get stock MACD
#[utoipa::path(
    get,
    context_path = "/v1",
    path = "/stock/macd",
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
    tag = "Stock - Analytics"
)]
#[get("/v1/stock/macd")]
pub async fn get_macd_stock(
    query: Result<web::Query<QueryParams>, Error>,
    data: web::Data<Arc<Client>>,
) -> HttpResponse {
    let offset: &i32 = match &query {
        Ok(query) => query.offset.as_ref().unwrap_or(&0),
        Err(_) => &0,
    };
    
    let limit: &i32 = match &query {
        Ok(query) => query.limit.as_ref().unwrap_or(&0),
        Err(_) => &0,
    };

    // Access the database client from app_data
    let db_client = data.get_ref();
    let query_str = format!(
        "
        WITH prices AS (
            SELECT 
                t.kode_saham,
                t.nama_perusahaan,
                t.open_price,
                t.penutupan,
                t.tanggal_perdagangan_terakhir
            FROM transactions t
            ORDER BY t.kode_saham, t.tanggal_perdagangan_terakhir
        ),
        ema_12 AS (
            SELECT 
                p.kode_saham,
                p.nama_perusahaan,
                p.open_price,
                p.penutupan,
                p.tanggal_perdagangan_terakhir,
                (p.penutupan * 2 / 13.0) + 
                (COALESCE(LAG(p.penutupan) OVER (PARTITION BY p.kode_saham ORDER BY p.tanggal_perdagangan_terakhir), p.penutupan) * 11 / 13.0) AS ema_12
            FROM prices p
        ),
        ema_26 AS (
            SELECT 
                p.kode_saham,
                p.nama_perusahaan,
                p.open_price,
                p.penutupan,
                p.tanggal_perdagangan_terakhir,
                (p.penutupan * 2 / 27.0) + 
                (COALESCE(LAG(p.penutupan) OVER (PARTITION BY p.kode_saham ORDER BY p.tanggal_perdagangan_terakhir), p.penutupan) * 25 / 27.0) AS ema_26
            FROM prices p
        ),
        macd_line AS (
            SELECT 
                e12.kode_saham,
                e12.nama_perusahaan,
                e12.open_price,
                e12.penutupan,
                e12.tanggal_perdagangan_terakhir,
                e12.ema_12,
                e26.ema_26,
                (e12.ema_12 - e26.ema_26) AS macd -- MACD Line
            FROM ema_12 e12
            JOIN ema_26 e26 
            ON e12.kode_saham = e26.kode_saham 
            AND e12.tanggal_perdagangan_terakhir = e26.tanggal_perdagangan_terakhir
        ),
        signal_line AS (
            SELECT 
                ml.kode_saham,
                ml.nama_perusahaan,
                ml.open_price,
                ml.penutupan,
                ml.tanggal_perdagangan_terakhir,
                ml.macd,
                (ml.macd * 2 / 10.0) + 
                (COALESCE(LAG(ml.macd) OVER (PARTITION BY ml.kode_saham ORDER BY ml.tanggal_perdagangan_terakhir), ml.macd) * 8 / 10.0) AS signal
            FROM macd_line ml
        ),
		trendline AS (
		    SELECT 
		        sl.kode_saham,
		        sl.nama_perusahaan,
		        sl.open_price,
		        sl.penutupan,
		        sl.tanggal_perdagangan_terakhir,
		        sl.macd AS macd_line,
		        sl.signal AS signal_line,
		        (sl.macd - sl.signal) AS macd_histogram,
		        CASE
		            WHEN sl.macd > sl.signal AND LAG(sl.macd) OVER (PARTITION BY sl.kode_saham ORDER BY sl.tanggal_perdagangan_terakhir) <= LAG(sl.signal) OVER (PARTITION BY sl.kode_saham ORDER BY sl.tanggal_perdagangan_terakhir)
		            THEN 'BUY'
		            WHEN sl.macd < sl.signal AND LAG(sl.macd) OVER (PARTITION BY sl.kode_saham ORDER BY sl.tanggal_perdagangan_terakhir) >= LAG(sl.signal) OVER (PARTITION BY sl.kode_saham ORDER BY sl.tanggal_perdagangan_terakhir)
		            THEN 'SELL'
		            ELSE 'HOLD'
		        END AS trendline
		    FROM signal_line sl
		)
        SELECT
		    t.kode_saham,
		    t.nama_perusahaan,
		    t.open_price,
		    t.penutupan,
		    t.tanggal_perdagangan_terakhir,
		    t.macd_line::DOUBLE PRECISION,
		    t.signal_line::DOUBLE PRECISION,
		    t.macd_histogram::DOUBLE PRECISION,
		    t.trendline
		FROM trendline t
		WHERE t.tanggal_perdagangan_terakhir = (
            SELECT MAX(t2.tanggal_perdagangan_terakhir)
			FROM transactions t2
            WHERE t2.kode_saham = t.kode_saham
		)
		ORDER BY t.trendline ASC, t.kode_saham ASC;
        ",
    );
    println!("query_str: {:?}", query_str);
    // Execute a SELECT query
    let rows = db_client
        .query(&query_str, &[])
        .await
        .map_err(|e| {
            eprintln!("Error executing query: {:?}", e);
            
            let resp = Response {
                data: None,
                error: None,
                message: Some("".to_string()),
                meta: None,
                status: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            };

            actix_web::error::InternalError::new(HttpResponse::InternalServerError().json(resp), StatusCode::INTERNAL_SERVER_ERROR)
        })
        .unwrap();

    let macd_stocks: Vec<MACDStock> = rows
        .iter()
        .map(|row| MACDStock::new(
            row.get("kode_saham"),
            row.get("nama_perusahaan"),
            row.get("open_price"),
            row.get("penutupan"),
            row
                .get::<&str, chrono::NaiveDate>("tanggal_perdagangan_terakhir")
                .to_string(),
            row.get("macd_line"),
            row.get("signal_line"),
            row.get("macd_histogram"),
            row.get("trendline")
        ))
        .collect();

    let resp = Response {
        data: Some(HashMap::from([(
            "stocks".to_string(),
            DataValue::MACDStockArray(macd_stocks),
        )])),
        error: None,
        message: None,
        meta: Some(Meta {
            count: rows.len() as u8,
            limit: *limit as u8,
            offset: *offset as u8,
        }),
        status: StatusCode::OK.as_u16(),
    };

    HttpResponse::Ok().json(web::Json(resp))
}

/// Get Stock RSI
///
/// To get stock RSI
#[utoipa::path(
    get,
    context_path = "/v1",
    path = "/stock/rsi",
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
    tag = "Stock - Analytics"
)]
#[get("/v1/stock/rsi")]
pub async fn get_rsi_stock(
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

    // Access the database client from app_data
    let db_client = data.get_ref();
    let mut query_where_str: String = "".to_string();

    if let Some(stock_code) = query_params.stock_code {
        if !stock_code.is_empty() {
            query_where_str += &format!("transactions.kode_saham = '{}' AND ", stock_code);
        }
    }

    if let Some(trend) = query_params.trend {
        if trend == 1 {
            query_where_str += &format!("summary.trend = 'Strong Buy' AND ");
        } else if trend == 0 {
            query_where_str += &format!("summary.trend = 'Neutral' AND ");
        } else if trend == -1 {
            query_where_str += &format!("summary.trend = 'Strong Sell' AND ");
        }
    }

    let query_str: String = format!(
        "
        WITH price_changes AS (
            SELECT
                t.kode_saham,
                t.nama_perusahaan,
                t.open_price,
                t.penutupan,
                t.tanggal_perdagangan_terakhir,
                LAG(t.penutupan) OVER (PARTITION BY t.kode_saham ORDER BY t.tanggal_perdagangan_terakhir) AS previous_close,
                -- Calculate Gain and Loss
                GREATEST(t.penutupan - LAG(t.penutupan) OVER (PARTITION BY t.kode_saham ORDER BY t.tanggal_perdagangan_terakhir), 0) AS gain,
                GREATEST(LAG(t.penutupan) OVER (PARTITION BY t.kode_saham ORDER BY t.tanggal_perdagangan_terakhir) - t.penutupan, 0) AS loss
            FROM transactions t
        ),
        average_gain_loss AS (
            SELECT
                p.kode_saham,
                p.nama_perusahaan,
                p.open_price,
                p.penutupan,
                p.tanggal_perdagangan_terakhir,
                -- Calculate 6-period Average Gain and Loss
                AVG(p.gain) OVER (PARTITION BY p.kode_saham ORDER BY p.tanggal_perdagangan_terakhir ROWS BETWEEN 5 PRECEDING AND CURRENT ROW) AS avg_gain_6,
                AVG(p.loss) OVER (PARTITION BY p.kode_saham ORDER BY p.tanggal_perdagangan_terakhir ROWS BETWEEN 5 PRECEDING AND CURRENT ROW) AS avg_loss_6,
                -- Calculate 12-period Average Gain and Loss
                AVG(p.gain) OVER (PARTITION BY p.kode_saham ORDER BY p.tanggal_perdagangan_terakhir ROWS BETWEEN 11 PRECEDING AND CURRENT ROW) AS avg_gain_12,
                AVG(p.loss) OVER (PARTITION BY p.kode_saham ORDER BY p.tanggal_perdagangan_terakhir ROWS BETWEEN 11 PRECEDING AND CURRENT ROW) AS avg_loss_12,
                -- Calculate 24-period Average Gain and Loss
                AVG(p.gain) OVER (PARTITION BY p.kode_saham ORDER BY p.tanggal_perdagangan_terakhir ROWS BETWEEN 23 PRECEDING AND CURRENT ROW) AS avg_gain_24,
                AVG(p.loss) OVER (PARTITION BY p.kode_saham ORDER BY p.tanggal_perdagangan_terakhir ROWS BETWEEN 23 PRECEDING AND CURRENT ROW) AS avg_loss_24
            FROM price_changes p
        ),
        rsi_calculation AS (
            SELECT
                a.kode_saham,
                a.nama_perusahaan,
                a.open_price,
                a.penutupan,
                a.tanggal_perdagangan_terakhir,
                -- RSI 6
                CASE 
                    WHEN a.avg_loss_6 = 0 THEN 100
                    ELSE 100 - (100 / (1 + (a.avg_gain_6 / a.avg_loss_6)))
                END AS rsi_6,
                -- RSI 12
                CASE 
                    WHEN a.avg_loss_12 = 0 THEN 100
                    ELSE 100 - (100 / (1 + (a.avg_gain_12 / a.avg_loss_12)))
                END AS rsi_12,
                -- RSI 24
                CASE 
                    WHEN a.avg_loss_24 = 0 THEN 100
                    ELSE 100 - (100 / (1 + (a.avg_gain_24 / a.avg_loss_24)))
                END AS rsi_24
            FROM average_gain_loss a
        ),
        trendline_signals AS (
            SELECT 
                r.kode_saham,
                r.nama_perusahaan,
                r.open_price,
                r.penutupan,
                r.tanggal_perdagangan_terakhir,
                ROUND(r.rsi_6, 2) AS rsi_6,
                ROUND(r.rsi_12, 2) AS rsi_12,
                ROUND(r.rsi_24, 2) AS rsi_24,
                -- Generate Trendline Signal
                CASE 
                    WHEN r.rsi_6 < 30 OR r.rsi_12 < 30 OR r.rsi_24 < 30 THEN 'BUY' -- Oversold condition
                    WHEN r.rsi_6 > 70 OR r.rsi_12 > 70 OR r.rsi_24 > 70 THEN 'SELL' -- Overbought condition
                    ELSE 'HOLD' -- No clear trend
                END AS trendline
            FROM rsi_calculation r
        )
        SELECT 
            t.kode_saham,
            t.nama_perusahaan,
            t.open_price,
            t.penutupan,
            t.tanggal_perdagangan_terakhir,
            t.rsi_6::DOUBLE PRECISION,
            t.rsi_12::DOUBLE PRECISION,
            t.rsi_24::DOUBLE PRECISION,
            t.trendline
        FROM trendline_signals t
        WHERE t.tanggal_perdagangan_terakhir = (
            SELECT MAX(tanggal_perdagangan_terakhir) 
            FROM transactions
        )
        ORDER BY t.trendline ASC, t.kode_saham ASC;
        "
    );

    // Execute a SELECT query
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

    // Convert the rows to a vector of tuples
    let data: Vec<RSIStock> = rows
        .iter()
        .map(|row| {
            RSIStock::new(
                row.get("kode_saham"),
                row.get("nama_perusahaan"),
                row.get("open_price"),
                row.get("penutupan"),
                row
                    .get::<&str, chrono::NaiveDate>("tanggal_perdagangan_terakhir")
                    .to_string(),
                row.get("rsi_6"),
                row.get("rsi_12"),
                row.get("rsi_24"),
                row.get("trendline")
            )
        })
        .collect();

    let resp = Response::new(
        Some(HashMap::from([(
            "stocks".to_string(),
            DataValue::RSIStockArray(data),
        )])),
        None,
        None,
        Some(Meta {
            count: rows.len() as u8,
            limit: 0,
            offset: 0,
        }),
        StatusCode::OK.as_u16(),
    );

    Ok(HttpResponse::Ok().json(web::Json(resp)))
}

/// Get Stock SMA
///
/// To get stock SMA
#[utoipa::path(
    get,
    context_path = "/v1",
    path = "/stock/sma",
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
    tag = "Stock - Analytics"
)]
#[get("/v1/stock/sma")]
pub async fn get_sma_stock(
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

    // Access the database client from app_data
    let db_client = data.get_ref();
    let mut query_where_str: String = "".to_string();

    if let Some(stock_code) = query_params.stock_code {
        if !stock_code.is_empty() {
            query_where_str += &format!("transactions.kode_saham = '{}' AND ", stock_code);
        }
    }

    if let Some(trend) = query_params.trend {
        if trend == 1 {
            query_where_str += &format!("summary.trend = 'Strong Buy' AND ");
        } else if trend == 0 {
            query_where_str += &format!("summary.trend = 'Neutral' AND ");
        } else if trend == -1 {
            query_where_str += &format!("summary.trend = 'Strong Sell' AND ");
        }
    }

    let query_str: String = format!(
        "
        WITH sma_7 AS (
            SELECT
                kode_saham,
                nama_perusahaan,
                ROUND(AVG(penutupan)) AS sma_7_value
            FROM
                transactions
            WHERE
                tanggal_perdagangan_terakhir BETWEEN DATE_TRUNC(
                    'day',
                    (
                        SELECT
                            MAX(tanggal_perdagangan_terakhir)
                        FROM
                            transactions
                    ) - INTERVAL '6 days'
                )
                AND (
                    SELECT
                        MAX(tanggal_perdagangan_terakhir)
                    FROM
                        transactions
                )
                AND penutupan > 100
            GROUP BY
                kode_saham, 
                nama_perusahaan
        ),
        sma_14 AS (
            SELECT
                kode_saham,
                nama_perusahaan,
                ROUND(AVG(penutupan)) AS sma_14_value
            FROM
                transactions
            WHERE
                tanggal_perdagangan_terakhir BETWEEN DATE_TRUNC(
                    'day',
                    (
                        SELECT
                            MAX(tanggal_perdagangan_terakhir)
                        FROM
                            transactions
                    ) - INTERVAL '13 days'
                )
                AND (
                    SELECT
                        MAX(tanggal_perdagangan_terakhir)
                    FROM
                        transactions
                )
            AND penutupan > 100
            GROUP BY
                kode_saham,
                nama_perusahaan
        ),
        sma_200 AS (
            SELECT
                kode_saham,
                nama_perusahaan,
                ROUND(AVG(penutupan)) AS sma_200_value
            FROM
                transactions
            WHERE
                tanggal_perdagangan_terakhir BETWEEN DATE_TRUNC(
                    'day',
                    (
                        SELECT
                            MAX(tanggal_perdagangan_terakhir)
                        FROM
                            transactions
                    ) - INTERVAL '185 days'
                )
                AND (
                    SELECT
                        MAX(tanggal_perdagangan_terakhir)
                    FROM
                        transactions
                )
            AND penutupan > 100
            GROUP BY
                kode_saham,
                nama_perusahaan
        ),
        min_max AS (
            SELECT
                transactions.kode_saham,
                transactions.nama_perusahaan,
                (
                    SELECT
                        MIN(penutupan)
                    FROM
                        transactions min_max
                    WHERE
                        min_max.kode_saham = transactions.kode_saham
                ) AS lowest_price,
                (
                    SELECT
                        MAX(penutupan)
                    FROM
                        transactions min_max
                    WHERE
                        min_max.kode_saham = transactions.kode_saham
                ) AS highest_price
            FROM
                transactions
            GROUP BY
                transactions.kode_saham,
                transactions.nama_perusahaan
        ),
            summary AS (SELECT sma_7.kode_saham, (CASE
                WHEN sma_7.sma_7_value > sma_14.sma_14_value
                AND sma_7.sma_7_value > sma_200.sma_200_value THEN 'Strong Buy'
                WHEN sma_7.sma_7_value < sma_14.sma_14_value
                AND sma_14.sma_14_value > sma_200.sma_200_value THEN 'Strong Sell'
                ELSE 'Neutral'
            END) AS trend FROM sma_7 
            JOIN sma_14 ON sma_7.kode_saham = sma_14.kode_saham
            JOIN sma_200 ON sma_7.kode_saham = sma_200.kode_saham GROUP BY sma_7.kode_saham, sma_7.sma_7_value, sma_14.sma_14_value, sma_200.sma_200_value)
        SELECT DISTINCT
            transactions.kode_saham AS stock_code,
            transactions.nama_perusahaan AS nama_perusahaan,
            transactions.penutupan AS close_price,
            CAST(sma_7.sma_7_value AS INTEGER),
            CAST(sma_14.sma_14_value AS INTEGER),
            CAST(sma_200.sma_200_value AS INTEGER),
            min_max.lowest_price,
            min_max.highest_price,
            summary.trend
        FROM
            transactions
            JOIN sma_7 ON transactions.kode_saham = sma_7.kode_saham
            JOIN sma_14 ON transactions.kode_saham = sma_14.kode_saham
            JOIN sma_200 ON transactions.kode_saham = sma_200.kode_saham
            JOIN min_max ON transactions.kode_saham = min_max.kode_saham
            JOIN summary ON transactions.kode_saham = summary.kode_saham
        WHERE
            {}
            transactions.tanggal_perdagangan_terakhir = (
                SELECT
                    MAX(transactions.tanggal_perdagangan_terakhir)
                FROM
                    transactions
            ) AND
            (sma_7.sma_7_value <= sma_14.sma_14_value OR
            sma_7.sma_7_value - sma_14.sma_14_value <= 0.05 * sma_14.sma_14_value)
        ORDER BY
            transactions.kode_saham ASC
    ",
    query_where_str
    );

    println!("{}", query_str);
    // Execute a SELECT query
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

    // Convert the rows to a vector of tuples
    let data: Vec<StockSMA> = rows
        .iter()
        .map(|row| {
            StockSMA::new(
                row.get(0),
                row.get(1),
                row.get(2),
                row.get(3),
                row.get(4),
                row.get(5),
                row.get(6),
                row.get(7),
                row.get(8),
            )
        })
        .collect();

    let resp = Response::new(
        Some(HashMap::from([(
            "stocks".to_string(),
            DataValue::StockSMAArray(data),
        )])),
        None,
        None,
        Some(Meta {
            count: rows.len() as u8,
            limit: 0,
            offset: 0,
        }),
        StatusCode::OK.as_u16(),
    );

    Ok(HttpResponse::Ok().json(web::Json(resp)))
}

/// Add Stock
///
/// Do add new stock
#[utoipa::path(
    post,
    context_path = "/v1",
    path = "/stock/ringkasan-saham",
    params(Header),
    request_body(content = UploadRequestBody, content_type = "multipart/form-data"),
    responses(
        (status = 100, description = "Continue", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::CONTINUE.to_string(), "message": ""})),
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
        // (),
        // ("my_auth" = ["read:items", "edit:items"]),
        // ("token_jwt" = []),
        ("oauth2" = [])
    ),
    tag = "Stock - Ringkasan Saham"
)]
#[post("/v1/stock/ringkasan-saham")]
pub async fn add_stock(
    _req: HttpRequest,
    mut payload: Multipart,
    data: web::Data<Arc<Client>>,
) -> Result<impl Responder> {
    // Access the database client from app_data
    let db_client = data.get_ref();

    while let Some(item) = payload.next().await {
        let mut filepath: String = "".to_string();
        let mut field = item?;
        let content_disposition = field.content_disposition();

        // Check the content disposition to see if the field name is "file"
        if let Some(content_disp) = content_disposition {
            // Extract the field name
            if let Some(name) = content_disp.get_name() {
                if name != "file" {
                    let resp = Response::new(
                        None,
                        None,
                        Some("Field file not found".to_string()),
                        None,
                        StatusCode::BAD_REQUEST.as_u16(),
                    );
        
                    return Ok(HttpResponse::BadRequest().json(web::Json(resp)));
                }
        
                // Get the filename from the content disposition
                if let Some(filename) = content_disp.get_filename() {
                    // Check if filename ends with ".xlsx"
                    if !filename.ends_with(".xlsx") {
                        let resp = Response::new(
                            None,
                            None,
                            Some("File format not supported".to_string()),
                            None,
                            StatusCode::UNSUPPORTED_MEDIA_TYPE.as_u16(),
                        );
        
                        return Ok(HttpResponse::UnsupportedMediaType().json(web::Json(resp)));
                    }
        
                    // Extract the date from the filename
                    if let Some((year, month, _day)) = extract_date(filename) {
                        let directory_path = format!("./uploads/{}/{}", year, month);
        
                        // Create directory if it doesn't exist
                        if let Err(e) = fs::create_dir_all(&directory_path) {
                            println!("Error creating directory: {}", e);
                        } else {
                            println!("Directory created or already exists");
                        }
        
                        // Construct the full file path
                        filepath = format!("{}/{}", directory_path, filename);
        
                        // Create a file and write the content to it
                        let mut f = File::create(&filepath)?;
                        while let Some(chunk) = field.next().await {
                            let data = chunk?;
                            f.write_all(&data)?;
                        }
                        println!("File created at {}", filepath);
                    }
                }
            }
        }        

        // if let Some(filepath) = filepath {
        // Open the workbook using the filepath
        let mut workbook: Xlsx<_> = match open_workbook(filepath) {
            Ok(workbook) => workbook,
            Err(e) => {
                let resp = Response::new(
                    None,
                    None,
                    Some(format!("Cannot open file: {:?}", e)),
                    None,
                    StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                );

                return Ok(HttpResponse::InternalServerError().json(web::Json(resp)));
            }
        };

        // Process the workbook
        if let Some(range) = workbook.worksheet_range("Sheet1").ok() {
            for (i, row) in range.rows().enumerate() {
                if i == 0 {
                    continue; // Skip the first row
                }

                let parsed_date = parse_custom_date(row[6].get_string().unwrap_or(""))?;
                let no: i32 = transform_to_i32(&row[0]);
                let kode_saham: &str = row[1].get_string().unwrap_or("");
                let nama_perusahaan: &str = row[2].get_string().unwrap_or("");
                let remarks: &str = row[3].get_string().unwrap_or("");
                let sebelumnya: i32 = transform_to_i32(&row[4]);
                let open_price: i32 = transform_to_i32(&row[5]);
                let tanggal_perdagangan_terakhir: &CustomNaiveDate::CustomDate = &parsed_date;
                let first_trade: i32 = transform_to_i32(&row[7]);
                let tertinggi: i32 = transform_to_i32(&row[8]);
                let terendah: i32 = transform_to_i32(&row[9]);
                let penutupan: i32 = transform_to_i32(&row[10]);
                let selisih: i32 = transform_to_i32(&row[11]);
                let volume: i32 = transform_to_i32(&row[12]);
                let nilai: i32 = transform_to_i32(&row[13]);
                let frekuensi: i32 = transform_to_i32(&row[14]);
                let index_individual: i32 = transform_to_i32(&row[15]);
                let offer: i32 = transform_to_i32(&row[16]);
                let offer_volume: i32 = transform_to_i32(&row[17]);
                let bid: i32 = transform_to_i32(&row[18]);
                let bid_volume: i32 = transform_to_i32(&row[19]);
                let listed_shares: i32 = transform_to_i32(&row[20]);
                let tradeble_shares: i32 = transform_to_i32(&row[21]);
                let weight_for_index: i32 = transform_to_i32(&row[22]);
                let foreign_sell: i32 = transform_to_i32(&row[23]);
                let foreign_buy: i32 = transform_to_i32(&row[24]);
                let non_regular_volume: i32 = transform_to_i32(&row[25]);
                let non_regular_value: i32 = transform_to_i32(&row[26]);
                let non_regular_frequency: i32 = transform_to_i32(&row[27]);

                // Insert the data into the PostgreSQL database
                let insert_query = "INSERT INTO transactions (no, kode_saham, nama_perusahaan, remarks, sebelumnya, open_price, tanggal_perdagangan_terakhir, first_trade, tertinggi, terendah, penutupan, selisih, volume, nilai, frekuensi, index_individual, offer, offer_volume, bid, bid_volume, listed_shares, tradeble_shares, weight_for_index, foreign_sell, foreign_buy, non_regular_volume, non_regular_value, non_regular_frequency) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22, $23, $24, $25, $26, $27, $28) ON CONFLICT (no, kode_saham, tanggal_perdagangan_terakhir) DO NOTHING";
                if let Err(e) = db_client
                    .execute(
                        insert_query,
                        &[
                            &no,
                            &kode_saham,
                            &nama_perusahaan,
                            &remarks,
                            &sebelumnya,
                            &open_price,
                            &tanggal_perdagangan_terakhir,
                            &first_trade,
                            &tertinggi,
                            &terendah,
                            &penutupan,
                            &selisih,
                            &volume,
                            &nilai,
                            &frekuensi,
                            &index_individual,
                            &offer,
                            &offer_volume,
                            &bid,
                            &bid_volume,
                            &listed_shares,
                            &tradeble_shares,
                            &weight_for_index,
                            &foreign_sell,
                            &foreign_buy,
                            &non_regular_volume,
                            &non_regular_value,
                            &non_regular_frequency,
                        ],
                    )
                    .await
                {
                    let resp = Response::new(
                        None,
                        None,
                        Some(e.to_string()),
                        None,
                        StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                    );

                    return Ok(HttpResponse::InternalServerError().json(web::Json(resp)));
                }
            }
        } else {
            let resp = Response::new(
                None,
                None,
                Some("Error reading worksheet".to_string()),
                None,
                StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            );

            return Ok(HttpResponse::InternalServerError().json(web::Json(resp)));
        }
        // }
    }

    let resp = Response::new(
        None,
        None,
        Some("Success".to_string()),
        None,
        StatusCode::OK.as_u16(),
    );

    return Ok(HttpResponse::Ok().json(web::Json(resp)));
}

/// Get Stock Summary
///
/// To get stock summary
#[utoipa::path(
    get,
    context_path = "/v1",
    path = "/stock/summary",
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
    tag = "Stock - Analytics"
)]
#[get("/v1/stock/summary")]
pub async fn get_stock_summary(
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
    
    // Fetch stock summaries
    let stocks_summary = match SummaryStock::get_stocks_summary(data.clone()).await {
        Ok(summary) => summary,
        Err(err) => {
            let resp = Response::new(
                None,
                None,
                Some(format!("Error fetching stock summary: {}", err)),
                None,
                StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            );

            return Ok(HttpResponse::InternalServerError().json(web::Json(resp)));
        }
    };

    // Construct response with meta data
    let resp = Response::new(
        Some(HashMap::from([(
            "stocks".to_string(),
            DataValue::SummaryStockArray(stocks_summary),
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

/// Get Stock Pemantauan Khusus
///
/// Do get stock pemantauan khusus
#[utoipa::path(
    get,
    context_path = "/v1",
    path = "/stock/pemantauan-khusus",
    params(Header),
    responses(
        (status = 100, description = "Continue", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::CONTINUE.to_string(), "message": ""})),
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
        // (),
        // ("my_auth" = ["read:items", "edit:items"]),
        // ("token_jwt" = []),
        ("oauth2" = [])
    ),
    tag = "Stock - Pemantauan Khusus"
)]
#[get("/v1/stock/pemantauan-khusus")]
pub async fn get_pemantauan_khusus_stock(
    _query: Result<web::Query<QueryParams>, Error>,
    _data: web::Data<Arc<Client>>,
) -> Result<impl Responder> {
    Ok(HttpResponse::Ok().json(""))
}

/// Add Stock Pemantauan Khusus
///
/// Do add new stock pemantauan khusus
#[utoipa::path(
    post,
    context_path = "/v1",
    path = "/stock/pemantauan-khusus",
    params(Header),
    // request_body(content = UploadRequestBody, description = "Upload a file", content_type = "multipart/form-data"),
    request_body(content = UploadRequestBody, content_type = "multipart/form-data"),
    // request_body = UploadRequestBody,
    responses(
        (status = 100, description = "Continue", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::CONTINUE.to_string(), "message": ""})),
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
        // (),
        // ("my_auth" = ["read:items", "edit:items"]),
        // ("token_jwt" = []),
        ("oauth2" = [])
    ),
    tag = "Stock - Pemantauan Khusus"
)]
#[post("/v1/stock/pemantauan-khusus")]
pub async fn add_pemantauan_khusus_stock(
    _req: HttpRequest,
    mut payload: Multipart,
    data: web::Data<Arc<Client>>,
) -> Result<impl Responder> {
    // Access the database client from app_data
    let db_client = data.get_ref();

    while let Some(item) = payload.next().await {
        let mut filepath = None;
        let mut field = item?;
        let content_disposition = field.content_disposition();

        // Check the content disposition to see if the field name is "file"
        if let Some(content_disp) = content_disposition {
            // Get the name from ContentDisposition
            if let Some(name) = content_disp.get_name() {
                if name != "file" {
                    let resp = Response::new(
                        None,
                        None,
                        Some("Field file not found".to_string()),
                        None,
                        StatusCode::BAD_REQUEST.as_u16(),
                    );
        
                    return Ok(HttpResponse::BadRequest().json(web::Json(resp)));
                }
            }
        
            // Get the filename from ContentDisposition
            if let Some(filename) = content_disp.get_filename() {
                // Check if filename ends with ".xlsx"
                if !filename.ends_with(".xlsx") {
                    let resp = Response::new(
                        None,
                        None,
                        Some("File format not supported".to_string()),
                        None,
                        StatusCode::UNSUPPORTED_MEDIA_TYPE.as_u16(),
                    );
        
                    return Ok(HttpResponse::UnsupportedMediaType().json(web::Json(resp)));
                }
        
                let filepath_str = format!("./uploads/{}", filename);
                filepath = Some(filepath_str.clone());
        
                // Create a file and write the content to it
                let mut f = File::create(&filepath_str)?;
                while let Some(chunk) = field.next().await {
                    let data = chunk?;
                    f.write_all(&data)?;
                }
            }
        }        

        if let Some(filepath) = filepath {
            // Open the workbook using the filepath
            let mut workbook: Xlsx<_> = match open_workbook(filepath) {
                Ok(workbook) => workbook,
                Err(e) => {
                    let resp = Response::new(
                        None,
                        None,
                        Some(format!("Cannot open file: {:?}", e)),
                        None,
                        StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                    );

                    return Ok(HttpResponse::InternalServerError().json(web::Json(resp)));
                }
            };

            // Process the workbook
            if let Some(range) = workbook.worksheet_range("Sheet1").ok() {
                for (i, row) in range.rows().enumerate() {
                    if i == 0 {
                        continue; // Skip the first row
                    }

                    let no: i32 = transform_to_i32(&row[0]);
                    let code: &str = row[1].get_string().unwrap_or("");
                    let name: &str = row[2].get_string().unwrap_or("");
                    let notasi: &str = row[3].get_string().unwrap_or("");

                    // Insert the data into the PostgreSQL database
                    let insert_query = "INSERT INTO list_saham_dengan_notifikasi_khusus (no, code, name, notasi) VALUES ($1, $2, $3, $4) ON CONFLICT (no, code) DO NOTHING";
                    if let Err(e) = db_client
                        .execute(insert_query, &[&no, &code, &name, &notasi])
                        .await
                    {
                        let resp = Response::new(
                            None,
                            None,
                            Some(e.to_string()),
                            None,
                            StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                        );

                        return Ok(HttpResponse::InternalServerError().json(web::Json(resp)));
                    }
                }
            } else {
                let resp = Response::new(
                    None,
                    None,
                    Some("Error reading worksheet".to_string()),
                    None,
                    StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                );

                return Ok(HttpResponse::InternalServerError().json(web::Json(resp)));
            }

            // Process the workbook
            if let Some(range) = workbook.worksheet_range("Sheet2").ok() {
                for (i, row) in range.rows().enumerate() {
                    if i == 0 {
                        continue; // Skip the first row
                    }

                    let notasi: &str = row[0].get_string().unwrap_or("");
                    let deskripsi: &str = row[1].get_string().unwrap_or("");

                    // Insert the data into the PostgreSQL database
                    let insert_query = "INSERT INTO notifikasi_khusus (notasi, deskripsi) VALUES ($1, $2) ON CONFLICT (notasi) DO NOTHING";
                    if let Err(e) = db_client
                        .execute(insert_query, &[&notasi, &deskripsi])
                        .await
                    {
                        let resp = Response::new(
                            None,
                            None,
                            Some(e.to_string()),
                            None,
                            StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                        );

                        return Ok(HttpResponse::InternalServerError().json(web::Json(resp)));
                    }
                }
            } else {
                let resp = Response::new(
                    None,
                    None,
                    Some("Error reading worksheet".to_string()),
                    None,
                    StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                );

                return Ok(HttpResponse::InternalServerError().json(web::Json(resp)));
            }

            // Process the workbook
            if let Some(range) = workbook.worksheet_range("Sheet3").ok() {
                for (i, row) in range.rows().enumerate() {
                    if i == 0 {
                        continue; // Skip the first row
                    }
                    // println!("i {} value {:?}", i, row[3]);
                    // let parsed_tanggal_masuk = parse_custom_date(row[3].get_string().unwrap_or(""))?;
                    // let parsed_tanggal_keluar = parse_custom_date(row[4].get_string().unwrap_or(""))?;
                    // let parsed_tanggal_keluar: Option<&NaiveDate> = if let Some(date_str) = row[4].get_string() {
                    //     parse_custom_date(date_str)?
                    // } else {
                    //     None
                    // };
                    // let parsed_tanggal_keluar: Option<&NaiveDate> = row[4].get_string().map(|date_str| parse_custom_date(date_str).as_ref());

                    let no: i32 = transform_to_i32(&row[0]);
                    let kode_saham: &str = row[1].get_string().unwrap_or("");
                    let nama_perusahaan: &str = row[2].get_string().unwrap_or("");
                    let _tanggal_masuk: &str = "";
                    let _tanggal_keluar: &str = "";
                    let kriteria: &str = row[5].get_string().unwrap_or("");

                    // Insert the data into the PostgreSQL database
                    let insert_query = format!("INSERT INTO daftar_efek_bersifat_ekuitas_dalam_pemantauan_khusus (no, kode_saham, nama_perusahaan, tanggal_masuk, tanggal_keluar, kriteria) VALUES ('{}', '{}', '{}', NULL, NULL, '{}') ON CONFLICT (kode_saham) DO NOTHING", no, kode_saham, nama_perusahaan, kriteria);

                    if let Err(e) = db_client.execute(&insert_query, &[]).await {
                        let resp = Response::new(
                            None,
                            None,
                            Some(e.to_string()),
                            None,
                            StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                        );

                        return Ok(HttpResponse::InternalServerError().json(web::Json(resp)));
                    }
                }
            } else {
                let resp = Response::new(
                    None,
                    None,
                    Some("Error reading worksheet".to_string()),
                    None,
                    StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                );

                return Ok(HttpResponse::InternalServerError().json(web::Json(resp)));
            }

            // Process the workbook
            if let Some(range) = workbook.worksheet_range("Sheet4").ok() {
                for (i, row) in range.rows().enumerate() {
                    if i == 0 {
                        continue; // Skip the first row
                    }

                    let _no: i32 = transform_to_i32(&row[0]);
                    let kode_saham: &str = row[1].get_string().unwrap_or("");
                    let _nama_perusahaan: &str = row[2].get_string().unwrap_or("");
                    let kriteria_efek_dalam_pemantauan_khusus: i32 = transform_to_i32(&row[3]);
                    let keterangan: &str = row[4].get_string().unwrap_or("");

                    // Insert the data into the PostgreSQL database
                    let insert_query = format!("UPDATE daftar_efek_bersifat_ekuitas_dalam_pemantauan_khusus SET kriteria_efek_dalam_pemantauan_khusus = '{}', keterangan = '{}' WHERE kode_saham = '{}'", kriteria_efek_dalam_pemantauan_khusus, keterangan, kode_saham);

                    if let Err(e) = db_client.execute(&insert_query, &[]).await {
                        let resp = Response::new(
                            None,
                            None,
                            Some(e.to_string()),
                            None,
                            StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                        );

                        return Ok(HttpResponse::InternalServerError().json(web::Json(resp)));
                    }
                }
            } else {
                let resp = Response::new(
                    None,
                    None,
                    Some("Error reading worksheet".to_string()),
                    None,
                    StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                );

                return Ok(HttpResponse::InternalServerError().json(web::Json(resp)));
            }

            // Process the workbook
            if let Some(range) = workbook.worksheet_range("Sheet5").ok() {
                for (i, row) in range.rows().enumerate() {
                    if i == 0 {
                        continue; // Skip the first row
                    }

                    let no: i32 = transform_to_i32(&row[0]);
                    let keterangan: &str = row[1].get_string().unwrap_or("");

                    // Insert the data into the PostgreSQL database
                    let insert_query = "INSERT INTO kriteria_efek_bersifat_ekuitas_dalam_pemantauan_khusus (no, keterangan) VALUES ($1, $2) ON CONFLICT (no, keterangan) DO NOTHING";
                    if let Err(e) = db_client.execute(insert_query, &[&no, &keterangan]).await {
                        let resp = Response::new(
                            None,
                            None,
                            Some(e.to_string()),
                            None,
                            StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                        );

                        return Ok(HttpResponse::InternalServerError().json(web::Json(resp)));
                    }
                }
            } else {
                let resp = Response::new(
                    None,
                    None,
                    Some("Error reading worksheet".to_string()),
                    None,
                    StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                );

                return Ok(HttpResponse::InternalServerError().json(web::Json(resp)));
            }
        }
    }

    let resp = Response::new(
        None,
        None,
        Some("Success".to_string()),
        None,
        StatusCode::OK.as_u16(),
    );

    return Ok(HttpResponse::Ok().json(web::Json(resp)));
}

/// Revoke stock
///
/// To delete stock
#[utoipa::path(
    delete,
    context_path = "/v1",
    path = "/stock",
    responses(
        (status = 100, description = "Continue", content_type = "application/json", body = Response, content_type = "application/json",
            example = json!({"status": StatusCode::CONTINUE.to_string(), "message": ""})),
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
        ("auth" = [])
    ),
    tag = "Stock"
)]
#[delete("/v1/stock")]
pub async fn delete_stock(_req: HttpRequest) -> Result<impl Responder> {
    // let resp = repository::Response {
    //     data: Some(repository::DUUID {
    //         uuid: Uuid::new_v4().to_string(),
    //     }),
    //     error: None,
    //     message: Some("".to_string()),
    //     meta: Some(repository::Meta { count: 0, limit: 0, offset: 0 }),
    //     status: StatusCode::OK.to_string(),
    // };
    // Ok(web::Json(resp))
    Ok(HttpResponse::Ok().body("Success"))
}

/// Get Stock Rank By Frequency
///
/// To get stock rank by frequency
#[utoipa::path(
    get,
    context_path = "/v1",
    path = "/stock/frequency",
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
    tag = "Stock - Analytics"
)]
#[get("/v1/stock/frequency")]
pub async fn get_stock_rank_by_frequency(
    query: Result<web::Query<QueryParams>, Error>,
    data: web::Data<Arc<Client>>,
) -> HttpResponse {
    let offset: &i32 = match &query {
        Ok(query) => query.offset.as_ref().unwrap_or(&0),
        Err(_) => &0,
    };
    
    let limit: &i32 = match &query {
        Ok(query) => query.limit.as_ref().unwrap_or(&0),
        Err(_) => &0,
    };

    // Access the database client from app_data
    let db_client = data.get_ref();
    let query_str = format!(
        "
        WITH last_trade_date AS (
            SELECT MAX(tanggal_perdagangan_terakhir) AS last_date
            FROM transactions
        ),
        ranked_stocks AS (
            SELECT
                t.kode_saham,
                t.nama_perusahaan,
                t.open_price,
                t.penutupan,
                t.frekuensi,
                t.tanggal_perdagangan_terakhir,
                RANK() OVER (ORDER BY t.frekuensi DESC) AS rank
            FROM transactions t
            WHERE t.tanggal_perdagangan_terakhir = (SELECT last_date FROM last_trade_date)
        )
        SELECT 
            rank,
            kode_saham,
            nama_perusahaan,
            open_price,
            penutupan,
            frekuensi,
            tanggal_perdagangan_terakhir
        FROM ranked_stocks
        ORDER BY rank ASC;
        ",
    );
    
    // Execute a SELECT query
    let rows = db_client
        .query(&query_str, &[])
        .await
        .map_err(|e| {
            eprintln!("Error executing query: {:?}", e);
            
            let resp = Response {
                data: None,
                error: None,
                message: Some("".to_string()),
                meta: None,
                status: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            };

            actix_web::error::InternalError::new(HttpResponse::InternalServerError().json(resp), StatusCode::INTERNAL_SERVER_ERROR)
        })
        .unwrap();

    let frequency_stocks: Vec<FrequencyStock> = rows
        .iter()
        .map(|row| FrequencyStock::new(
            row.get("rank"),
            row.get("kode_saham"),
            row.get("nama_perusahaan"),
            row.get("open_price"),
            row.get("penutupan"),
            row.get("frekuensi"),
            row
                .get::<&str, chrono::NaiveDate>("tanggal_perdagangan_terakhir")
                .to_string()
        ))
        .collect();

    let resp = Response {
        data: Some(HashMap::from([(
            "stocks".to_string(),
            DataValue::FrequencyStockArray(frequency_stocks),
        )])),
        error: None,
        message: None,
        meta: Some(Meta {
            count: rows.len() as u8,
            limit: *limit as u8,
            offset: *offset as u8,
        }),
        status: StatusCode::OK.as_u16(),
    };

    HttpResponse::Ok().json(web::Json(resp))
}

fn parse_custom_date(date_str: &str) -> Result<CustomNaiveDate::CustomDate, ParseDateError> {
    let months = HashMap::from([
        ("Jan", 1),
        ("Feb", 2),
        ("Mar", 3),
        ("Apr", 4),
        ("Mei", 5),
        ("Jun", 6),
        ("Jul", 7),
        ("Agt", 8),
        ("Sep", 9),
        ("Okt", 10),
        ("Nov", 11),
        ("Des", 12),
    ]);

    let parts: Vec<&str> = date_str.split_whitespace().collect();
    if parts.len() != 3 {
        return Err(ParseDateError::NotEnough);
    }

    let day: u32 = parts[0].parse().map_err(|_| ParseDateError::Impossible)?;
    let month: u32 = *months.get(parts[1]).ok_or(ParseDateError::Impossible)?;
    let year: i32 = parts[2].parse().map_err(|_| ParseDateError::Impossible)?;

    CustomNaiveDate::CustomDate::from_ymd_opt(year, month, day).ok_or(ParseDateError::OutOfRange)
}

fn transform_to_i32(data: &calamine::Data) -> i32 {
    match data {
        calamine::Data::Float(f) => *f as i32,
        calamine::Data::Int(i) => *i as i32,
        _ => 0,
    }
}

#[allow(dead_code)]
fn calculate_macd(data: &[i32]) -> (Vec<f64>, Vec<f64>, Vec<Option<bool>>) {
    let mut macd_line = Vec::with_capacity(data.len());
    let mut signal_line = Vec::with_capacity(data.len());
    let mut crossover_signals = Vec::with_capacity(data.len());

    let mut ema_12 = VecDeque::with_capacity(12);
    let mut ema_26 = VecDeque::with_capacity(26);
    let mut ema_9 = VecDeque::with_capacity(9);
    let mut sma_7 = VecDeque::with_capacity(7);
    let mut sma_14 = VecDeque::with_capacity(14);
    let mut sma_200 = VecDeque::with_capacity(200);

    let mut prev_macd: Option<f64> = None;
    let mut prev_signal: Option<f64> = None;
    let mut _prev_signal_direction: Option<bool> = None; // true if signal line is going up, false if going down

    for price in data {
        // Calculate 7-period SMA
        sma_7.push_back(*price as f64);
        if sma_7.len() > 7 {
            sma_7.pop_front();
        }
        let sma_7_value = calculate_sma(&sma_7, 7);

        // Calculate 14-period SMA
        sma_14.push_back(*price as f64);
        if sma_14.len() > 14 {
            sma_14.pop_front();
        }
        let sma_14_value = calculate_sma(&sma_14, 14);

        // Calculate 200-period SMA
        sma_200.push_back(*price as f64);
        if sma_200.len() > 200 {
            sma_200.pop_front();
        }
        let sma_200_value = calculate_sma(&sma_200, 200);

        // Calculate 12-period EMA
        ema_12.push_back(*price as f64);
        if ema_12.len() > 12 {
            ema_12.pop_front();
        }
        let ema_12_value = calculate_ema(&ema_12, 12);

        // Calculate 26-period EMA
        ema_26.push_back(*price as f64);
        if ema_26.len() > 26 {
            ema_26.pop_front();
        }
        let ema_26_value = calculate_ema(&ema_26, 26);

        // Calculate MACD line
        let macd = ema_12_value - ema_26_value;
        macd_line.push(macd);

        // Calculate 9-period EMA of MACD line (signal line)
        ema_9.push_back(macd);
        if ema_9.len() > 9 {
            ema_9.pop_front();
        }
        let signal = calculate_ema(&ema_9, 9);
        signal_line.push(signal);

        // Detect MACD and signal line crossover
        let crossover_signal = match (prev_macd, prev_signal) {
            (Some(prev_macd), Some(prev_signal)) => {
                println!("{} >= {} >= {}", sma_7_value, sma_14_value, sma_200_value);
                if sma_7_value >= sma_14_value
                    && sma_14_value >= sma_200_value
                    && sma_7_value >= sma_200_value
                {
                    println!("SMA Positive");
                    Some(true) // Bullish crossover
                }
                // else if prev_macd <= prev_signal && macd > signal && (prev_signal_direction == Some(true) || prev_signal <= signal) && prev_macd <= macd && macd >= 0.0 && signal >= 0.0 {
                //     Some(true) // Bullish crossover
                // }
                else if prev_macd >= prev_signal && macd < signal {
                    Some(false) // Bearish crossover
                } else {
                    None // No crossover
                }
            }
            _ => None, // Not enough history to detect crossover
        };
        crossover_signals.push(crossover_signal);

        prev_macd = Some(macd);
        prev_signal = Some(signal);
        _prev_signal_direction = Some(signal >= prev_signal.unwrap_or(signal));
    }

    (macd_line, signal_line, crossover_signals)
}

#[allow(dead_code)]
fn calculate_sma(values: &VecDeque<f64>, period: usize) -> f64 {
    let mut sum = 0.0;

    // Calculate initial sum for first period values
    for val in values.iter() {
        sum += val;
    }

    sum / period as f64
}

#[allow(dead_code)]
fn calculate_ema(values: &VecDeque<f64>, period: usize) -> f64 {
    let multiplier = 2.0 / (period as f64 + 1.0);
    let mut ema = 0.0;

    for (i, value) in values.iter().enumerate() {
        ema = if i == 0 {
            *value
        } else {
            *value * multiplier + ema * (1.0 - multiplier)
        };
    }

    ema
}

fn extract_date(filename: &str) -> Option<(&str, &str, &str)> {
    let re = Regex::new(r"(\d{4})(\d{2})(\d{2})").unwrap(); // Pattern to match 8 digits and capture year, month, day
    if let Some(caps) = re.captures(filename) {
        let year = caps.get(1).map_or("", |m| m.as_str());
        let month = caps.get(2).map_or("", |m| m.as_str());
        let day = caps.get(3).map_or("", |m| m.as_str());
        return Some((year, month, day));
    }
    None
}

#[allow(dead_code)]
fn get_last_numeric(filename: &str) -> &str {
    // let re = Regex::new(r"\d+(?=\.\w+$)").unwrap();

    // Some(re.find(filename)?.as_str())
    let re = Regex::new(r"\d{8}").unwrap(); // Pattern to match 8 digits
    let mut date: &str = "";
    if let Some(caps) = re.captures(filename) {
        if let Some(date_match) = caps.get(0) {
            date = date_match.as_str();
            println!("Extracted date: {}", date);
        }
    }

    date
}

#[allow(dead_code)]
fn get_last_6_digits(filename: &str) -> Option<&str> {
    let len = filename.len();
    if len >= 6 {
        let (_, last_6) = filename.split_at(len - 6);
        Some(last_6)
    } else {
        None
    }
}

#[allow(dead_code)]
fn is_bearish(_index: &u32) -> bool {
    true
}

#[allow(dead_code)]
fn is_bullish(_index: &u32) -> bool {
    true
}