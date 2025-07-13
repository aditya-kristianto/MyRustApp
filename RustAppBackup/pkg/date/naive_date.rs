use chrono::Datelike;
use chrono::NaiveDate;
use postgres_types::{FromSql, ToSql};
use serde::{Deserialize, Serialize};
use std::error::Error;
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CustomDate {
    #[schema(example = "2024-12-07")]
    date: NaiveDate,
}

impl CustomDate {
    // Define a constructor similar to NaiveDate::from_ymd_opt
    pub fn from_ymd_opt(year: i32, month: u32, day: u32) -> Option<Self> {
        NaiveDate::from_ymd_opt(year, month, day).map(|date| CustomDate { date })
    }

    /// Helper constructor to create CustomDate from a NaiveDate
    pub fn from_naive_date(naive_date: NaiveDate) -> Self {
        Self { date: naive_date }
    }

    /// Extract year from the NaiveDate
    pub fn year(&self) -> i32 {
        self.date.year()
    }

    /// Extract month from the NaiveDate
    pub fn month(&self) -> u32 {
        self.date.month()
    }

    /// Extract day from the NaiveDate
    pub fn day(&self) -> u32 {
        self.date.day()
    }
}

// Implement ToSql
impl ToSql for CustomDate {
    fn to_sql(
        &self,
        ty: &postgres_types::Type,
        out: &mut bytes::BytesMut,
    ) -> Result<postgres_types::IsNull, Box<dyn Error + Sync + Send>> {
        self.date.to_sql(ty, out)
    }

    fn accepts(ty: &postgres_types::Type) -> bool {
        <NaiveDate as ToSql>::accepts(ty)
    }

    postgres_types::to_sql_checked!();
}

// Implement FromSql
impl FromSql<'_> for CustomDate {
    fn from_sql(
        ty: &postgres_types::Type,
        raw: &[u8],
    ) -> Result<Self, Box<dyn Error + Sync + Send>> {
        let date = NaiveDate::from_sql(ty, raw)?;
        Ok(CustomDate { date })
    }

    fn accepts(ty: &postgres_types::Type) -> bool {
        <NaiveDate as FromSql>::accepts(ty)
    }
}