pub mod bitfinex;
pub mod exporter;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FinancialData {
    time: i64,
    open: f64,
    close: f64,
    low: f64,
    high: f64,
    volume: f64,
}
