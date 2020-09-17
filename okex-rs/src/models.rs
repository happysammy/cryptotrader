#![allow(dead_code)]
#![allow(unused_variables)]

use serde::de::{self, Unexpected, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use std::fmt::{self, Display};
use std::str::FromStr;



#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct OkexOrderBook {
    pub timestamp: String,
    pub bids: Vec<[f64;2]>,
    pub asks: Vec<[f64;2]>,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct OkexBalance {
    pub currency: String,
    pub balance: f64,
    pub hold: f64,
    pub available: f64,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct OkexPlaceOrderResponse {
    pub order_id: String,
    pub client_oid: String,
    pub result: bool,
}



#[derive(Serialize, Deserialize, Debug)]
pub struct OkexCancelOrderResponse {
    pub order_id: String,
    pub client_oid: String,
    pub result: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OkexOrderDetailResponse {
    pub client_oid: String,
    pub order_id: String,
    pub price: String,
    pub size: String,
    pub instrument_id: String,
    pub side: String,
    pub filled_size: f64,
    pub filled_notional: f64,
    pub status: String,
    pub state: i16,
    pub notional: f64,
    pub timestamp: String,    
    pub fee_currency: String,
    pub fee: f64,
    pub rebate_currency: String,
    pub rebate: f64,
}










