use actix_http::StatusCode;
// use actix_rt::{JwtAuth, JwtPayload, JwtClaims};
use actix_web::delete;
use actix_web::get;
use actix_web::HttpResponse;
use actix_web::post;
use actix_web::put;
use actix_web::web;
use actix_web::web::ServiceConfig;
// use chrono::Duration;
use chrono::NaiveDateTime;
use chrono::Utc;
use mime::APPLICATION_JSON;
use rand::distributions::Alphanumeric;
use rand::SeedableRng;
use rand::Rng;
use rand_chacha::ChaCha20Rng;
use serde::Serialize;
use serde::Deserialize;
use std::sync::Arc;
use tokio_postgres::Client;
use uuid::Uuid;
use std::time::Duration;
// use rdkafka::producer::FutureProducer;
// use rdkafka::producer::FutureRecord;
// use rdkafka::util::Timeout;


#[path = "../../../../pkg/response/response.rs"] mod response;
#[path = "auth.rs"] mod model;
#[path = "../../../../pkg/db/db.rs"] mod db;

#[derive(Deserialize)]
struct SignInRequest {
    email: String,
    password: String,
}

#[derive(Serialize, Deserialize)]
struct User {
    user_id: String,
    name: String,
}

// pub struct JwtAuthentication;

// impl<JwtError> JwtAuth for JwtAuthentication {
//     fn validate_token(&self, token: &str) -> Result<JwtPayload, JwtError> {
//       // Validate JWT signature and expiration
//     //   Jwt::validate_token(token)
//     }
// }

pub fn configure_v1(config: &mut ServiceConfig) {
    config
        .service(authorize)
        .service(create_new_client)
        .service(delete_client)
        .service(get_all_clients)
        .service(sign_in)
        .service(sign_out)
        .service(token)
        .service(update_client);
}

