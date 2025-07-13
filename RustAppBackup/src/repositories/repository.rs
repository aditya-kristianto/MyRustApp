use serde::{Serialize, Deserialize};
use std::{collections::HashMap, string::String};
use utoipa::{IntoParams, ToSchema};

#[path = "../../pkg/date/naive_date.rs"] pub mod CustomNaiveDate;

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
    pub authorization: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(untagged)]
pub enum DataValue {
    String(String),
    StringArray(Vec<String>),
    DateArray(Vec<CustomNaiveDate::CustomDate>)
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[schema(example = json!({"status": 200, "message": "OK"}))]
pub struct Response {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<HashMap<String, DataValue>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<Vec<Error>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<Meta>,
    #[serde(default = "default_status")]
    pub status: u16,
}

#[allow(dead_code)]
impl Response {
    pub fn new(data: Option<HashMap<String, DataValue>>, error: Option<Vec<Error>>, message: Option<String>, meta: Option<Meta>, status: u16) -> Self {
        Response {
            data: data,
            error: error,
            message: message,
            meta: meta,
            status: status,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Meta {
    pub count: u8,
    pub limit: u8,
    pub offset: u8,
}

fn default_status() -> u16 {
    200
}

#[allow(dead_code)]
fn default_message() -> String {
    "/".to_string()
}
#[derive(Clone, Debug, Deserialize)]
pub struct QueryParams {
    pub bottom_price: Option<u32>,
    pub date: Option<String>,
    pub field: Option<String>,
    pub offset: Option<i32>,
    pub limit: Option<i32>,
    pub stock_code: Option<String>,
    pub trend: Option<i32>
}

impl QueryParams {
    #[allow(dead_code)]
    pub fn new(bottom_price: Option<u32>, date: Option<String>, field: Option<String>, offset: Option<i32>, limit: Option<i32>, stock_code: Option<String>, trend: Option<i32>) -> Result<QueryParams, String> {
        if date.is_none() && field.is_none() && offset.is_none() && limit.is_none() {
            return Err("At least one query parameter must be provided".to_string());
        } else if date.clone().expect("").len() < 10 || date.clone().expect("").len() > 10 {
            return Err("Invalid date format".to_string());
        }
        
        Ok(QueryParams { bottom_price, date, field, offset, limit, stock_code, trend })
    }

    pub fn validate(&self) -> Result<(), String> {
        // if let Some(bottom_price) = self.bottom_price {
        //     match validate_bottom_price(bottom_price) {
        //         Ok(_) => (),
        //         Err(e) => return Err(e),
        //     }
        // }

        // if let Some(date) = self.date.as_deref() {
        //     match validate_date(date) {
        //         Ok(_) => (),
        //         Err(e) => return Err(e),
        //     }
        // }

        // if let Some(offset) = self.offset {
        //     match validate_offset(offset) {
        //         Ok(_) => (),
        //         Err(e) => return Err(e),
        //     }
        // }

        // if let Some(limit) = self.limit {
        //     match validate_limit(limit) {
        //         Ok(_) => (),
        //         Err(e) => return Err(e),
        //     }
        // }

        // if let Some(stock_code) = &self.stock_code {
        //     match validate_stock_code(stock_code) {
        //         Ok(_) => (),
        //         Err(e) => return Err(e),
        //     }
        // }

        Ok(())
    }
}