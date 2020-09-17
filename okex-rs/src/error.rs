#![allow(dead_code)]
#![allow(unused_variables)]

use core::fmt;
use std::error::Error;
use serde::{Deserialize, Deserializer, Serialize};

pub type APIResult<T> = Result<T, Box<dyn std::error::Error>>;


#[derive(Serialize, Deserialize, Debug)]
pub struct OkexAPIErrorResponse {
    pub code: Option<String>,

    pub message: Option<String>,

    pub error_code: Option<String>,

    pub error_msg: Option<String>,

}


#[derive(Debug, Clone)]
pub enum OkexError {
    ApiError(String),
}

impl fmt::Display for OkexError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.clone() {
            OkexError::ApiError(why) => write!(f, "OkexApiError: {}", why),
        }
    }
}

impl Error for OkexError {
    fn description(&self) -> &str {
        "Okex Error"
    }
}


