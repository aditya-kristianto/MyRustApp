use serde::{Serialize, Deserialize};
use utoipa::{ToSchema};

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct SummaryStock {
    kode_saham: String,
    nama_perusahaan: String,
    open_price: i32,
    penutupan: i32,
    tanggal_perdagangan_terakhir: String, // Adjust type as needed
    rsi_6: f64,
    rsi_12: f64,
    rsi_24: f64,
    macd_line: f64,
    signal_line: f64,
    macd_histogram: f64,
    macd_trendline: String,
    rsi_trendline: String,
    michael_harris_signal: String,
    ichimoku_trendline: String,
}

impl SummaryStock {
    pub fn new(kode_saham: String, nama_perusahaan: String, open_price: i32, penutupan: i32, tanggal_perdagangan_terakhir: String, rsi_6: f64, rsi_12: f64, rsi_24: f64, macd_line: f64, signal_line: f64, macd_histogram: f64, macd_trendline: String, rsi_trendline: String, michael_harris_signal: String, ichimoku_trendline: String) -> Self {
        SummaryStock { kode_saham, nama_perusahaan, open_price, penutupan, tanggal_perdagangan_terakhir, rsi_6, rsi_12, rsi_24, macd_line, signal_line, macd_histogram, macd_trendline, rsi_trendline, michael_harris_signal, ichimoku_trendline }
    }

