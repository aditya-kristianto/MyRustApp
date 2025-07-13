use serde::{Serialize, Deserialize};
use utoipa::{ToSchema};

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

impl StockSMA {
    pub fn new(stock_code: String, nama_perusahaan: String, stock_price: i32, sma_7_value: i32, sma_14_value: i32, sma_200_value: i32, lowest_price: i32, highest_price: i32, trend: String) -> Self {
        StockSMA { stock_code, nama_perusahaan, stock_price, sma_7_value, sma_14_value, sma_200_value, lowest_price, highest_price, trend }
    }
}