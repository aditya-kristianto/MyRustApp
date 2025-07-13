use std::time::SystemTime;

use actix_web::HttpRequest;
use std::string::String;
use uuid::Uuid;

#[path = "../repositories/duuid.rs"]
pub mod model;
#[path = "./repository.rs"]
mod repository;

pub fn get_uuid(request: &HttpRequest) {
    println!("get_uuid");
    let uuid = Uuid::new_v4();
    let _data = model::DUUIDData {
        cloudfront_header: model::CloudFrontHeader {
            cloudfront_is_android_viewer: request
                .headers()
                .get("CloudFront-Is-Android-Viewer")
                .map(|x| x.to_string()),
            cloudfront_is_ios_viewer: request
                .headers()
                .get("CloudFront-Is-IOS-Viewer")
                .map(|x| x.to_string()),
            cloudfront_is_desktop_viewer: request
                .headers()
                .get("CloudFront-Is-Desktop-Viewer")
                .map(|x| x.to_string()),
            cloudfront_is_tablet_viewer: request
                .headers()
                .get("CloudFront-Is-Tablet-Viewer")
                .map(|x| x.to_string()),
            cloudfront_is_mobile_viewer: request
                .headers()
                .get("CloudFront-Is-Mobile-Viewer")
                .map(|x| x.to_string()),
            cloudfront_is_smarttv_viewer: request
                .headers()
                .get("CloudFront-Is-SmartTV-Viewer")
                .map(|x| x.to_string()),
            cloudfront_viewer_latitude: request
                .headers()
                .get("CloudFront-Viewer-Latitude")
                .map(|x| x.to_string()),
            cloudfront_viewer_longitude: request
                .headers()
                .get("CloudFront-Viewer-Longitude")
                .map(|x| x.to_string()),
            cloudfront_forwarded_proto: request
                .headers()
                .get("CloudFront-Forwarded-Proto")
                .map(|x| x.to_string()),
            cloudfront_viewer_tls: request
                .headers()
                .get("CloudFront-Viewer-TLS")
                .map(|x| x.to_string()),
            cloudfront_viewer_asn: request
                .headers()
                .get("CloudFront-Viewer-ASN")
                .map(|x| x.to_string()),
            cloudfront_viewer_country: request
                .headers()
                .get("Cloudfront-Viewer-Country")
                .map(|x| x.to_string()),
            cloudfront_viewer_country_name: request
                .headers()
                .get("CloudFront-Viewer-Country-Name")
                .map(|x| x.to_string()),
            cloudfront_viewer_country_region: request
                .headers()
                .get("CloudFront-Viewer-Country-Region")
                .map(|x| x.to_string()),
            cloudfront_viewer_country_region_name: request
                .headers()
                .get("CloudFront-Viewer-Country-Region-Name")
                .map(|x| x.to_string()),
            cloudfront_viewer_city: request
                .headers()
                .get("CloudFront-Viewer-City")
                .map(|x| x.to_string()),
            cloudfront_viewer_address: request
                .headers()
                .get("CloudFront-Viewer-Address")
                .map(|x| x.to_string()),
            cloudfront_viewer_postal_code: request
                .headers()
                .get("CloudFront-Viewer-Postal-Code")
                .map(|x| x.to_string()),
            cloudfront_viewer_metro_code: request
                .headers()
                .get("CloudFront-Viewer-Metro-Cod")
                .map(|x| x.to_string()),
            cloudfront_viewer_time_zone: request
                .headers()
                .get("CloudFront-Viewer-Time-Zone")
                .map(|x| x.to_string()),
            cloudfront_viewer_ja3_fingerprint: request
                .headers()
                .get("CloudFront-Viewer-JA3-Fingerprint")
                .map(|x| x.to_string()),
            cloudfront_viewer_header_order: request
                .headers()
                .get("CloudFront-Viewer-Header-Order")
                .map(|x| x.to_string()),
            cloudfront_viewer_http_version: request
                .headers()
                .get("CloudFront-Viewer-Http-Version")
                .map(|x| x.to_string()),
            cloudfront_viewer_header_count: request
                .headers()
                .get("CloudFront-Viewer-Header-Count")
                .map(|x| x.to_string()),
        },
        created_at: SystemTime::now(),
        origin: request.headers().get("origin").map(|x| x.to_string()),
        uuid: uuid.to_string(),
    };

    // let mut client_options = ClientOptions::parse(dotenv!("MONGODB_URI"));

    // // Manually set an option
    // client_options.app_name = Some(dotenv!("APP_NAME").to_string());

    // let client = Client::with_options(client_options)?;
    // // let new_doc = model::OauthAccessTokens {
    // //             id: "".to_string(),
    // //             user_id: "".to_string(),
    // //             client_id: "".to_string(),
    // //             name: new_user.name,
    // //             scopes: "".to_string(),
    // //             revoked: false,
    // //             created_at: SystemTime::now(),
    // //             updated_at: Some(SystemTime::now()),
    // //             expires_at: Some(SystemTime::now())
    // //         };
    // let database = client.database("rust");
    // let collection = database.collection("duuids");
    // let bson_document = to_document(&data)?;
    // let result = collection.insert_one(bson_document, None);
    // println!("result");
    // Ok(collection.insert_one(bson_document, None));
}

// impl MongoRepo {
// pub fn init() -> Self {
//     let uri = dotenv!("MONGODB_URI").to_string();
//     let client = Client::with_uri_str(uri).await.unwrap();
//     let db = client.database("rustDB");
//     let col: Collection<model::OauthAccessTokens> = db.collection("oauth_access_tokens");
//     MongoRepo { col }
// }

// pub fn create_user(new_user: model::OauthAccessTokens) -> Result<InsertOneResult, Error> {
//     let new_doc = model::OauthAccessTokens {
//         id: "".to_string(),
//         user_id: "".to_string(),
//         client_id: "".to_string(),
//         name: new_user.name,
//         scopes: "".to_string(),
//         revoked: false,
//         created_at: SystemTime::now(),
//         updated_at: Some(SystemTime::now()),
//         expires_at: Some(SystemTime::now())
//     };
// let user = self
//     .col
//     .insert_one(new_doc, None)
//     .ok()
//     .expect("Error creating user");
// Ok(user)
// }
// }

pub trait HeaderValueExt {
    fn to_string(&self) -> String;
}

impl HeaderValueExt for http::HeaderValue {
    fn to_string(&self) -> String {
        self.to_str().unwrap_or_default().to_string()
    }
}
