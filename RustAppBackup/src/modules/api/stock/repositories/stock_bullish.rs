use serde::{Serialize, Deserialize};
use utoipa::{ToSchema};

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct BullishStock {
    rank: i64,
    kode_saham: String,
    nama_perusahaan: String,
    open_price: i32,
    penutupan: i32,
    tanggal_perdagangan_terakhir: String, // Adjust type as needed
    bullish_value: i32,
    bullish_percentage: f64,
}

impl BullishStock {
    pub fn new(rank: i64, kode_saham: String, nama_perusahaan: String, open_price: i32, penutupan: i32, tanggal_perdagangan_terakhir: String, bullish_value: i32, bullish_percentage: f64) -> Self {
        BullishStock { rank, kode_saham, nama_perusahaan, open_price, penutupan, tanggal_perdagangan_terakhir, bullish_value, bullish_percentage }
    }
}