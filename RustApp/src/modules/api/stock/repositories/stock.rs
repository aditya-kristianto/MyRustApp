use actix_http::StatusCode;
use actix_web::HttpResponse;
use actix_web::web;
use crate::Error;
use crate::Response;
use serde::{Serialize, Deserialize};
use std::sync::Arc;
use tokio_postgres::Client;
use utoipa::{ToSchema};

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct Stock {
    kode_saham: String,
    nama_perusahaan: String,
    open_price: i32,
    penutupan: i32,
    tanggal_perdagangan_terakhir: String
}

impl Stock {
    pub fn new(kode_saham: String, nama_perusahaan: String, open_price: i32, penutupan: i32, tanggal_perdagangan_terakhir: String, ) -> Self {
        Stock { kode_saham, nama_perusahaan, open_price, penutupan, tanggal_perdagangan_terakhir }
    }

    // Function to query the database and get stocks
    pub async fn get_stocks(data: web::Data<Arc<Client>>) -> Result<Vec<Stock>, Error> {
        // Access the database client from app_data
        let db_client = data.get_ref();
        let query_str: String = format!(
            "
                SELECT t.kode_saham, t.nama_perusahaan, t.open_price, t.penutupan, t.tanggal_perdagangan_terakhir
                FROM transactions t
                JOIN (
                    SELECT kode_saham, MAX(tanggal_perdagangan_terakhir) AS max_tanggal
                    FROM transactions
                    GROUP BY kode_saham
                ) sub ON t.kode_saham = sub.kode_saham AND t.tanggal_perdagangan_terakhir = sub.max_tanggal;
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

        let data: Vec<Stock> = rows
            .iter()
            .map(|row| {
                Stock{
                    kode_saham: row.get(0),
                    nama_perusahaan: row.get(1),
                    open_price: row.get("open_price"),
                    penutupan: row.get("penutupan"),
                    tanggal_perdagangan_terakhir: row
                        .get::<&str, chrono::NaiveDate>("tanggal_perdagangan_terakhir")
                        .to_string()
                }
            })
            .collect();
    
        Ok(data)
    }
}