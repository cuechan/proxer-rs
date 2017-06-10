#[allow(unused)]
pub mod response;
pub mod entity;
pub mod http;

use std;
use std::option::Option;
use std::result::Result;
use hyper;
use hyper::client;
use std::collections::HashMap;
use std::fmt::Debug;
use std::io::Read;
use serde_json;
use std::process::exit;





#[derive(Debug)]
pub struct Api {
    base_uri: &'static str,
    api_key: String,
    login_token: String,
    client: hyper::client::Client,
    version: String,
    user_agent: String,
}


#[allow(unused)]
impl Api {

    /// Create a new api client
    pub fn new(api_key: String) -> Api {
        let crates_version = &std::env::var("CARGO_PKG_VERSION")
            .unwrap_or("unknown".to_string());
        let crates_name = std::env::var("CARGO_PKG_NAME").
            unwrap_or("unknown".to_string());

        let ua = format!("libproxer-rust({}/v{})", crates_name, crates_version);


        Api {
            base_uri:    "http://proxer.me/api/v1/",
            api_key:     api_key,
            login_token: "None".to_string(),
            client:      hyper::client::Client::new(),
            user_agent:  ua,
            version:     std::env::var("HOME").unwrap()
        }
    }


    pub fn info_get_full_info(self, id: u32) -> Result<client::response::Response, hyper::Error> {
        use api::entity::response::*;
        let url = "info/fullentry";

        response::Response::new::<info::fullinfo::FullInfo>(self.http_req(url))
    }








    fn http_req(self, url: &str) -> Result<client::response::Response, hyper::Error> {
        let uri = self.base_uri.to_string()+url;
        let hyper_url = hyper::Url::parse(&uri).unwrap();
        header! {(ProxerApiKeyHeader, "proxer-api-key") => [String]}
        header! {(UserAgent, "User-Agent") => [String]}

        let mut headers = hyper::header::Headers::new();
        headers.set(ProxerApiKeyHeader(self.api_key));
        headers.set(ProxerApiKeyHeader(self.user_agent));


        self.client
            .post(hyper_url)
            .headers(headers)
            .send()
    }


    pub fn read_json(json: String) -> std::result::Result<serde_json::Value, serde_json::Error> {
        serde_json::from_str(&json)
    }
}




fn hashmap_to_encoded_url(params: HashMap<String, String>) -> String{
    "foobar".to_string()
}
