use serde::{Serialize, Deserialize};
use utoipa::{ToSchema};

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