#![allow(dead_code)]
#![allow(unused_variables)]

use serde::de::{self, Unexpected, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use std::fmt::{self, Display};
use std::str::FromStr;

//订单薄
#[derive(Serialize, Deserialize, Debug)]
pub struct OkexOrderBook {
    pub timestamp: String,
    pub bids: Vec<[f64;2]>,
    pub asks: Vec<[f64;2]>,
}

//账户余额详情
#[derive(Serialize, Deserialize, Debug)]
pub struct OkexBalance {
    pub currency: String,
    pub balance: f64,
    pub hold: f64,
    pub available: f64,
}

//下单返回值
#[derive(Serialize, Deserialize, Debug)]
pub struct OkexPlaceOrderResponse {
    pub order_id: String,
    pub client_oid: String,
    pub result: bool,
}






