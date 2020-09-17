use crate::{error::*, models::*};
use ring::{digest, hmac};
use std::collections::BTreeMap;
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use reqwest::Method;
use serde_json::from_str;

#[derive(Clone,Debug)]
pub struct Client {
    api_key: String,
    secret_key: String,
    passphrase: String
}


static API_HOST: &'static str = "www.okex.com";

impl Client {
    pub fn new(api_key: &str, secret_key: &str, passphrase: &str) -> Self {
        Client {
            api_key: api_key.into(),
            secret_key: secret_key.into(),
            passphrase:  passphrase.into(),
            
        }
    }

    fn authenticate(&self, endpoint: &str, method: Method, params: &BTreeMap<String, String>, timestamp: &str)-> String {   
        let mut request_path: String = String::from(endpoint);   
        let mut body = String::new();                 
        if method == Method::GET {
            let params_str = build_query_string(params);
            if params.len() != 0 {
                request_path =  format!("{}?{}", endpoint, params_str);               
            }
        }
        else {
            if params.len() != 0 {
                body = serde_json::to_string(params).unwrap();
            }
        }
        let hash_str = format!("{}{}{}{}", timestamp, method.as_str().to_uppercase(), request_path, &body);
        let sign_str = sign_hmac_sha256_base64(&self.secret_key, &hash_str);
        sign_str
    }

    pub fn get(&self, endpoint: &str, params: &BTreeMap<String, String>, signed: bool) -> APIResult<String> {
        let params_str = build_query_string(params);
        let url = format!("https://{}{}?{}", API_HOST, endpoint, params_str,);

        let timestamp = get_timestamp();
        let mut sign_str = String::new();
        if signed {
            sign_str = self.authenticate(endpoint, Method::GET, params, &timestamp);
        }

        let mut header_map = HeaderMap::new();
        header_map.insert("OK-ACCESS-KEY", HeaderValue::from_str(&self.api_key).unwrap());
        header_map.insert("OK-ACCESS-SIGN", HeaderValue::from_str(&sign_str).unwrap());
        header_map.insert("OK-ACCESS-TIMESTAMP", HeaderValue::from_str(&timestamp).unwrap());
        header_map.insert("OK-ACCESS-PASSPHRASE", HeaderValue::from_str(&self.passphrase).unwrap());
        header_map.insert(CONTENT_TYPE, HeaderValue::from_static("application/json; charset=UTF-8"));
        let client = reqwest::Client::new();
        let body = client
        .get(url.as_str())
        .headers(header_map)
        .send().unwrap().text()?;   

        let err_response: OkexAPIErrorResponse  = serde_json::from_str(body.as_str())?;
        if err_response.error_code != None &&  err_response.error_msg != None {
            if let Some(err_msg) = err_response.error_msg {
                return Err(Box::new(OkexError::ApiError(err_msg)));
            } else {
                return Err(Box::new(OkexError::ApiError(format!(
                    "result dump: {:?}",
                    err_response
                ))));
            }
            
        }
        Ok(body)
    }

    pub fn post(&self, endpoint: &str, params: &BTreeMap<String, String>, signed: bool) -> APIResult<String> {
        let url = format!("https://{}{}", API_HOST, endpoint);
        let timestamp = get_timestamp();
        let mut sign_str = String::new();
        if signed {
            sign_str = self.authenticate(endpoint, Method::POST, params, &timestamp);
        }
        let mut header_map = HeaderMap::new();
        header_map.insert("OK-ACCESS-KEY", HeaderValue::from_str(&self.api_key).unwrap());
        header_map.insert("OK-ACCESS-SIGN", HeaderValue::from_str(&sign_str).unwrap());
        header_map.insert("OK-ACCESS-TIMESTAMP", HeaderValue::from_str(&timestamp).unwrap());
        header_map.insert("OK-ACCESS-PASSPHRASE", HeaderValue::from_str(&self.passphrase).unwrap());
        header_map.insert(CONTENT_TYPE, HeaderValue::from_static("application/json; charset=UTF-8"));
        
        let client = reqwest::Client::new();
        let body = client
        .post(&url)
        .json(params)
        .headers(header_map)
        .send().unwrap().text()?;   

        let err_response: OkexAPIErrorResponse  = serde_json::from_str(body.as_str())?;
        if err_response.error_code != None &&  err_response.error_msg != None {
            if let Some(err_msg) = err_response.error_msg {
                return Err(Box::new(OkexError::ApiError(err_msg)));
            } else {
                return Err(Box::new(OkexError::ApiError(format!(
                    "result dump: {:?}",
                    err_response
                ))));
            }
            
        }
        Ok(body)     

    }

   

