#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(warnings)]
#![allow(unused)]

#[macro_use] extern crate log;
#[macro_use] extern crate reqwest;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate serde;
#[macro_use] extern crate chrono;
extern crate futures;
extern crate hyper;
extern crate serde_json;
extern crate tokio_core;

pub mod error;
pub mod types;
pub mod request;
pub mod response;

use error::api;
use reqwest::IntoUrl;
use std::collections::HashMap;
use std::process::exit;
use std::result::Result;
use std::thread;
use std::time;




#[derive(Debug)]
pub struct Api {
    api_key: &'static str,
    base_uri: &'static str,
    client: reqwest::Client,
    user_agent: String,
}



// Struct for storing api response without parsing type specific data
#[derive(Debug, Deserialize)]
struct Response {
    pub error: i64,
    pub message: String,
    pub data: serde_json::Value,
}


#[allow(unused)]
impl<'a> Api {
    /// Create a new api client
    pub fn new(api_key: &'static str) -> Self {
        let crates_version = &std::env::var("CARGO_PKG_VERSION").unwrap_or("unknown".to_string());
        let crates_name = std::env::var("CARGO_PKG_NAME").unwrap_or("unknown".to_string());


        let ua = format!("libproxer-rust({}/v{})", crates_name, crates_version);


        Api {
            api_key:           api_key,
            base_uri:          "http://proxer.me/api/v1/",
            client:            reqwest::Client::new().expect("failed to create client"),
            user_agent:        ua,
        }
    }


    fn request(&mut self, mut request: request::Request) -> reqwest::Result<reqwest::Response> {
        let uri = self.base_uri.to_string() + request.url;


        request.add_parameter("api_key", self.api_key.to_owned());

        let mut headers: reqwest::header::Headers = reqwest::header::Headers::new();
        headers.set(reqwest::header::UserAgent(self.user_agent.to_owned()));
        headers.set(reqwest::header::ContentType::form_url_encoded());

        let mut req = self.client.request(reqwest::Method::Post, uri.into_url().unwrap());



        let foo = req.headers(headers).form(&request.get_paramter());
        foo.send()
    }



























    // Here the api access methods begin


    /// Get the full information for an anime or manga
    ///
    /// See [Proxer wiki](http://proxer.me/wiki/Proxer_API/v1/Info#Get_Full_Entry)

    pub fn info_get_full_info(&mut self, id: i64) -> Result<types::FullInfo, error::Error> {
        let url = "info/fullentry";


        let mut request = request::Request::new(url);
        request.add_parameter("id", id.to_string());


        let res = self.request(request);


        // HTTP error?
        if res.is_err() {
            return Err(error::Error::Http)
        }


        // JSON error?
        let json: serde_json::Value = match serde_json::from_reader(res.unwrap()) {
            Err(e) => return Err(error::Error::Json),
            Ok(r) => r
        };


        // API error?
        let error_code = json.get("error").unwrap().as_i64().unwrap();

        if error_code != 0 {
            let err = error::api::Api::from(json);
            return Err(error::Error::Api(err));
        }




        // when everything went fine:

        let data: serde_json::Value = json.get("data").unwrap().clone();

        Ok(types::FullInfo::from(data))


        // Err(error::Error::Unknown)
    }






    pub fn info_get_comments(&mut self, id: i64) -> Result<Vec<types::Comment>, error::Error> {
        let url = "info/comments";

        let mut request = request::Request::new(url);
        request.add_parameter("id", id.to_string());


        let res = self.request(request);


        // HTTP error?
        if res.is_err() {
            return Err(error::Error::Http)
        }


        // JSON error?
        let json: serde_json::Value = match serde_json::from_reader(res.unwrap()) {
            Err(e) => return Err(error::Error::Json),
            Ok(r) => r
        };


        // API error?
        let error_code = json.get("error").unwrap().as_i64().unwrap();

        if error_code != 0 {
            let err = error::api::Api::from(json);
            return Err(error::Error::Api(err));
        }




        // when everything went fine:

        let mut comments = Vec::new();

        let batch = json.get("data").unwrap().as_array().unwrap();

        for raw_comment in batch {
            comments.insert(0, types::Comment::from(raw_comment.clone()));
        }


        Ok(comments)
    }
}
