use actix_http::StatusCode;
use actix_web::HttpResponse;
use actix_web::web;
use regex::Regex;
use serde::{Serialize, Deserialize};
use std::{collections::HashMap, fmt, string::String};
use std::sync::Arc;
use tokio_postgres::Client;
use utoipa::{IntoParams, ToSchema};

#[path = "./repositories/stock_bearish.rs"] pub mod BearishStock;
#[path = "./repositories/stock_bullish.rs"] pub mod BullishStock;
#[path = "../../../../pkg/date/naive_date.rs"] pub mod CustomNaiveDate;
#[path = "./repositories/stock.rs"] pub mod Stock;


#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Error {
    pub key: String,
    pub value: String,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self) // or provide a custom message if needed
    }
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
    BearishStockArray(Vec<BearishStock::BearishStock>),
    BullishStockArray(Vec<BullishStock::BullishStock>),
    String(String),
    StringArray(Vec<String>),
    DateArray(Vec<CustomNaiveDate::CustomDate>),
    FrequencyStockArray(Vec<FrequencyStock>),
    MACDStockArray(Vec<MACDStock>),
    RSIStockArray(Vec<RSIStock>),
    StockArray(Vec<StockInfo>),
    StocksArray(Vec<Stock::Stock>),
    StockEMAArray(Vec<StockEMA>),
    StockSMAArray(Vec<StockSMA>),
    SummaryStockArray(Vec<SummaryStock>),
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
pub struct MACDStock {
    kode_saham: String,
    nama_perusahaan: String,
    open_price: i32,
    penutupan: i32,
    tanggal_perdagangan_terakhir: String, // Adjust type as needed
    macd_line: f64,
    signal_line: f64,
    macd_histogram: f64,
    trendline: String
}

