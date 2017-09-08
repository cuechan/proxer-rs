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




const API_BASE_PATH: &str = "http://proxer.me/api/v1/";



#[derive(Debug, Clone)]
pub struct Proxer {
    api_key: &'static str,
    base_uri: &'static str,
    user_agent: String,
}




#[allow(unused)]
impl<'a> Proxer {
    /// Create a new api client
    pub fn new(api_key: &'static str) -> Self {
        let crates_version = &std::env::var("CARGO_PKG_VERSION").unwrap_or("unknown".to_string());
        let crates_name = std::env::var("CARGO_PKG_NAME").unwrap_or("unknown".to_string());


        let ua = format!("libproxer-rust({}/v{})", crates_name, crates_version);

        Self {
            api_key: api_key,
            base_uri: API_BASE_PATH,
            user_agent: ua,
        }
    }



    // add few parameters (api-key) and fire up the request.
    // do the error handling and parsing stuff
    // hand back a Json-Value with the actuall data

    pub fn execute(mut self, mut request: request::Request) -> Result<reqwest::Response, error::Error> {
        let url = Url::parse(&(self.base_uri.to_string() + request.clone().get_url())).unwrap();
        let orig_request = request.clone();

        request.set_parameter("api_key", self.api_key.to_string());

        let mut headers = reqwest::header::Headers::new();
        headers.set(header::UserAgent::new(self.user_agent));
        headers.set(reqwest::header::ContentType::form_url_encoded());



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


    pub fn api(self) -> api::Api {
        api::Api(self)
    }
}



#[derive(Debug, Clone, Deserialize)]
pub struct ApiResponse {
    error: i64,
    message: String,
    data: Option<Value>,
    code: Option<i64>,
}
