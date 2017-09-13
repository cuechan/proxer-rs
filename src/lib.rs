#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(warnings)]
#![allow(unused)]

#[macro_use]
extern crate log;
#[macro_use]
extern crate reqwest;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate chrono;
extern crate futures;
extern crate serde_json;
extern crate tokio_core;

pub mod error;
pub mod types;
pub mod request;
pub mod api;
pub mod prelude;

use prelude::*;
use reqwest::{Url, Method};
use reqwest::IntoUrl;
use reqwest::header;
use serde_json::Value;
use std::collections::HashMap;
use std::process::exit;
use std::result::Result;
use std::thread;
use std::time;
use std::rc::Rc;




const API_BASE_PATH: &str = "http://proxer.me/api/v1/";



#[derive(Debug, Clone)]
pub struct Proxer<'a> {
    api_key: &'a str,
    base_uri: &'a str,
    user_agent: String,
}




#[allow(unused)]
impl<'a> Proxer<'a> {
    /// Create a new api client
    pub fn new(api_key: &'a str) -> Self {
        let crates_version = &std::env::var("CARGO_PKG_VERSION").unwrap_or("unknown".to_string());
        let crates_name = std::env::var("CARGO_PKG_NAME").unwrap_or("unknown".to_string());


        let ua = format!("libproxer-rust({}/v{})", crates_name, crates_version);

        Self {
            api_key: api_key,
            base_uri: API_BASE_PATH,
            user_agent: ua,
        }
    }




    pub fn execute(self, mut request: request::Request) -> Result<reqwest::Response, error::Error> {
        let url = Url::parse(&(self.base_uri.to_string() + request.clone().get_url())).unwrap();




        let mut headers = reqwest::header::Headers::new();

        headers.set(header::UserAgent::new(self.user_agent.to_string()));
        headers.set(reqwest::header::ContentType::form_url_encoded());


        request.set_parameter("api_key", self.api_key.to_string());

        let client = reqwest::Client::new().unwrap();



        let response = client
            .post(url)
            .unwrap()
            .form(&request.get_paramter())
            .unwrap()
            .headers(headers)
            .send();



        match response {
            Err(e) => {
                info!("request unsuccessfull");
                Err(error::Error::Http)
            }
            Ok(request) => {
                info!("request was successfull");
                Ok(request)
            }
        }
    }


    pub fn api(&self) -> api::Api<'a> {
        api::Api{
            proxer: self.clone()
        }
    }
}



#[derive(Debug, Clone, Deserialize)]
pub struct ApiResponse {
    pub error: i64,
    pub message: String,
    pub data: Option<Value>,
    pub code: Option<i64>,
}
