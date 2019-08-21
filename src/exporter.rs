use serde::{Deserialize, Serialize};
use std::fs;

pub struct JSONExporter {}

impl JSONExporter {
    pub fn emit(filename: String, data: String) {
        fs::write(&filename, &data).expect("unable to write file");
    }
}
