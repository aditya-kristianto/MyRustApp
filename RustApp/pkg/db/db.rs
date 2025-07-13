use chrono::DateTime;
use chrono::Utc;
use chrono::NaiveDateTime;

pub fn _format_datetime(dt: &DateTime<Utc>) -> String {
    dt.to_rfc3339() // You can choose a different format if needed
}

// Function to convert DateTime<Utc> to PgTimestamp
// pub fn convert_to_pg_timestamp(dt: &DateTime<Utc>) -> tokio_postgres::types::PgTimestamp {
//     tokio_postgres::types::PgTimestamp::from_std(dt.into())
// }

// Function to convert DateTime<Utc> to NaiveDateTime
pub fn _convert_to_naive_datetime(dt: &DateTime<Utc>) -> NaiveDateTime {
    dt.naive_utc()
}