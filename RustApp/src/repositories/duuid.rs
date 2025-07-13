use std::time::SystemTime;

use serde::{Deserialize, Serialize};
use std::string::String;
// use std::vec;
use utoipa::{IntoParams, ToSchema};

#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CloudFrontHeader {
    pub cloudfront_is_android_viewer: Option<String>,
    pub cloudfront_is_ios_viewer: Option<String>,
    pub cloudfront_is_desktop_viewer: Option<String>,
    pub cloudfront_is_tablet_viewer: Option<String>,
    pub cloudfront_is_mobile_viewer: Option<String>,
    pub cloudfront_is_smarttv_viewer: Option<String>,
    pub cloudfront_viewer_latitude: Option<String>,
    pub cloudfront_viewer_longitude: Option<String>,
    pub cloudfront_forwarded_proto:Option<String>,
    pub cloudfront_viewer_tls: Option<String>,
    pub cloudfront_viewer_asn: Option<String>,
    pub cloudfront_viewer_country: Option<String>,
    pub cloudfront_viewer_country_name: Option<String>,
    pub cloudfront_viewer_country_region: Option<String>,
    pub cloudfront_viewer_country_region_name: Option<String>,
    pub cloudfront_viewer_city: Option<String>,
    pub cloudfront_viewer_address: Option<String>,
    pub cloudfront_viewer_postal_code: Option<String>,
    pub cloudfront_viewer_metro_code: Option<String>,
    pub cloudfront_viewer_time_zone: Option<String>,
    pub cloudfront_viewer_ja3_fingerprint: Option<String>,
    pub cloudfront_viewer_header_order: Option<String>,
    pub cloudfront_viewer_http_version: Option<String>,
    pub cloudfront_viewer_header_count: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct DUUID {
    pub uuid: String,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct DUUIDData {
    pub cloudfront_header: CloudFrontHeader,
    pub origin: Option<String>,
    pub uuid: String,
    pub created_at: SystemTime
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Error {
    pub key: String,
    pub value: String,
}

#[derive(Debug, Deserialize, IntoParams, Serialize, ToSchema)]
#[into_params(parameter_in = Header)]
#[schema(example = "Bearer")]
#[serde(rename_all = "PascalCase")]
pub struct Header {
    /// Authorization token header
    #[param(example = "Bearer 12345")]
    pub authorization: String
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Meta {
    pub count: u8,
    pub offset: u8,
    pub page: u8,
    pub limit: u8,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[schema(example = json!({"status": "OK", "message": "OK", "data": {"uuid": ""}}))]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub data: Option<DUUID>,
    pub error: Option<Vec<Error>>,
    pub message: Option<String>,
    pub meta: Option<Meta>,
    pub status: u16,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[schema(example = json!({"limit": "10", "search": "525bec45-a0cb-4c52-942e-82faa77d8558", "start": 1}))]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub limit: Option<u8>,
    pub search: Option<String>,
    pub start: Option<u8>,
}