/// Sign In
///
/// Sign In
#[utoipa::path(
post,
path = "/sign-in",
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
request_body = RequestBody,
params(
("Authorization", Header, description = "Authorization Token", example = "Bearer ..."),
("X-API-Key", Header, description = "Current X-API-Key of user", example = "123ASD"),
),
tag = "Auth"
)]
#[post("/sign-in")]
async fn sign_in(
    data: web::Data<Arc<Client>>, 
    json: web::Json<SignInRequest>
) -> HttpResponse {
    let _bootstrap_servers = "localhost:9092";
    // let producer: FutureProducer = rdkafka::config::ClientConfig::new()
    //     .set("bootstrap.servers", bootstrap_servers)
    //     .create()
    //     .expect("Failed to create producer");

    let _topic = "your_topic_name";
    let _message = "Hello, Kafka from Rust!";

    // produce_message(&producer, topic, message).await;
    // let google_client_id = ClientId::new(env::var("GOOGLE_CLIENT_ID").expect("").to_string());
    // let google_client_secret = ClientSecret::new(env::var("GOOGLE_CLIENT_SECRET").expect("").to_string());
    // let auth_url = AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".to_string())
    //     .expect("Invalid authorization endpoint URL");
    // let token_url = TokenUrl::new("https://www.googleapis.com/oauth2/v3/token".to_string())
    //     .expect("Invalid token endpoint URL");
    //
    // // Set up the config for the Google OAuth2 process.
    // let client = BasicClient::new(
    //     google_client_id,
    //     Some(google_client_secret),
    //     auth_url,
    //     Some(token_url),
    // )
    // // This example will be running its own server at localhost:8080.
    // // See below for the server implementation.
    // .set_redirect_uri(
    //     RedirectUrl::new("http://localhost:8080/api/sessions/oauth/google".to_string()).expect("Invalid redirect URL"),
    // )
    // // Google supports OAuth 2.0 Token Revocation (RFC-7009)
    // .set_revocation_uri(
    //     RevocationUrl::new("https://oauth2.googleapis.com/revoke".to_string())
    //         .expect("Invalid revocation endpoint URL"),
    // );
    //
    // // Google supports Proof Key for Code Exchange (PKCE - https://oauth.net/2/pkce/).
    //     // Create a PKCE code verifier and SHA-256 encode it as a code challenge.
    //     let (pkce_code_challenge, pkce_code_verifier) = PkceCodeChallenge::new_random_sha256();
    //
    //     // Generate the authorization URL to which we'll redirect the user.
    //     let (authorize_url, csrf_state) = client
    //         .authorize_url(CsrfToken::new_random)
    //         // This example is requesting access to the "calendar" features and the user's profile.
    //         .add_scope(Scope::new(
    //             "https://www.googleapis.com/auth/calendar".to_string(),
    //         ))
    //         .add_scope(Scope::new(
    //             "https://www.googleapis.com/auth/plus.me".to_string(),
    //         ))
    //         .set_pkce_challenge(pkce_code_challenge)
    //         .url();
    //
    //     println!(
    //         "Open this URL in your browser:\n{}\n",
    //         authorize_url.to_string()
    //     );
    //
    //     // A very naive implementation of the redirect server.
    //     let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    //     for stream in listener.incoming() {
    //         if let Ok(mut stream) = stream {
    //             let code;
    //             let state;
    //             {
    //                 let mut reader = BufReader::new(&stream);
    //
    //                 let mut request_line = String::new();
    //                 reader.read_line(&mut request_line).unwrap();
    //
    //                 let redirect_url = request_line.split_whitespace().nth(1).unwrap();
    //                 let url = Url::parse(&("http://localhost".to_string() + redirect_url)).unwrap();
    //
    //                 let code_pair = url
    //                     .query_pairs()
    //                     .find(|pair| {
    //                         let &(ref key, _) = pair;
    //                         key == "code"
    //                     })
    //                     .unwrap();
    //
    //                 let (_, value) = code_pair;
    //                 code = AuthorizationCode::new(value.into_owned());
    //
    //                 let state_pair = url
    //                     .query_pairs()
    //                     .find(|pair| {
    //                         let &(ref key, _) = pair;
    //                         key == "state"
    //                     })
    //                     .unwrap();
    //
    //                 let (_, value) = state_pair;
    //                 state = CsrfToken::new(value.into_owned());
    //             }
    //
    //             let message = "Go back to your terminal :)";
    //             let response = format!(
    //                 "HTTP/1.1 200 OK\r\ncontent-length: {}\r\n\r\n{}",
    //                 message.len(),
    //                 message
    //             );
    //             stream.write_all(response.as_bytes()).unwrap();
    //
    //             println!("Google returned the following code:\n{}\n", code.secret());
    //             println!(
    //                 "Google returned the following state:\n{} (expected `{}`)\n",
    //                 state.secret(),
    //                 csrf_state.secret()
    //             );
    //
    //             // Exchange the code with a token.
    //             let token_response = client
    //                 .exchange_code(code)
    //                 .set_pkce_verifier(pkce_code_verifier)
    //                 .request(http_client);
    //
    //             println!(
    //                 "Google returned the following token:\n{:?}\n",
    //                 token_response
    //             );
    //
    //             // Revoke the obtained token
    //             let token_response = token_response.unwrap();
    //             let token_to_revoke: StandardRevocableToken = match token_response.refresh_token() {
    //                 Some(token) => token.into(),
    //                 None => token_response.access_token().into(),
    //             };
    //
    //             client
    //                 .revoke_token(token_to_revoke)
    //                 .unwrap()
    //                 .request(http_client)
    //                 .expect("Failed to revoke token");
    //
    //             // The server will terminate itself after revoking the token.
    //             break;
    //         }
    //     }
    println!("sign in");
    let email_length = json.email.len();
    let password_length = json.password.len();
    println!("email : {}", json.email);
    println!("password : {}", json.password);
    println!("ok");
    let client = data.get_ref();

    // Define the SQL query
    let query = "SELECT user_id, username FROM users WHERE email = $1 AND password_hash = $2";
    let rows = client.query(query, &[&json.email, &json.password]).await.expect("Failed to execute query");

    for row in rows {
        // Access columns of the row using row.get(index) method
        let column_value: Uuid = row.get(0);
        println!("Column Value: {}", column_value);
    }

    // let users: Vec<User> = rows
    //     .iter()
    //     .map(|row| User {
    //         user_id: row.user_id,
    //         name: row.username,
    //     })
    //     .collect();

    if email_length >= 3 && password_length >= 6 {
        let response = response::JsonResponse{
            success: true,
            code: StatusCode::OK.as_u16(),
            message: Some(String::from("".to_string())),
            data: None,
        };

        // Serialize the struct to a JSON string
        let json_data = match serde_json::to_string(&response) {
            Ok(json_str) => json_str,
            Err(_) => {
                // Return an error response as JSON
                return HttpResponse::BadRequest()
                    .content_type(APPLICATION_JSON.to_string())
                    .body("Invalid JSON data");
            }

        };

        // Valid JSON data
        return HttpResponse::Ok()
            .content_type(APPLICATION_JSON.to_string())
            .body(json_data);
    }
    else {
        // Invalid JSON data
        return HttpResponse::BadRequest().body("Invalid JSON data");
    }
}

