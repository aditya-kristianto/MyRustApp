use actix_web::middleware::{Compress, ErrorHandlers, Logger};
use actix_web::{HttpRequest};
use actix_web::{http::StatusCode, App, HttpServer};
use actix_web::web::{self, Data};
use stock::repository::DataValue;
use stock::repository::Error;
use stock::repository::Meta;
use stock::repository::Response;
use stock::dotenv;
use stock::middleware;
use stock::postgres;
use stock::swagger;
use stock::repository::BearishStock;
use stock::repository::BullishStock;
use stock::repository::FrequencyStock;
use stock::repository::MACDStock;
use stock::repository::RSIStock;
use stock::repository::StockEMA;
use stock::repository::StockInfo;
use stock::repository::StockSMA;
use stock::repository::SummaryStock;
use stock::repository::UploadRequestBody;
use tokio_postgres::Client;
use std::env;
use std::sync::Arc;
use utoipa::openapi::security::{SecurityScheme, ApiKey, ApiKeyValue};
use utoipa::{OpenApi, Modify};
use utoipa_swagger_ui::SwaggerUi;

#[path = "../../oauth/authorize.rs"] mod authorize;
#[path = "../../oauth/clients.rs"] mod clients;
#[path = "../../../pkg/date/naive_date.rs"] mod CustomNaiveDate;
#[path = "../../modules/api/stock/mod.rs"] pub mod stock;


#[derive(OpenApi)]
#[openapi(
    info(
        title = "My Rust Stock API",
    ),
    paths(
        stock::route::healthcheck,
        stock::v1::add_stock,
        stock::v1::add_pemantauan_khusus_stock,
        stock::v1::delete_stock,
        stock::v1::get_bearish_stock,
        stock::v1::get_bullish_stock,
        stock::v1::get_macd_stock,
        stock::v1::get_pemantauan_khusus_stock,
        stock::v1::get_rsi_stock,
        stock::v1::get_sma_stock,
        stock::v1::get_stocks,
        stock::v1::get_stock_rank_by_frequency,
        stock::v1::get_stock_summary,
        stock::v2::get_macd_stock,
    ), components(schemas(
        BearishStock::BearishStock,
        BullishStock::BullishStock,
        FrequencyStock,
        DataValue,
        Error,
        Meta,
        CustomNaiveDate::CustomDate,
        MACDStock,
        Response,
        RSIStock,
        StockInfo,
        StockEMA,
        StockSMA,
        SummaryStock,
        UploadRequestBody,
    )
), 
modifiers(&SecurityAddon),
tags(
    (
        name = "Stock",
        description = "Stock API",
        external_docs(url = "https://adityakristianto.com/", description = "Find out more")
    )
), external_docs(url = "https://adityakristianto.com/", description = "Find out more"))]
struct ApiDoc;
struct SecurityAddon;

/// Define a message type for TCP connections
// struct TcpConnection(pub TcpStream);

// impl Actor for TcpServer {
//     type Context = Context<Self>;
// }

// impl Message for TcpConnection {
//     type Result = ();
// }

// impl Handler<TcpConnection> for TcpServer {
//     type Result = ();

