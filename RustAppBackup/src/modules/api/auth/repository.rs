use chrono::NaiveDate;
use regex::Regex;
use serde::{Serialize, Deserialize};
use std::{collections::HashMap, string::String};
use utoipa::{IntoParams, ToSchema};


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
    DateArray(Vec<NaiveDate>),
    StockArray(Vec<StockInfo>),
    StockEMAArray(Vec<StockEMA>),
    StockSMAArray(Vec<StockSMA>),
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


#[derive(Debug, Deserialize)]
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
        if let Some(bottom_price) = self.bottom_price {
            match validate_bottom_price(bottom_price) {
                Ok(_) => (),
                Err(e) => return Err(e),
            }
        }

        if let Some(date) = self.date.as_deref() {
            match validate_date(date) {
                Ok(_) => (),
                Err(e) => return Err(e),
            }
        }

        if let Some(offset) = self.offset {
            match validate_offset(offset) {
                Ok(_) => (),
                Err(e) => return Err(e),
            }
        }

        if let Some(limit) = self.limit {
            match validate_limit(limit) {
                Ok(_) => (),
                Err(e) => return Err(e),
            }
        }

        if let Some(stock_code) = &self.stock_code {
            match validate_stock_code(stock_code) {
                Ok(_) => (),
                Err(e) => return Err(e),
            }
        }

        Ok(())
    }
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct StockInfo {
    pub stock_name: String,
    // pub stock_price: i32,
    pub highest_price: i32,
    pub lowest_price: i32,
    pub average_price: i32,
    pub last_price: i32,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct StockEMA {
    pub stock_code: String,
    // pub stock_price: i32,
    // pub ema_9_value: i32,
    // pub ema_12_value: i32,
    // pub ema_26_value: i32,
    pub macd: i32,
    pub signal: i32,
    // pub lowest_price: i32,
    // pub highest_price: i32,
    pub trend: String,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct StockSMA {
    pub stock_code: String,
    pub stock_price: i32,
    pub sma_7_value: i32,
    pub sma_14_value: i32,
    pub sma_200_value: i32,
    pub lowest_price: i32,
    pub highest_price: i32,
    pub trend: String,
}

impl StockInfo {
    pub fn new(stock_name: String, highest_price: i32, lowest_price: i32, average_price: i32, last_price: i32) -> Self {
        StockInfo { stock_name, highest_price, lowest_price, average_price, last_price }
    }
}

impl StockEMA {
    pub fn new(stock_code: String, macd: i32, signal: i32, trend: String) -> Self {
        // StockEMA { stock_code, stock_price, ema_9_value, ema_12_value, ema_26_value, lowest_price, highest_price, trend }
        StockEMA { stock_code, macd, signal, trend }
    }
}

impl StockSMA {
    pub fn new(stock_code: String, stock_price: i32, sma_7_value: i32, sma_14_value: i32, sma_200_value: i32, lowest_price: i32, highest_price: i32, trend: String) -> Self {
        StockSMA { stock_code, stock_price, sma_7_value, sma_14_value, sma_200_value, lowest_price, highest_price, trend }
    }
}

#[allow(dead_code)]
#[derive(ToSchema)]
#[schema(example = "Ringkasan Saham-20240530.xlsx")]
pub struct UploadRequestBody {
    #[schema(value_type = String, format = Binary)]
    file: String,
}

fn validate_bottom_price(bottom_price: u32) -> Result<(), String> {
    if bottom_price <= 0 {
        return Err(format!("The bottom_price value {} must be greather than 0", bottom_price).to_string());
    }

    Ok(())
}

fn validate_date(date: &str) -> Result<(), String> {
    // Create a new Regex object
    let regex_date = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();

    if date.len() < 10 {
        return Err(format!("The date value {} length is too short", date).to_string());
    } else if date.len() > 10 {
        return Err(format!("The date value {} length is too long", date).to_string());
    } else if !regex_date.is_match(date) {
        return Err(format!("The date value {} format is not valid", date).to_string());
    }

    Ok(())
}

fn validate_offset(offset: i32) -> Result<(), String> {
    if offset < 0 {
        return Err(format!("The offset value {} must be greather than or equal 0", offset).to_string());
    }

    Ok(())
}

fn validate_limit(limit: i32) -> Result<(), String> {
    if limit <= 0 {
        return Err(format!("The limit value {} must be greather than 0", limit).to_string());
    }

    Ok(())
}

fn validate_stock_code(stock_code: &str) -> Result<(), String> {
    // Create a new Regex object
    let regex_stock_code = Regex::new(r"^[A-Z]{4}$").unwrap();

    if stock_code != "" && !regex_stock_code.is_match(&stock_code) {
        return Err(format!("The stock_code value {} format is not valid", stock_code).to_string());
    }

    Ok(())
}