/// Sign Out
///
/// Sign Out
#[utoipa::path(
post,
path = "/sign-out",
responses(
(status = 100, description = "Continue", content_type = "application/json", body = Response),
(status = 101, description = "Switching Protocols", content_type = "application/json", body = Response),
(status = 103, description = "Early Hints", content_type = "application/json", body = Response),
(status = 200, description = "OK", content_type = "application/json", body = Response),
(status = 201, description = "Created", content_type = "application/json", body = Response),
(status = 202, description = "Accepted", content_type = "application/json", body = Response),
(status = 203, description = "Non-Authoritative Information", content_type = "application/json", body = Response),
(status = 204, description = "No Content", content_type = "application/json", body = Response),
(status = 205, description = "Reset Content", content_type = "application/json", body = Response),
(status = 206, description = "Partial Content", content_type = "application/json", body = Response),
(status = 300, description = "Multiple Choices", content_type = "application/json", body = Response),
(status = 301, description = "Moved Permanently", content_type = "application/json", body = Response),
(status = 302, description = "Found", content_type = "application/json", body = Response),
(status = 303, description = "See Other", content_type = "application/json", body = Response),
(status = 304, description = "Not Modified", content_type = "application/json", body = Response),
(status = 307, description = "Temporary Redirect", content_type = "application/json", body = Response),
(status = 308, description = "Permanent Redirect", content_type = "application/json", body = Response),
(status = 400, description = "Bad Request", content_type = "application/json", body = Response),
(status = 401, description = "Unauthorized", content_type = "application/json", body = Response),
(status = 402, description = "Payment Required", content_type = "application/json", body = Response),
(status = 403, description = "Forbidden", content_type = "application/json", body = Response),
(status = 404, description = "Not Found", content_type = "application/json", body = Response),
(status = 405, description = "Method Not Allowed", content_type = "application/json", body = Response),
(status = 406, description = "Not Acceptable", content_type = "application/json", body = Response),
(status = 407, description = "Proxy Authentication Required", content_type = "application/json", body = Response),
(status = 408, description = "Request Timeout", content_type = "application/json", body = Response),
(status = 409, description = "Conflict", content_type = "application/json", body = Response),
(status = 410, description = "Gone", content_type = "application/json", body = Response),
(status = 411, description = "Length Required", content_type = "application/json", body = Response),
(status = 412, description = "Precondition Failed", content_type = "application/json", body = Response),
(status = 413, description = "Payload Too Large", content_type = "application/json", body = Response),
(status = 414, description = "URI Too Long", content_type = "application/json", body = Response),
(status = 415, description = "Unsupported Media Type", content_type = "application/json", body = Response),
(status = 416, description = "Range Not Satisfiable", content_type = "application/json", body = Response),
(status = 417, description = "Expectation Failed", content_type = "application/json", body = Response),
(status = 418, description = "I'm a teapot", content_type = "application/json", body = Response),
(status = 422, description = "Unprocessable Entity", content_type = "application/json", body = Response),
(status = 425, description = "Too Early", content_type = "application/json", body = Response),
(status = 426, description = "Upgrade Required", content_type = "application/json", body = Response),
(status = 428, description = "Precondition Required", content_type = "application/json", body = Response),
(status = 429, description = "Too Many Requests", content_type = "application/json", body = Response),
(status = 431, description = "Request Header Fields Too Large", content_type = "application/json", body = Response),
(status = 451, description = "Unavailable For Legal Reasons", content_type = "application/json", body = Response),
(status = 500, description = "Internal Server Error", content_type = "application/json", body = Response),
(status = 501, description = "Not Implemented", content_type = "application/json", body = Response),
(status = 502, description = "Bad Gateway", content_type = "application/json", body = Response),
(status = 503, description = "Service Unavailable", content_type = "application/json", body = Response),
(status = 504, description = "Gateway Timeout", content_type = "application/json", body = Response),
(status = 505, description = "HTTP Version Not Supported", content_type = "application/json", body = Response),
(status = 506, description = "Variant Also Negotiates", content_type = "application/json", body = Response),
(status = 507, description = "Insufficient Storage", content_type = "application/json", body = Response),
(status = 508, description = "Loop Detected", content_type = "application/json", body = Response),
(status = 510, description = "Not Extended", content_type = "application/json", body = Response),
(status = 511, description = "Network Authentication Required", content_type = "application/json", body = Response),
),
params(
("id" = u64, Path, description = "Pet database id to get Pet for"),
),
request_body = RequestBody,
tag = "Auth"
)]
#[post("/sign-out")]
pub async fn sign_out(data: web::Data<Arc<Client>>) -> HttpResponse {
    // Access the database client from app_data
    let db_client = data.get_ref();

    let me = model::Person {
        id: 0,
        name: "Steven".to_string(),
        data: Some("None".to_string()),
    };
    let _ = db_client.execute("INSERT INTO person (id, name, data) VALUES ($1, $2, $3)",
                   &[&me.id, &me.name, &me.data]).await;

    let response = response::JsonResponse::new(true, StatusCode::OK.as_u16(), Some(String::from("You have successfully sign out".to_string())), None);

    // Serialize the struct to a JSON string
    let json_data = match serde_json::to_string(&response) {
        Ok(json_str) => json_str,
        Err(_) => {
            // Return an error response as JSON
            return HttpResponse::BadRequest()
                .content_type(APPLICATION_JSON.to_string())
                .body("Invalid JSON data");
        }
    };

    // Valid JSON data
    return HttpResponse::Ok()
        .content_type(APPLICATION_JSON.to_string())
        .body(json_data);
}