    pub fn get_orderbook(&self, symbol: &str, size: i16) -> APIResult<OkexOrderBook> {
       let okex_symbol = symbol;
       let endpoint = format!("/api/spot/v3/products/{}/book", okex_symbol);     
       let mut params: BTreeMap<String, String> = BTreeMap::new();
       params.insert("size".into(), size.to_string());
       params.insert("depth".into(), "0".into());
       let data = self.get(&endpoint, &params, false)?;
       let response = serde_json::from_str(data.as_str())?;
       //String::from(data.as_str())
        Ok(response)
    }


    pub fn get_balance(&self) -> APIResult<Vec<OkexBalance>> {
        let endpoint =  "/api/spot/v3/accounts";
        let  params: BTreeMap<String, String> = BTreeMap::new();
        let data = self.get(&endpoint, &params, true)?;
        let response = serde_json::from_str(data.as_str())?;
        
         Ok(response)
    }

    pub fn place_order(&self, trade_type: &str, symbol: &str, price: f64, amount: f64, user_orderid: Option<String>) -> APIResult<OkexPlaceOrderResponse> {
        let okex_symbol = symbol;
        let mut trade_type_convert: String = String::new();
        if trade_type == "buy-limit" {
            trade_type_convert = String::from("buy");
        } else if trade_type == "sell-limit" {
            trade_type_convert = String::from("sell");
        }
        let endpoint = "/api/spot/v3/orders";
        let mut params: BTreeMap<String, String> = BTreeMap::new();
        params.insert("client_oid".into(), user_orderid.unwrap());
        params.insert("type".into(), "limit".into());
        params.insert("side".into(), trade_type_convert);
        params.insert("price".into(), price.to_string());
        params.insert("amount".into(), amount.to_string());
        params.insert("instrument_id".into(), okex_symbol.into());

        let data = self.post(&endpoint, &params, true)?;
        let response = serde_json::from_str(data.as_str())?;

        Ok(response)
    }

    pub fn cancel_order(&self, orderid: &str, symbol: &str) -> APIResult<OkexCancelOrderResponse> {
        let okex_symbol = symbol;
        let endpoint =  format!("/api/spot/v3/cancel_orders/{}", orderid);
        let mut params: BTreeMap<String, String> = BTreeMap::new();
        params.insert("instrument_id".into(), okex_symbol.into());

        let data = self.post(&endpoint, &params, true)?;
        let response = serde_json::from_str(data.as_str())?;

        Ok(response)
    }


    pub fn query_order_state(&self, orderid: &str, symbol: &str)  -> APIResult<OkexOrderDetailResponse> {
        let okex_symbol = symbol;
        let endpoint =  format!("/api/spot/v3/orders/{}", orderid);
        let mut params: BTreeMap<String, String> = BTreeMap::new();
        params.insert("instrument_id".into(), okex_symbol.into());

        let data = self.get(&endpoint, &params, true)?;
        let response = serde_json::from_str(data.as_str())?;

        Ok(response)
    }

    pub fn get_unfilled_orders(&self, symbol: &str) -> APIResult<Vec<OkexOrderDetailResponse>> {
        let okex_symbol = symbol;
        let endpoint = "/api/spot/v3/orders_pending";
        let mut params: BTreeMap<String, String> = BTreeMap::new();
        params.insert("instrument_id".into(), okex_symbol.into());
        params.insert("limit".into(), "100".into());

        let data = self.get(&endpoint, &params, true)?;
        let response = serde_json::from_str(data.as_str())?;

        Ok(response)
    }


}

pub fn build_query_string(parameters: &BTreeMap<String, String>) -> String {
    parameters
        .into_iter()
        .map(|(key, value)| format!("{}={}", key, value))
        .collect::<Vec<String>>()
        .join("&")
}


pub fn sign_hmac_sha256_base64(secret: &str, digest: &str) -> String {
    use data_encoding::BASE64;

    let signed_key = hmac::SigningKey::new(&digest::SHA256, secret.as_bytes());
    let signature = hmac::sign(&signed_key, digest.as_bytes());
    let b64_encoded_sig = BASE64.encode(signature.as_ref());

    b64_encoded_sig
}


pub fn get_timestamp() -> String {
    let utc_time = chrono::Utc::now();
    let formatted_time = utc_time.format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string();

    formatted_time
}