//     fn handle(&mut self, msg: TcpConnection, _: &mut Self::Context) {
//         let mut stream = msg.0;
//         let mut buffer = [0; 1024];
//         let _ = stream.read(&mut buffer);
//         let response = "Hello from TCP server!\n";
//         let _ = stream.write_all(response.as_bytes());
//     }
// }

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi.components.as_mut();
        if ! components.is_none() {
            let component = components.unwrap(); // we can unwrap safely since there already is components registered.
            component.add_security_scheme(
                "apiKey",
                SecurityScheme::ApiKey(ApiKey::Header(ApiKeyValue::new(middleware::_X_API_KEY_HEADER))),
            );
            component.add_security_scheme(
                "oauth2",
                SecurityScheme::ApiKey(ApiKey::Header(ApiKeyValue::new(actix_web::http::header::AUTHORIZATION.to_string())))
            );
        }
        
        // let components = openapi.components.as_mut().unwrap(); // we can unwrap safely since there already is components registered.
        // components.add_security_scheme(
        //     "apiKey",
        //     SecurityScheme::ApiKey(ApiKey::Header(ApiKeyValue::new(middleware::X_API_KEY_HEADER))),
        // );
        // components.add_security_scheme(
        //     "oauth2",
        //     SecurityScheme::ApiKey(ApiKey::Header(ApiKeyValue::new(actix_web::http::header::AUTHORIZATION.to_string())))
        // );
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize the logger
    env_logger::init();
    dotenv::init().await;

    let template_type = env::var("TEMPLATE_TYPE").expect("");
    
    // Set the ASKAMA_TEMPLATE_DIR environment variable
    env::set_var("ASKAMA_TEMPLATE_DIR", format!("templates/{}/dist/", template_type));

    let (http_host, http_port) = dotenv::get_http_host_and_port("stock");
    let app_worker_count = dotenv::get_app_worker_count();
    // let db = mongodb::DB::init().await;
    // if db.as_ref().is_err() {
    //     print!("{:?}", db.as_ref().err())
    // }

    // for db_name in db.unwrap().client.list_database_names(None, None).await {
    //     println!("{:?}", db_name);
    // }
    
    // while let Ok(db_name) = db.as_ref().unwrap().client.list_database_names(None, None).await {
    //     println!("{:?}", db_name);
    // }

    // Generate the OpenAPI spec
    let mut openapi = ApiDoc::openapi();

    let _result = swagger::init(&mut openapi).await;
    let postgres_client_result = postgres::init().await;
    let postgres_client: Arc<Client>;

    match postgres_client_result {
        Ok(client) => {
            println!("Connected to the database");
            postgres_client = Arc::new(client);
        }
        Err(e) => {
            eprintln!("Failed to connect to the database at {}:{}. Error: {}", postgres::get_host(), postgres::get_port(), e);

            panic!("Unable to initialize the database client");
        }
    }

    stock::logger::http_start(http_host.as_str(), http_port).await;
    
    let postgres_client_arc = Arc::clone(&postgres_client);
    
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(postgres_client_arc.clone()))
            .service(swagger::get_swagger_json)
            .service(SwaggerUi::new("/swagger/{_:.*}").url("/docs/swagger.json", openapi.clone()))
            .default_service(
                web::route().to(|req: HttpRequest| async {
                    stock::default_route::handle_400(req).await.map_err(|e| actix_web::error::ErrorInternalServerError(e))
                })
            )
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .wrap(ErrorHandlers::new().handler(
                StatusCode::BAD_REQUEST, 
                middleware::add_error_header,
            ))
            .wrap(ErrorHandlers::new().handler(
                StatusCode::NOT_FOUND, 
                middleware::add_error_header,
            ))
            .wrap(ErrorHandlers::new().handler(
                StatusCode::INTERNAL_SERVER_ERROR,
                middleware::add_error_header,
            ))
            .wrap(Compress::default())
            .configure(authorize::configure())
            .configure(clients::configure())
            .configure(stock::default_route::configure())
            .configure(stock::route::configure())
            .configure(stock::configure())
    })
    .workers(app_worker_count)
    .bind((http_host.clone(), http_port))?
    .run()
    .await?;

    // Start TCP server in a separate thread
    // let tcp_listener = TcpListener::bind("127.0.0.1:9090").unwrap();
    // thread::spawn(move || {
    //     for stream in tcp_listener.incoming() {
    //         let stream = stream.unwrap();
    //         // Get address of the TcpServer actor and send TcpConnection message
    //         let addr = TcpServer.start();
    //         addr.do_send(TcpConnection(stream));
    //     }
    // });

    stock::logger::tcp_start(http_host.as_str(), http_port).await;

    Ok(())
}