// async fn produce_message(producer: &FutureProducer, topic: &str, message: &str) {
//     println!("Produce Message");
//     let record = FutureRecord::to(topic)
//         .payload(message)
//         .key("some_key") // optional key
//         .partition(0);   // optional partition

//         match producer.send(record, Timeout::After(Duration::from_secs(1))).await {
//             Ok((partition, offset)) => {
//                 println!("Message sent successfully to partition {}, offset {}", partition, offset);
//             }
//             Err((error, _message)) => {
//                 eprintln!("Error sending message: {:?}", error);
//             }
//         }        
// }

#[get("/authorize")]
pub async fn authorize(_data: web::Data<Arc<Client>>) -> HttpResponse {
    return HttpResponse::Ok()
        .content_type(APPLICATION_JSON.to_string())
        .body("Invalid JSON data");
}

#[get("/oauth/token")]
pub async fn token(data: web::Data<Arc<Client>>, _json: web::Json<model::OauthTokenRequest>) -> HttpResponse {
    let token_length = 32; // Adjust the length as needed
    let access_token = generate_random_token(token_length);
    let refresh_token = generate_random_token(token_length);

    let db_client = data.get_ref();

    let parsed_client_id =  Uuid::parse_str("6405f4ad-e390-4f82-aac6-3b0bc51d56c7")
        .map(|uuid| uuid)
        .unwrap_or_else(|e| {
            eprintln!("Error parsing UUID: {}", e);
            // Handle the error case, e.g., provide a default UUID
            Uuid::nil()
        });

    let parsed_user_id =  Uuid::parse_str("b6cf4166-6014-4b63-bb1b-24be69db037d")
        .map(|uuid| uuid)
        .unwrap_or_else(|e| {
            eprintln!("Error parsing UUID: {}", e);
            // Handle the error case, e.g., provide a default UUID
            Uuid::nil()
        });

    let token_value = access_token.clone();
    let expires_in: u16 = 3600;
    println!("token value: {}", token_value);

    let data = model::AccessToken {
        client_id: Uuid::new_v4(),
        user_id: Uuid::new_v4(),
        token_value: token_value,
        scope: "None".to_string(),
        expires_at: Utc::now() + (Duration::from_secs((expires_in as i64).try_into().unwrap())),
        created_at: Utc::now(),
        created_by: Uuid::new_v4(),
        updated_at: None,
        updated_by: None,
        deleted_at: None,
        deleted_by: None,
    };

    // Convert DateTime<Utc> to NaiveDateTime
    let expires_at_naive: NaiveDateTime = data.expires_at.naive_utc();
    let created_at_naive: NaiveDateTime = data.created_at.naive_utc();

    // Define the SQL query
    let query = "INSERT INTO access_tokens (client_id, user_id, token_value, scope, expires_at, created_at, created_by) VALUES ($1, $2, $3, $4, $5, $6, $7)";

    // Prepare the query
    db_client.prepare(query).await.expect("Error preparing query");

    // Execute the prepared query with actual values
    let _ = db_client
        .execute(query, &[
            &parsed_client_id,
            &parsed_user_id,
            &data.token_value,
            &data.scope,
            &expires_at_naive,
            &created_at_naive,
            &parsed_user_id
        ])
        .await
        .expect("Error executing query");

    let response_json = model::AccessTokenResponse {
        access_token: access_token,
        token_type: "Bearer".to_string(),
        expires_in: expires_in,
        refresh_token: refresh_token,
    };

    // Attempt to serialize the struct to a JSON string
    let result: Result<String, _> = serde_json::to_string(&response_json);

    return match result {
        Ok(json_string) => {
            HttpResponse::Ok()
                .content_type(APPLICATION_JSON.to_string())
                .body(json_string)
        },
        Err(error) => {
            HttpResponse::InternalServerError()
                .content_type(APPLICATION_JSON.to_string())
                .body(format!("Error serializing JSON: {}", error))
        }
    };
}