    // Function to query the database and get a user by their ID
    pub async fn get_stocks_summary(data: web::Data<Arc<Client>>) -> Result<Vec<SummaryStock>, Error> {
        // Access the database client from app_data
        let db_client = data.get_ref();
        let query_str: String = format!(
            "
            WITH price_changes AS (
                SELECT
                    t.kode_saham,
                    t.nama_perusahaan,
                    t.open_price,
                    t.penutupan,
                    t.tanggal_perdagangan_terakhir,
                    LAG(t.penutupan) OVER (PARTITION BY t.kode_saham ORDER BY t.tanggal_perdagangan_terakhir) AS previous_close,
                    -- Calculate Gain and Loss
                    GREATEST(t.penutupan - LAG(t.penutupan) OVER (PARTITION BY t.kode_saham ORDER BY t.tanggal_perdagangan_terakhir), 0) AS gain,
                    GREATEST(LAG(t.penutupan) OVER (PARTITION BY t.kode_saham ORDER BY t.tanggal_perdagangan_terakhir) - t.penutupan, 0) AS loss
                FROM transactions t
            ),
            average_gain_loss AS (
                SELECT
                    p.kode_saham,
                    p.nama_perusahaan,
                    p.open_price,
                    p.penutupan,
                    p.tanggal_perdagangan_terakhir,
                    -- Calculate 6-period Average Gain and Loss
                    AVG(p.gain) OVER (PARTITION BY p.kode_saham ORDER BY p.tanggal_perdagangan_terakhir ROWS BETWEEN 5 PRECEDING AND CURRENT ROW) AS avg_gain_6,
                    AVG(p.loss) OVER (PARTITION BY p.kode_saham ORDER BY p.tanggal_perdagangan_terakhir ROWS BETWEEN 5 PRECEDING AND CURRENT ROW) AS avg_loss_6,
                    -- Calculate 12-period Average Gain and Loss
                    AVG(p.gain) OVER (PARTITION BY p.kode_saham ORDER BY p.tanggal_perdagangan_terakhir ROWS BETWEEN 11 PRECEDING AND CURRENT ROW) AS avg_gain_12,
                    AVG(p.loss) OVER (PARTITION BY p.kode_saham ORDER BY p.tanggal_perdagangan_terakhir ROWS BETWEEN 11 PRECEDING AND CURRENT ROW) AS avg_loss_12,
                    -- Calculate 24-period Average Gain and Loss
                    AVG(p.gain) OVER (PARTITION BY p.kode_saham ORDER BY p.tanggal_perdagangan_terakhir ROWS BETWEEN 23 PRECEDING AND CURRENT ROW) AS avg_gain_24,
                    AVG(p.loss) OVER (PARTITION BY p.kode_saham ORDER BY p.tanggal_perdagangan_terakhir ROWS BETWEEN 23 PRECEDING AND CURRENT ROW) AS avg_loss_24
                FROM price_changes p
            ),
            rsi_calculation AS (
                SELECT
                    a.kode_saham,
                    a.nama_perusahaan,
                    a.open_price,
                    a.penutupan,
                    a.tanggal_perdagangan_terakhir,
                    -- RSI 6
                    CASE 
                        WHEN a.avg_loss_6 = 0 THEN 100
                        ELSE 100 - (100 / (1 + (a.avg_gain_6 / a.avg_loss_6)))
                    END AS rsi_6,
                    -- RSI 12
                    CASE 
                        WHEN a.avg_loss_12 = 0 THEN 100
                        ELSE 100 - (100 / (1 + (a.avg_gain_12 / a.avg_loss_12)))
                    END AS rsi_12,
                    -- RSI 24
                    CASE 
                        WHEN a.avg_loss_24 = 0 THEN 100
                        ELSE 100 - (100 / (1 + (a.avg_gain_24 / a.avg_loss_24)))
                    END AS rsi_24
                FROM average_gain_loss a
            ),
            ema_calculations AS (
                SELECT 
                    r.kode_saham,
                    r.nama_perusahaan,
                    r.tanggal_perdagangan_terakhir,
                    r.penutupan,
                    -- EMA 12
                    ROUND(AVG(r.penutupan) OVER (PARTITION BY r.kode_saham ORDER BY r.tanggal_perdagangan_terakhir ROWS BETWEEN 11 PRECEDING AND CURRENT ROW), 2) AS ema_12,
                    -- EMA 26
                    ROUND(AVG(r.penutupan) OVER (PARTITION BY r.kode_saham ORDER BY r.tanggal_perdagangan_terakhir ROWS BETWEEN 25 PRECEDING AND CURRENT ROW), 2) AS ema_26
                FROM price_changes r
            ),
            macd_calculation AS (
                SELECT 
                    e.kode_saham,
                    e.nama_perusahaan,
                    e.tanggal_perdagangan_terakhir,
                    e.ema_12,
                    e.ema_26,
                    -- MACD Line
                    ROUND(e.ema_12 - e.ema_26, 2) AS macd_line,
                    -- Signal Line (9-period EMA of MACD)
                    ROUND(AVG(e.ema_12 - e.ema_26) OVER (PARTITION BY e.kode_saham ORDER BY e.tanggal_perdagangan_terakhir ROWS BETWEEN 8 PRECEDING AND CURRENT ROW), 2) AS signal_line
                FROM ema_calculations e
            ),
            michael_harris_precalculation AS (
                SELECT
                    r.kode_saham,
                    r.nama_perusahaan,
                    r.open_price,
                    r.penutupan,
                    r.tanggal_perdagangan_terakhir,
                    LAG(r.penutupan) OVER (PARTITION BY r.kode_saham ORDER BY r.tanggal_perdagangan_terakhir) AS prev_close1,
                    LAG(r.penutupan, 2) OVER (PARTITION BY r.kode_saham ORDER BY r.tanggal_perdagangan_terakhir) AS prev_close2,
                    LAG(r.penutupan, 3) OVER (PARTITION BY r.kode_saham ORDER BY r.tanggal_perdagangan_terakhir) AS prev_close3
                FROM price_changes r
            ),
            michael_harris_signals AS (
                SELECT
                    m.kode_saham,
                    m.nama_perusahaan,
                    m.open_price,
                    m.penutupan,
                    m.tanggal_perdagangan_terakhir,
                    CASE
                        -- Bullish Reversal
                        WHEN m.penutupan > m.prev_close1 AND m.prev_close1 > m.prev_close2 AND m.prev_close2 < m.prev_close3 THEN 'BUY'
                        -- Bearish Reversal
                        WHEN m.penutupan < m.prev_close1 AND m.prev_close1 < m.prev_close2 AND m.prev_close2 > m.prev_close3 THEN 'SELL'
                        ELSE 'HOLD'
                    END AS michael_harris_signal
                FROM michael_harris_precalculation m
            ),
            ichimoku_calculations AS (
                SELECT 
                    t.kode_saham,
                    t.nama_perusahaan,
                    t.tanggal_perdagangan_terakhir,
                    t.penutupan,
                    -- Tenkan-sen (9-period Conversion Line)
                    (MAX(t.penutupan) OVER (PARTITION BY t.kode_saham ORDER BY t.tanggal_perdagangan_terakhir ROWS BETWEEN 8 PRECEDING AND CURRENT ROW) +
                    MIN(t.penutupan) OVER (PARTITION BY t.kode_saham ORDER BY t.tanggal_perdagangan_terakhir ROWS BETWEEN 8 PRECEDING AND CURRENT ROW)) / 2 AS tenkan_sen,
                    -- Kijun-sen (26-period Base Line)
                    (MAX(t.penutupan) OVER (PARTITION BY t.kode_saham ORDER BY t.tanggal_perdagangan_terakhir ROWS BETWEEN 25 PRECEDING AND CURRENT ROW) +
                    MIN(t.penutupan) OVER (PARTITION BY t.kode_saham ORDER BY t.tanggal_perdagangan_terakhir ROWS BETWEEN 25 PRECEDING AND CURRENT ROW)) / 2 AS kijun_sen,
                    -- Senkou Span A (Leading Span A)
                    (MAX(t.penutupan) OVER (PARTITION BY t.kode_saham ORDER BY t.tanggal_perdagangan_terakhir ROWS BETWEEN 8 PRECEDING AND CURRENT ROW) +
                    MIN(t.penutupan) OVER (PARTITION BY t.kode_saham ORDER BY t.tanggal_perdagangan_terakhir ROWS BETWEEN 8 PRECEDING AND CURRENT ROW) +
                    MAX(t.penutupan) OVER (PARTITION BY t.kode_saham ORDER BY t.tanggal_perdagangan_terakhir ROWS BETWEEN 25 PRECEDING AND CURRENT ROW) +
                    MIN(t.penutupan) OVER (PARTITION BY t.kode_saham ORDER BY t.tanggal_perdagangan_terakhir ROWS BETWEEN 25 PRECEDING AND CURRENT ROW)) / 4 AS senkou_span_a,
                    -- Senkou Span B (Leading Span B)
                    (MAX(t.penutupan) OVER (PARTITION BY t.kode_saham ORDER BY t.tanggal_perdagangan_terakhir ROWS BETWEEN 51 PRECEDING AND CURRENT ROW) +
                    MIN(t.penutupan) OVER (PARTITION BY t.kode_saham ORDER BY t.tanggal_perdagangan_terakhir ROWS BETWEEN 51 PRECEDING AND CURRENT ROW)) / 2 AS senkou_span_b,
                    -- Chikou Span (Lagging Span)
                    LAG(t.penutupan, 26) OVER (PARTITION BY t.kode_saham ORDER BY t.tanggal_perdagangan_terakhir) AS chikou_span
                FROM transactions t
            ),
            trendline_signals AS (
                SELECT 
                    r.kode_saham,
                    r.nama_perusahaan,
                    r.open_price,
                    r.penutupan,
                    r.tanggal_perdagangan_terakhir,
                    r.rsi_6,
                    r.rsi_12,
                    r.rsi_24,
                    m.macd_line,
                    m.signal_line,
                    -- MACD Histogram
                    ROUND(m.macd_line - m.signal_line, 2) AS macd_histogram,
                    -- Trendline Signals based on MACD
                    CASE 
                        WHEN m.macd_line > m.signal_line THEN 'BUY'
                        WHEN m.macd_line < m.signal_line THEN 'SELL'
                        ELSE 'HOLD'
                    END AS macd_trendline,
                    -- Generate Trendline Signal
                    CASE 	
                        WHEN r.rsi_6 < 30 OR r.rsi_12 < 30 OR r.rsi_24 < 30 THEN 'BUY' -- Oversold condition
                        WHEN r.rsi_6 > 70 OR r.rsi_12 > 70 OR r.rsi_24 > 70 THEN 'SELL' -- Overbought condition
                        ELSE 'HOLD' -- No clear trend
                    END AS rsi_trendline,
                     -- Generate Ichimoku Trendline Signals
                    CASE 
                        WHEN i.tenkan_sen > i.kijun_sen AND i.penutupan > i.senkou_span_a AND i.penutupan > i.senkou_span_b THEN 'BUY'
                        WHEN i.tenkan_sen < i.kijun_sen AND i.penutupan < i.senkou_span_a AND i.penutupan < i.senkou_span_b THEN 'SELL'
                        ELSE 'HOLD'
                    END AS ichimoku_trendline,
                    mh.michael_harris_signal
                FROM rsi_calculation r
                JOIN macd_calculation m 
                    ON r.kode_saham = m.kode_saham 
                    AND r.tanggal_perdagangan_terakhir = m.tanggal_perdagangan_terakhir
                JOIN michael_harris_signals mh
                    ON r.kode_saham = mh.kode_saham 
                    AND r.tanggal_perdagangan_terakhir = mh.tanggal_perdagangan_terakhir
                JOIN ichimoku_calculations i
                    ON r.kode_saham = i.kode_saham 
                    AND r.tanggal_perdagangan_terakhir = i.tanggal_perdagangan_terakhir
            )
            SELECT 
                t.kode_saham,
                t.nama_perusahaan,
                t.open_price,
                t.penutupan,
                t.tanggal_perdagangan_terakhir,
                t.rsi_6::DOUBLE PRECISION,
                t.rsi_12::DOUBLE PRECISION,
                t.rsi_24::DOUBLE PRECISION,
                t.macd_line::DOUBLE PRECISION,
                t.signal_line::DOUBLE PRECISION,
                t.macd_histogram::DOUBLE PRECISION,
                t.macd_trendline,
                t.rsi_trendline,
                t.michael_harris_signal,
                t.ichimoku_trendline
            FROM trendline_signals t
            WHERE t.tanggal_perdagangan_terakhir = (
                SELECT MAX(tanggal_perdagangan_terakhir) 
                FROM transactions
            )
            ORDER BY t.michael_harris_signal ASC, t.kode_saham ASC;
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

        let data: Vec<SummaryStock> = rows
            .iter()
            .map(|row| {
                SummaryStock{
                    kode_saham: row.get(0),
                    nama_perusahaan: row.get(1),
                    open_price: row.get("open_price"),
                    penutupan: row.get("penutupan"),
                    tanggal_perdagangan_terakhir: row
                        .get::<&str, chrono::NaiveDate>("tanggal_perdagangan_terakhir")
                        .to_string(),
                    rsi_6: row.get("rsi_6"),
                    rsi_12: row.get("rsi_12"),
                    rsi_24: row.get("rsi_24"),
                    macd_line: row.get("macd_line"),
                    signal_line: row.get("signal_line"),
                    macd_histogram: row.get("macd_histogram"),
                    macd_trendline: row.get("macd_trendline"),
                    rsi_trendline: row.get("rsi_trendline"),
                    michael_harris_signal: row.get("michael_harris_signal"),
                    ichimoku_trendline: row.get("ichimoku_trendline")
                }
            })
            .collect();
    
        Ok(data)
    }
}