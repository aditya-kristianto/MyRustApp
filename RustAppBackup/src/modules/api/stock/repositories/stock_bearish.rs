use serde::{Serialize, Deserialize};
use utoipa::{ToSchema};

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct BearishStock {
    rank: i64,
    kode_saham: String,
    nama_perusahaan: String,
    open_price: i32,
    penutupan: i32,
    tanggal_perdagangan_terakhir: String, // Adjust type as needed
    bearish_value: i32,
    bearish_percentage: f64,
}

impl BearishStock {
    pub fn new(rank: i64, kode_saham: String, nama_perusahaan: String, open_price: i32, penutupan: i32, tanggal_perdagangan_terakhir: String, bearish_value: i32, bearish_percentage: f64) -> Self {
        BearishStock { rank, kode_saham, nama_perusahaan, open_price, penutupan, tanggal_perdagangan_terakhir, bearish_value, bearish_percentage }
    }
}