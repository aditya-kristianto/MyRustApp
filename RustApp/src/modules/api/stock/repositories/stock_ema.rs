use serde::{Serialize, Deserialize};
use utoipa::{ToSchema};

impl StockEMA {
    pub fn new(stock_code: String, nama_perusahaan: String, macd: i32, signal: i32, trend: String) -> Self {
        // StockEMA { stock_code, stock_price, ema_9_value, ema_12_value, ema_26_value, lowest_price, highest_price, trend }
        StockEMA { stock_code, nama_perusahaan, macd, signal, trend }
    }
}