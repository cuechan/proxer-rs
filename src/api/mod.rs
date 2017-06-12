#[allow(unused)]
pub mod entity;
pub mod http;
pub mod response;
pub mod postparams;

extern crate hyper;
extern crate serde_json;
extern crate urlencoded;

use hyper::client;
use hyper::mime::Mime;
use std;
use std::collections::HashMap;
use std::fmt::Debug;
use std::io::Read;
use std::option::Option;
use std::process::exit;
use std::result::Result;





#[derive(Debug)]
pub struct Api {
    base_uri: &'static str,
    api_key: String,
    login_token: String,
    client: hyper::client::Client,
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
        }
    }


    /// Get the full information for an anime or manga
    ///
    /// See [Proxer wiki](http://proxer.me/wiki/Proxer_API/v1/Info#Get_Full_Entry)

    pub fn info_get_full_info(self, id: u32) -> Result<client::response::Response, hyper::Error> {
        use api::entity::response::*;
        let url = "info/fullentry";

        let mut post = postparams::Postparams::new();
        post.add("id", id);

        let response = self.http_req(url, &post);

        response
    }


    /// Get basic anime or manga information
    ///
    /// See [Proxer wiki](http://proxer.me/wiki/Proxer_API/v1/Info#Get_Entry)
    pub fn info_get_entry(self, id: u32) -> Result<client::response::Response, hyper::Error> {
        use api::entity::response::*;
        let url = "info/entry";

        let mut post = postparams::Postparams::new();
        post.add("id", id);

        let response = self.http_req(url, &post);

        response
    }


    /// Get the different names of an anime or manga
    ///
    /// See [Proxer wiki](http://proxer.me/wiki/Proxer_API/v1/Info#Get_Names)
    pub fn info_get_names(self, id: u32) -> Result<client::response::Response, hyper::Error> {
        use api::entity::response::*;
        let url = "info/names";

        let mut post = postparams::Postparams::new();
        post.add("id", id);

        let response = self.http_req(url, &post);

        response
    }








    fn http_req(self, url: &str, data: &postparams::Postparams) -> Result<client::response::Response, hyper::Error> {
        let uri = self.base_uri.to_string()+url;
        let hyper_url = hyper::Url::parse(&uri).unwrap();


        let mut headers = hyper::header::Headers::new();
        header! {(ProxerApiKeyHeader, "proxer-api-key") => [String]}
        header! {(UserAgent, "User-Agent") => [String]}
        headers.set(ProxerApiKeyHeader(self.api_key));
        headers.set(UserAgent(self.user_agent));
        headers.set(hyper::header::ContentType::form_url_encoded());


        self.client
            .post(hyper_url)
            .headers(headers)
            .body(&data.to_string())
            .send()
    }


    pub fn read_json(json: String) -> std::result::Result<serde_json::Value, serde_json::Error> {
        serde_json::from_str(&json)
    }
}
