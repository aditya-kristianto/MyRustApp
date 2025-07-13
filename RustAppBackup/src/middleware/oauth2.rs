use actix_http::body::None;
use actix_utils::future::{ready, Ready};
use actix_web::{FromRequest, HttpRequest, dev::Payload};

#[derive(Debug, Clone)]
pub struct BearerAuth();
pub struct AuthenticationError();

impl BearerAuth {
}

impl FromRequest for BearerAuth {
    type Future = Ready<Result<Self, Self::Error>>;
    // type Error = AuthenticationError;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> <Self as FromRequest>::Future {
    }
}
