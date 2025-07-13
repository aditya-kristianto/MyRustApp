use actix_service::Service;
use actix_service::Transform;
use actix_web::http::header::ContentType;
use actix_web::http::header::CONTENT_SECURITY_POLICY;
use actix_web::middleware::ErrorHandlerResponse;
use actix_web::{dev, dev::ServiceRequest, dev::ServiceResponse, http::header, Error};
use actix_web_httpauth::extractors::basic::BasicAuth;
use actix_web_httpauth::extractors::bearer::BearerAuth;
use futures_util::future::LocalBoxFuture;
use std::rc::Rc;
use std::future::{ready, Ready};


pub const _X_API_KEY_HEADER: &str = "X-API-Key";

// Define the middleware struct
pub struct ContentSecurityPolicy;

impl<S, B> Transform<S, ServiceRequest> for ContentSecurityPolicy
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = actix_web::Error;
    type Transform = ContentSecurityPolicyMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(ContentSecurityPolicyMiddleware {
            service: Rc::new(service),
        }))
    }
}

pub struct ContentSecurityPolicyMiddleware<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for ContentSecurityPolicyMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = actix_web::Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&self, ctx: &mut std::task::Context<'_>) -> std::task::Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let fut = self.service.call(req);
        Box::pin(async move {
            let mut res = fut.await?;

            // "object-src 'none'; script-src 'self' 'sha256-lHepexR8gTB3d1EGJjSEqDeNklIOF09BUORIPqJexr4=' 'unsafe-hashes' https://assets.aditya-kristianto.com https://cdn.amcharts.com"
            res.headers_mut().insert(
                CONTENT_SECURITY_POLICY,
                    "script-src 'self' 'unsafe-inline' https://assets.aditya-kristianto.com https://cdn.amcharts.com"
                    .parse()
                    .unwrap(),
            );

            Ok(res)
        })
    }
}

#[allow(dead_code)]
pub fn add_error_header<B>(
    mut res: dev::ServiceResponse<B>,
) -> Result<ErrorHandlerResponse<B>, actix_web::Error> {
    let content_type = ContentType::json().to_string();

    res.response_mut().headers_mut().insert(
        header::CONTENT_TYPE,
        header::HeaderValue::from_str(&content_type)?,
    );
    
    Ok(ErrorHandlerResponse::Response(res.map_into_left_body()))
}

pub async fn _validator(
    req: ServiceRequest,
    _credentials: BasicAuth,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    println!("validator");
    Ok(req)
}

pub async fn _ok_validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    println!("{credentials:?}");

    Ok(req)
}

// pub async fn authorization_handler(req: HttpRequest) {
//     let auth_header = req.headers()
//         .get(http::header::AUTHORIZATION)
//         .and_then(|header| header.to_str().ok());
//
//     match auth_header {
//         Some(auth_header) if auth_header.eq("Basic Og==") => {
//             println!("authorization passed : {}", auth_header);
//         }
//         _ => {
//             // return HttpResponse::Unauthorized()
//             //     .content_type("application/json")
//             //     .body("{}")
//         }
//     }
// }