impl MACDStock {
    pub fn new(kode_saham: String, nama_perusahaan: String, open_price: i32, penutupan: i32, tanggal_perdagangan_terakhir: String, macd_line: f64, signal_line: f64, macd_histogram: f64, trendline: String) -> Self {
        MACDStock { kode_saham, nama_perusahaan, open_price, penutupan, tanggal_perdagangan_terakhir, macd_line, signal_line, macd_histogram, trendline }
    }
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct StockEMA {
    pub stock_code: String,
    pub nama_perusahaan: String,
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
    pub nama_perusahaan: String,
    pub stock_price: i32,
    pub sma_7_value: i32,
    pub sma_14_value: i32,
    pub sma_200_value: i32,
    pub lowest_price: i32,
    pub highest_price: i32,
    pub trend: String,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct FrequencyStock {
    rank: i64,
    kode_saham: String,
    nama_perusahaan: String,
    open_price: i32,
    penutupan: i32,
    frekuensi: i32,
    tanggal_perdagangan_terakhir: String,
}

impl FrequencyStock {
    pub fn new(rank: i64, kode_saham: String, nama_perusahaan: String, open_price: i32, penutupan: i32, frekuensi: i32, tanggal_perdagangan_terakhir: String) -> Self {
        FrequencyStock { rank, kode_saham, nama_perusahaan, open_price, penutupan, frekuensi, tanggal_perdagangan_terakhir }
    }
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct RSIStock {
    kode_saham: String,
    nama_perusahaan: String,
    open_price: i32, 
    penutupan: i32,
    tanggal_perdagangan_terakhir: String,
    rsi_6: f64,
    rsi_12: f64,
    rsi_24: f64,
    trendline: String,
}

impl RSIStock {
    pub fn new(kode_saham: String, nama_perusahaan: String, open_price: i32, penutupan: i32, tanggal_perdagangan_terakhir: String, rsi_6: f64, rsi_12: f64, rsi_24: f64, trendline: String) -> Self {
        RSIStock { kode_saham, nama_perusahaan, open_price, penutupan, tanggal_perdagangan_terakhir, rsi_6, rsi_12, rsi_24, trendline }
    }
}

impl StockInfo {
    pub fn new(stock_name: String, highest_price: i32, lowest_price: i32, average_price: i32, last_price: i32) -> Self {
        StockInfo { stock_name, highest_price, lowest_price, average_price, last_price }
    }
}

impl StockEMA {
    pub fn new(stock_code: String, nama_perusahaan: String, macd: i32, signal: i32, trend: String) -> Self {
        // StockEMA { stock_code, stock_price, ema_9_value, ema_12_value, ema_26_value, lowest_price, highest_price, trend }
        StockEMA { stock_code, nama_perusahaan, macd, signal, trend }
    }
}

impl StockSMA {
    pub fn new(stock_code: String, nama_perusahaan: String, stock_price: i32, sma_7_value: i32, sma_14_value: i32, sma_200_value: i32, lowest_price: i32, highest_price: i32, trend: String) -> Self {
        StockSMA { stock_code, nama_perusahaan, stock_price, sma_7_value, sma_14_value, sma_200_value, lowest_price, highest_price, trend }
    }
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct SummaryStock {
    kode_saham: String,
    nama_perusahaan: String,
    open_price: i32,
    penutupan: i32,
    tanggal_perdagangan_terakhir: String, // Adjust type as needed
    rsi_6: f64,
    rsi_12: f64,
    rsi_24: f64,
    macd_line: f64,
    signal_line: f64,
    macd_histogram: f64,
    macd_trendline: String,
    rsi_trendline: String,
    michael_harris_signal: String,
    ichimoku_trendline: String,
}

impl SummaryStock {
    pub fn new(kode_saham: String, nama_perusahaan: String, open_price: i32, penutupan: i32, tanggal_perdagangan_terakhir: String, rsi_6: f64, rsi_12: f64, rsi_24: f64, macd_line: f64, signal_line: f64, macd_histogram: f64, macd_trendline: String, rsi_trendline: String, michael_harris_signal: String, ichimoku_trendline: String) -> Self {
        SummaryStock { kode_saham, nama_perusahaan, open_price, penutupan, tanggal_perdagangan_terakhir, rsi_6, rsi_12, rsi_24, macd_line, signal_line, macd_histogram, macd_trendline, rsi_trendline, michael_harris_signal, ichimoku_trendline }
    }

    // Function to query the database and get a user by their ID
    pub async fn get_stocks_summary(data: web::Data<Arc<Client>>) -> Result<Vec<SummaryStock>, Error> {
        // Access the database client from app_data
        let db_client = data.get_ref();
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
            ema_calculations AS (
                SELECT 
                    r.kode_saham,
                    r.nama_perusahaan,
                    r.tanggal_perdagangan_terakhir,
                    r.penutupan,
                    -- EMA 12
                    ROUND(AVG(r.penutupan) OVER (PARTITION BY r.kode_saham ORDER BY r.tanggal_perdagangan_terakhir ROWS BETWEEN 11 PRECEDING AND CURRENT ROW), 2) AS ema_12,
                    -- EMA 26
                    ROUND(AVG(r.penutupan) OVER (PARTITION BY r.kode_saham ORDER BY r.tanggal_perdagangan_terakhir ROWS BETWEEN 25 PRECEDING AND CURRENT ROW), 2) AS ema_26
                FROM price_changes r
            ),
            macd_calculation AS (
                SELECT 
                    e.kode_saham,
                    e.nama_perusahaan,
                    e.tanggal_perdagangan_terakhir,
                    e.ema_12,
                    e.ema_26,
                    -- MACD Line
                    ROUND(e.ema_12 - e.ema_26, 2) AS macd_line,
                    -- Signal Line (9-period EMA of MACD)
                    ROUND(AVG(e.ema_12 - e.ema_26) OVER (PARTITION BY e.kode_saham ORDER BY e.tanggal_perdagangan_terakhir ROWS BETWEEN 8 PRECEDING AND CURRENT ROW), 2) AS signal_line
                FROM ema_calculations e
            ),
            michael_harris_precalculation AS (
                SELECT
                    r.kode_saham,
                    r.nama_perusahaan,
                    r.open_price,
                    r.penutupan,
                    r.tanggal_perdagangan_terakhir,
                    LAG(r.penutupan) OVER (PARTITION BY r.kode_saham ORDER BY r.tanggal_perdagangan_terakhir) AS prev_close1,
                    LAG(r.penutupan, 2) OVER (PARTITION BY r.kode_saham ORDER BY r.tanggal_perdagangan_terakhir) AS prev_close2,
                    LAG(r.penutupan, 3) OVER (PARTITION BY r.kode_saham ORDER BY r.tanggal_perdagangan_terakhir) AS prev_close3
                FROM price_changes r
            ),
            michael_harris_signals AS (
                SELECT
                    m.kode_saham,
                    m.nama_perusahaan,
                    m.open_price,
                    m.penutupan,
                    m.tanggal_perdagangan_terakhir,
                    CASE
                        -- Bullish Reversal
                        WHEN m.penutupan > m.prev_close1 AND m.prev_close1 > m.prev_close2 AND m.prev_close2 < m.prev_close3 THEN 'BUY'
                        -- Bearish Reversal
                        WHEN m.penutupan < m.prev_close1 AND m.prev_close1 < m.prev_close2 AND m.prev_close2 > m.prev_close3 THEN 'SELL'
                        ELSE 'HOLD'
                    END AS michael_harris_signal
                FROM michael_harris_precalculation m
            ),
            ichimoku_calculations AS (
                SELECT 
                    t.kode_saham,
                    t.nama_perusahaan,
                    t.tanggal_perdagangan_terakhir,
                    t.penutupan,
                    -- Tenkan-sen (9-period Conversion Line)
                    (MAX(t.penutupan) OVER (PARTITION BY t.kode_saham ORDER BY t.tanggal_perdagangan_terakhir ROWS BETWEEN 8 PRECEDING AND CURRENT ROW) +
                    MIN(t.penutupan) OVER (PARTITION BY t.kode_saham ORDER BY t.tanggal_perdagangan_terakhir ROWS BETWEEN 8 PRECEDING AND CURRENT ROW)) / 2 AS tenkan_sen,
                    -- Kijun-sen (26-period Base Line)
                    (MAX(t.penutupan) OVER (PARTITION BY t.kode_saham ORDER BY t.tanggal_perdagangan_terakhir ROWS BETWEEN 25 PRECEDING AND CURRENT ROW) +
                    MIN(t.penutupan) OVER (PARTITION BY t.kode_saham ORDER BY t.tanggal_perdagangan_terakhir ROWS BETWEEN 25 PRECEDING AND CURRENT ROW)) / 2 AS kijun_sen,
                    -- Senkou Span A (Leading Span A)
                    (MAX(t.penutupan) OVER (PARTITION BY t.kode_saham ORDER BY t.tanggal_perdagangan_terakhir ROWS BETWEEN 8 PRECEDING AND CURRENT ROW) +
                    MIN(t.penutupan) OVER (PARTITION BY t.kode_saham ORDER BY t.tanggal_perdagangan_terakhir ROWS BETWEEN 8 PRECEDING AND CURRENT ROW) +
                    MAX(t.penutupan) OVER (PARTITION BY t.kode_saham ORDER BY t.tanggal_perdagangan_terakhir ROWS BETWEEN 25 PRECEDING AND CURRENT ROW) +
                    MIN(t.penutupan) OVER (PARTITION BY t.kode_saham ORDER BY t.tanggal_perdagangan_terakhir ROWS BETWEEN 25 PRECEDING AND CURRENT ROW)) / 4 AS senkou_span_a,
                    -- Senkou Span B (Leading Span B)
                    (MAX(t.penutupan) OVER (PARTITION BY t.kode_saham ORDER BY t.tanggal_perdagangan_terakhir ROWS BETWEEN 51 PRECEDING AND CURRENT ROW) +
                    MIN(t.penutupan) OVER (PARTITION BY t.kode_saham ORDER BY t.tanggal_perdagangan_terakhir ROWS BETWEEN 51 PRECEDING AND CURRENT ROW)) / 2 AS senkou_span_b,
                    -- Chikou Span (Lagging Span)
                    LAG(t.penutupan, 26) OVER (PARTITION BY t.kode_saham ORDER BY t.tanggal_perdagangan_terakhir) AS chikou_span
                FROM transactions t
            ),
            trendline_signals AS (
                SELECT 
                    r.kode_saham,
                    r.nama_perusahaan,
                    r.open_price,
                    r.penutupan,
                    r.tanggal_perdagangan_terakhir,
                    r.rsi_6,
                    r.rsi_12,
                    r.rsi_24,
                    m.macd_line,
                    m.signal_line,
                    -- MACD Histogram
                    ROUND(m.macd_line - m.signal_line, 2) AS macd_histogram,
                    -- Trendline Signals based on MACD
                    CASE 
                        WHEN m.macd_line > m.signal_line THEN 'BUY'
                        WHEN m.macd_line < m.signal_line THEN 'SELL'
                        ELSE 'HOLD'
                    END AS macd_trendline,
                    -- Generate Trendline Signal
                    CASE 	
                        WHEN r.rsi_6 < 30 OR r.rsi_12 < 30 OR r.rsi_24 < 30 THEN 'BUY' -- Oversold condition
                        WHEN r.rsi_6 > 70 OR r.rsi_12 > 70 OR r.rsi_24 > 70 THEN 'SELL' -- Overbought condition
                        ELSE 'HOLD' -- No clear trend
                    END AS rsi_trendline,
                     -- Generate Ichimoku Trendline Signals
                    CASE 
                        WHEN i.tenkan_sen > i.kijun_sen AND i.penutupan > i.senkou_span_a AND i.penutupan > i.senkou_span_b THEN 'BUY'
                        WHEN i.tenkan_sen < i.kijun_sen AND i.penutupan < i.senkou_span_a AND i.penutupan < i.senkou_span_b THEN 'SELL'
                        ELSE 'HOLD'
                    END AS ichimoku_trendline,
                    mh.michael_harris_signal
                FROM rsi_calculation r
                JOIN macd_calculation m 
                    ON r.kode_saham = m.kode_saham 
                    AND r.tanggal_perdagangan_terakhir = m.tanggal_perdagangan_terakhir
                JOIN michael_harris_signals mh
                    ON r.kode_saham = mh.kode_saham 
                    AND r.tanggal_perdagangan_terakhir = mh.tanggal_perdagangan_terakhir
                JOIN ichimoku_calculations i
                    ON r.kode_saham = i.kode_saham 
                    AND r.tanggal_perdagangan_terakhir = i.tanggal_perdagangan_terakhir
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
                t.macd_line::DOUBLE PRECISION,
                t.signal_line::DOUBLE PRECISION,
                t.macd_histogram::DOUBLE PRECISION,
                t.macd_trendline,
                t.rsi_trendline,
                t.michael_harris_signal,
                t.ichimoku_trendline
            FROM trendline_signals t
            WHERE t.tanggal_perdagangan_terakhir = (
                SELECT MAX(tanggal_perdagangan_terakhir) 
                FROM transactions
            )
            ORDER BY t.michael_harris_signal ASC, t.kode_saham ASC;
            "
        );

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

        let data: Vec<SummaryStock> = rows
            .iter()
            .map(|row| {
                SummaryStock{
                    kode_saham: row.get(0),
                    nama_perusahaan: row.get(1),
                    open_price: row.get("open_price"),
                    penutupan: row.get("penutupan"),
                    tanggal_perdagangan_terakhir: row
                        .get::<&str, chrono::NaiveDate>("tanggal_perdagangan_terakhir")
                        .to_string(),
                    rsi_6: row.get("rsi_6"),
                    rsi_12: row.get("rsi_12"),
                    rsi_24: row.get("rsi_24"),
                    macd_line: row.get("macd_line"),
                    signal_line: row.get("signal_line"),
                    macd_histogram: row.get("macd_histogram"),
                    macd_trendline: row.get("macd_trendline"),
                    rsi_trendline: row.get("rsi_trendline"),
                    michael_harris_signal: row.get("michael_harris_signal"),
                    ichimoku_trendline: row.get("ichimoku_trendline")
                }
            })
            .collect();
    
        Ok(data)
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