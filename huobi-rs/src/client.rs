use crate::{error::*, models::*};
use ring::{digest, hmac};
use std::collections::BTreeMap;
use serde_json::from_str;
use reqwest;

#[derive(Clone)]
pub struct Client {
    //    key: Option<APIKey>,
    api_key: String,
    secret_key: String,
}

#[derive(Clone)]
pub struct APIKey {
    api_key: String,
    secret_key: String,
}

static API_HOST: &'static str = "api.huobi.pro";

impl Client {
    pub fn new(api_key: &str, secret_key: &str) -> Self {
        Client {
            api_key: api_key.into(),
            secret_key: secret_key.into(),
        }
    }

    pub fn get(&self, endpoint: &str, params: &str) -> APIResult<String> {
        let request = format!("https://{}{}?{}", API_HOST, endpoint, params,);
        ::log::info!("request: {:?}", request.clone());
        // let proxy = reqwest::Proxy::all("http://127.0.0.1:1080").unwrap();
        // let client = reqwest::Client::new();
        // let body = client.get(request.as_str()).send()?.text();
        let body = reqwest::get(request.as_str())?.text()?;
        ::log::info!("result: {:?}", body.clone());

        // check for errors
        let err_response: APIErrorResponse = serde_json::from_str(body.as_str())?;


        if err_response.status == "error" {
            if let Some(err_msg) = err_response.err_msg {
                return Err(Box::new(HuobiError::ApiError(err_msg)));
            } else {
                return Err(Box::new(HuobiError::ApiError(format!(
                    "result dump: {:?}",
                    err_response
                ))));
            }
        }

        Ok(body)
    }

    pub fn get_signed(
        &self,
        endpoint: &str,
        mut params: BTreeMap<String, String>,
    ) -> APIResult<String> {
        params.insert("AccessKeyId".to_string(), self.api_key.clone());
        params.insert("SignatureMethod".to_string(), "HmacSHA256".to_string());
        params.insert("SignatureVersion".to_string(), "2".to_string());
        params.insert("Timestamp".to_string(), get_timestamp());


        println!("params: {:?}", params.clone());

        let params = build_query_string(params);
        let signature = sign_hmac_sha256_base64(
            &self.secret_key,
            &format!("{}\n{}\n{}\n{}", "GET", API_HOST, endpoint, params,),
        )
        .to_string();

        let request = format!(
            "https://{}{}?{}&Signature={}",
            API_HOST,
            endpoint,
            params,
            percent_encode(&signature.clone())
        );

        ::log::info!("request: {:?}", request.clone());
        
        let mut response = reqwest::get(request.as_str())?;
        let body = response.text()?;

        ::log::info!("body: {:?}", body.clone());

        // check for errors
        let err_response: APIErrorResponse = serde_json::from_str(body.as_str())?;

        if err_response.status == "error" {
            if let Some(err_msg) = err_response.err_msg {
                return Err(Box::new(HuobiError::ApiError(err_msg)));
            } else {
                return Err(Box::new(HuobiError::ApiError(format!(
                    "result dump: {:?}",
                    err_response
                ))));
            }
        }

        Ok(body)
    }

    pub fn accounts(&self) -> APIResult<Vec<Account>> {
        let params: BTreeMap<String, String> = BTreeMap::new();
        let data = self.get_signed("/v1/account/accounts", params)?;
        let response: APIResponse<Vec<Account>> = from_str(data.as_str())?;
        Ok(response.data)
    }

    pub fn balance(&self, id: u32) -> APIResult<Balance> {
        let params: BTreeMap<String, String> = BTreeMap::new();
        let data = self.get_signed(&format!("/v1/account/accounts/{}/balance", id), params)?;
        let response: APIResponse<Balance> = from_str(data.as_str())?;
        Ok(response.data)
    }

    pub fn orders(&self, symbol: &str, states: &str) -> APIResult<Vec<Order>> {
        let mut params: BTreeMap<String, String> = BTreeMap::new();
        params.insert("symbol".to_string(), symbol.to_string());
        params.insert(
            "states".to_string(),
            states.to_string(),
        );
//        params.insert("types".to_string(), "buy-limit".to_string());
        let data = self.get_signed("/v1/order/orders", params)?;
        let response: APIResponse<Vec<Order>> = from_str(data.as_str())?;

        Ok(response.data)
    }

     /// This endpoint retrieves the latest tickers for all supported pairs.
     pub fn tickers(&self) -> APIResult<Vec<Ticker>> {
        let data = self.get("/market/tickers", "")?;
        let response: APIResponse<Vec<Ticker>> = from_str(data.as_str())?;

        Ok(response.data)
    }

    /// return all symbol pairs used on the exchange.
    pub fn common_symbols(&self) -> APIResult<Vec<Pair>> {
        let data = self.get("/v1/common/symbols", "")?;
        let response: APIResponse<Vec<Pair>> = from_str(data.as_str())?;

        Ok(response.data)
    }

    pub fn common_currencys(&self) -> APIResult<Vec<Currency>> {
        let data = self.get("/v1/common/currencys", "")?;
        let response: APIResponse<Vec<Currency>> = from_str(data.as_str())?;

        Ok(response.data)
    }

    pub fn common_timestamp(&self) -> APIResult<Timestamp> {
        let data = self.get("/v1/common/timestamp", "")?;
        let response: APIResponse<Timestamp> = from_str(data.as_str())?;

        Ok(response.data)
    }
}


pub fn build_query_string(parameters: BTreeMap<String, String>) -> String {
    parameters
        .into_iter()
        .map(|(key, value)| format!("{}={}", key, percent_encode(&value.clone())))
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


pub fn percent_encode(source: &str) -> String {
    use percent_encoding::{define_encode_set, utf8_percent_encode, USERINFO_ENCODE_SET};
    define_encode_set! {
        pub CUSTOM_ENCODE_SET = [USERINFO_ENCODE_SET] | { '+', ',' }
    }
    let signature = utf8_percent_encode(&source, CUSTOM_ENCODE_SET).to_string();
    signature
}

pub fn get_timestamp() -> String {
    let utc_time = chrono::Utc::now();
    let formatted_time = utc_time.format("%Y-%m-%dT%H:%M:%S").to_string();

    formatted_time
}
