use error;
use prelude::*;
use request;
use reqwest;
use reqwest::{Url, Method};
use reqwest::header;
use reqwest::IntoUrl;
use serde_json::Value;
use std;
use std::collections::HashMap;
use api;
use api::info::Request;




const API_BASE_PATH: &str = "http://proxer.me/api/v1/";



#[derive(Debug, Clone)]
pub struct Client<'a> {
    api_key: &'a str,
    base_uri: &'a str,
    user_agent: String,
}




#[allow(unused)]
impl<'a> Client<'a> {
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




    pub fn execute<T: Request + Clone>(self, request: T) -> Result<reqwest::Response, error::Error> {
        let url = Url::parse(&(self.base_uri.to_string() + &request.get_url())).unwrap();




        let mut headers = reqwest::header::Headers::new();

        headers.set(header::UserAgent::new(self.user_agent.to_string()));
        headers.set(reqwest::header::ContentType::form_url_encoded());


        let parameter = request.get_data().clone();
        parameter.insert("api_key".to_string(), self.api_key.to_string());

        let client = reqwest::Client::new().unwrap();



        let response = client
            .post(url)
            .unwrap()
            .form(&parameter)
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
        api::Api { client: self.clone() }
    }
}



#[derive(Debug, Clone, Deserialize)]
pub struct ApiResponse {
    pub error: i64,
    pub message: String,
    pub data: Option<Value>,
    pub code: Option<i64>,
}
