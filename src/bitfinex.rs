extern crate reqwest;

use crate::FinancialData;
use serde_json::{Result, Value};
use std::process::exit;
use std::thread::sleep;
use std::time::{Duration, SystemTime};

pub struct Bitfinex {}

impl Bitfinex {
    pub fn get(timeframe: String, currency: String, start: i64, end: i64) -> Vec<FinancialData> {
        let mut now = SystemTime::now();
        let mut counter = 0;
        let mut lstart = start;

        let mut ret: Vec<FinancialData> = Vec::new();
        loop {
            if (end - lstart) < 10000 {
                return ret;
            }

            match now.elapsed() {
                Ok(elapsed) => {
                    if elapsed.as_secs() == 60 {
                        counter = 0;
                        now = SystemTime::now();
                    }
                    if counter < 60 {
                        let url = format!("https://api-pub.bitfinex.com/v2/candles/trade:{}:{}/hist?limit=5000&start={}&end={}&sort=1",
                  timeframe, currency, lstart, end);
                        println!("{}", url);

                        let mut body = reqwest::get(&url).unwrap();
                        let text = body.text().unwrap();

                        let values: Vec<FinancialData> = serde_json::from_str(&text).unwrap();
                        for value in &values {
                            ret.push(value.clone());
                        }
                        if values.len() == 0 {
                            println!("no values in the data range");
                            exit(2);
                        }
                        let last: i64 = (values[values.len() - 1].time) as i64;
                        lstart = last;
                        println!("last = {:?}", lstart);
                        counter += 1;
                        sleep(Duration::new(1, 0));
                    }
                }
                Err(e) => {
                    println!("Error: {:?}", e);
                }
            }
        }
    }
}
