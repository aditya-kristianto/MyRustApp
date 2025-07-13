use super::repository::QueryParams;
use super::repository::StockEMA;
use super::repository::DataValue;
use super::repository::Header;
use super::repository::Meta;
use super::repository::Response;
use actix_http::StatusCode;
use actix_web::web;
use actix_web::get;
use actix_web::web::ServiceConfig;
use actix_web::Error;
use actix_web::HttpResponse;
use actix_web::Responder;
use std::collections::HashMap;
use std::sync::Arc;
use tokio_postgres::Client;

pub fn configure_v2(config: &mut ServiceConfig) {
    config
    // .service(
    //     web::scope("/v2/stock").route("/macd", web::get().to(get_macd)),
    //     // Add more routes for v1 here
    // )
    .service(get_macd_stock);
}

/// Get Stock MACD
///
/// To get stock MACD
#[utoipa::path(
    get,
    context_path = "/v2",
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
#[get("/v2/stock/macd")]
pub async fn get_macd_stock(
    query: Result<web::Query<QueryParams>, Error>,
    data: web::Data<Arc<Client>>,
) -> impl Responder {
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

            return HttpResponse::BadRequest().json(web::Json(resp));
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

            return HttpResponse::BadRequest().json(web::Json(resp));
        }
    }

    // Access the database client from app_data
    let db_client = data.get_ref();
    let mut query_where_stock_code_str: String = "".to_string();
    let mut query_where_trend_str: String = "".to_string();

    if let Some(stock_code) = query_params.stock_code {
        if !stock_code.is_empty() {
            query_where_stock_code_str += &format!("t1.kode_saham = '{}' AND ", stock_code);
        }
    }

    if let Some(trend) = query_params.trend {
        if trend == 1 {
            query_where_trend_str += &format!("WHERE trend = 'Strong Buy'");
        } else if trend == 0 {
            query_where_trend_str += &format!("WHERE trend = 'Neutral'");
        } else if trend == -1 {
            query_where_trend_str += &format!("WHERE trend = 'Strong Sell'");
        }
    }

    let query_str: String = format!(
        "
        WITH RECURSIVE trading_days AS (
            SELECT
                t1.kode_saham,
                t1.nama_perusahaan,
                t1.tanggal_perdagangan_terakhir,
                t1.penutupan,
                ROW_NUMBER() OVER (PARTITION BY t1.kode_saham ORDER BY t1.tanggal_perdagangan_terakhir) AS seq
            FROM
                transactions t1
            WHERE
                {}
                t1.tanggal_perdagangan_terakhir BETWEEN (SELECT MAX(tanggal_perdagangan_terakhir) - INTERVAL '199 days' FROM transactions)
                AND (SELECT MAX(tanggal_perdagangan_terakhir) FROM transactions)
        ),
        ema_12_cte AS (
            SELECT
                ad1.kode_saham,
                ad1.nama_perusahaan,
                ad1.tanggal_perdagangan_terakhir,
                ad1.penutupan,
                ad1.penutupan AS ema_12,
                ad1.seq
            FROM
                trading_days ad1
            WHERE
                ad1.seq = 1
            UNION ALL
            SELECT
                ad2.kode_saham,
                ad2.nama_perusahaan,
                ad2.tanggal_perdagangan_terakhir,
                ad2.penutupan,
                CAST(ROUND((ad2.penutupan * (2.0 / (12 + 1))) + (ema_12_cte.ema_12 * (1 - (2.0 / (12 + 1))))) AS INTEGER) AS ema_12,
                ad2.seq
            FROM
                trading_days ad2
                JOIN ema_12_cte ON ad2.kode_saham = ema_12_cte.kode_saham
                AND ad2.seq = ema_12_cte.seq + 1
        ),
        ema_26_cte AS (
            SELECT
                ad1.kode_saham,
                ad1.nama_perusahaan,
                ad1.tanggal_perdagangan_terakhir,
                ad1.penutupan,
                ad1.penutupan AS ema_26,
                ad1.seq
            FROM
                trading_days ad1
            WHERE
                ad1.seq = 1
            UNION ALL
            SELECT
                ad2.kode_saham,
                ad2.nama_perusahaan,
                ad2.tanggal_perdagangan_terakhir,
                ad2.penutupan,
                CAST(ROUND((ad2.penutupan * (2.0 / (26 + 1))) + (ema_26_cte.ema_26 * (1 - (2.0 / (26 + 1))))) AS INTEGER) AS ema_26,
                ad2.seq
            FROM
                trading_days ad2
                JOIN ema_26_cte ON ad2.kode_saham = ema_26_cte.kode_saham
                AND ad2.seq = ema_26_cte.seq + 1
        ),
        macd_cte AS (
            SELECT
                e12.kode_saham,
                e12.nama_perusahaan,
                e12.tanggal_perdagangan_terakhir,
                e12.ema_12 - e26.ema_26 AS macd,
                e12.seq
            FROM
                ema_12_cte e12
                JOIN ema_26_cte e26 ON e12.kode_saham = e26.kode_saham
                AND e12.tanggal_perdagangan_terakhir = e26.tanggal_perdagangan_terakhir
        ),
        signal_cte AS (
            SELECT
                m1.kode_saham,
                m1.nama_perusahaan,
                m1.tanggal_perdagangan_terakhir,
                m1.macd,
                m1.macd AS signal,
                m1.seq
            FROM
                macd_cte m1
            WHERE
                m1.seq = 1
            UNION ALL
            SELECT
                m2.kode_saham,
                m2.nama_perusahaan,
                m2.tanggal_perdagangan_terakhir,
                m2.macd,
                CAST(ROUND((m2.macd * (2.0 / (9 + 1))) + (signal_cte.signal * (1 - (2.0 / (9 + 1))))) AS INTEGER) AS signal,
                m2.seq
            FROM
                macd_cte m2
                JOIN signal_cte ON m2.kode_saham = signal_cte.kode_saham
                AND m2.seq = signal_cte.seq + 1
        ),
        combine_cte AS (
            SELECT
                m.kode_saham,
                m.nama_perusahaan,
                m.macd,
                s.signal,
                CASE
                    WHEN m.macd > s.signal THEN 'Strong Buy'
                    WHEN m.macd < s.signal THEN 'Strong Sell'
                    ELSE 'Neutral'
                END AS trend
            FROM
                macd_cte m
                JOIN signal_cte s ON m.kode_saham = s.kode_saham
                AND m.tanggal_perdagangan_terakhir = s.tanggal_perdagangan_terakhir
            WHERE
                m.macd > 0
                AND s.signal > 0
                AND m.tanggal_perdagangan_terakhir = (SELECT MAX(tanggal_perdagangan_terakhir) FROM macd_cte)
            ORDER BY
                m.kode_saham
        )
        SELECT
            *
        FROM 
            combine_cte
        {};
    ",
    query_where_stock_code_str,
    query_where_trend_str
    );

    println!("{}", query_str);
    // Execute a SELECT query
    let rows = db_client
        .query(&query_str, &[])
        .await
        .map_err(|e| {
            eprintln!("Error executing query: {:?}", e);
            // actix_web::Error::from(e)

            Response::new(
                None,
                None,
                Some("".to_string()),
                None,
                StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            );

            // Ok(HttpResponse::InternalServerError().json(web::Json(resp)))
        })
        .unwrap();

    // Convert the rows to a vector of tuples
    let data: Vec<StockEMA> = rows
        .iter()
        .map(|row| {
            StockEMA::new(
                row.get(0),
                row.get(1),
                row.get(2),
                row.get(3),
                row.get(4)
            )
        })
        .collect();

    let resp = Response::new(
        Some(HashMap::from([(
            "stocks".to_string(),
            DataValue::StockEMAArray(data),
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

    HttpResponse::Ok().json(web::Json(resp))
}
