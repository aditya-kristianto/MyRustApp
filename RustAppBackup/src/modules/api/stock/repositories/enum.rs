use serde::{Serialize, Deserialize};
use utoipa::{ToSchema};

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(untagged)]
pub enum DataValue {
    BearishStockArray(Vec<BearishStock>),
    BullishStockArray(Vec<BullishStock>),
    String(String),
    StringArray(Vec<String>),
    DateArray(Vec<CustomNaiveDate::CustomDate>),
    FrequencyStockArray(Vec<FrequencyStock>),
    MACDStockArray(Vec<MACDStock>),
    RSIStockArray(Vec<RSIStock>),
    StockArray(Vec<StockInfo>),
    StockEMAArray(Vec<StockEMA>),
    StockSMAArray(Vec<StockSMA>),
    SummaryStockArray(Vec<SummaryStock>),
}