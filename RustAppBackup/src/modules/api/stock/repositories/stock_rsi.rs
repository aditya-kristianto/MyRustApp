use serde::{Serialize, Deserialize};
use utoipa::{ToSchema};

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