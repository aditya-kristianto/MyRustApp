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