#[get("/oauth/clients")]
pub async fn get_all_clients(data: web::Data<Arc<Client>>) -> HttpResponse {
    let db_client = data.get_ref();
    // Perform a select query
    let rows = db_client
        .query("SELECT client_id, client_secret, client_name, redirect_uri FROM clients", &[])
        .await
        .expect("Error executing query");
    let mut data = Vec::new();
    let total_items = rows.len() as u16;

    // Process the results
    for row in rows {
        data.push(model::ClientData {
            client_id: row.get(0),
            client_secret: row.get(1),
            client_name: row.get(2),
            redirect_uri: row.get(3),
        });
        println!("loop");
    }

    let response = model::ClientResponse {
        data: data,
        meta: model::MetaResponse {
            total_items: total_items,
            items_per_page: 10,
            current_page: 1,
            prev_page: None,
            next_page: "next_page".to_string(),
        }
    };

    // Serialize the struct to a JSON string
    let json_data = match serde_json::to_string(&response) {
        Ok(json_str) => json_str,
        Err(_) => {
            // Return an error response as JSON
            return HttpResponse::BadRequest()
                .content_type(APPLICATION_JSON.to_string())
                .body("Invalid JSON data");
        }
    };

    HttpResponse::Ok()
        .content_type(APPLICATION_JSON.to_string())
        .body(json_data)
}

#[post("/oauth/clients")]
pub async fn create_new_client(_data: web::Data<Arc<Client>>, _json: web::Json<model::CreateClientRequest>) -> HttpResponse {
    let _client = model::Client{
        _client_id: Uuid::new_v4(),
        _client_secret: "".to_string(),
        _client_name: "".to_string(),
        _redirect_uri: "".to_string(),
        _created_at: Utc::now(),
        _created_by: Uuid::new_v4(),
        _updated_at: None,
        _updated_by: None,
        _deleted_at: None,
        _deleted_by: None,
    };

    HttpResponse::Ok()
        .content_type(APPLICATION_JSON.to_string())
        .body("")
}

#[put("/oauth/clients/{client_id}")]
pub async fn update_client(_data: web::Data<Arc<Client>>, _json: web::Json<model::UpdateClientRequest>) -> HttpResponse {
    HttpResponse::Ok()
        .content_type(APPLICATION_JSON.to_string())
        .body("")
}

#[delete("/oauth/clients/{client_id}")]
pub async fn delete_client(_data: web::Data<Arc<Client>>) -> HttpResponse {
    HttpResponse::Ok()
        .content_type(APPLICATION_JSON.to_string())
        .body("")
}

fn generate_random_token(length: usize) -> String {
    let rng = ChaCha20Rng::from_entropy();

    let random_token: String = rng
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect();

    random_token
}