#[allow(unused)]
pub mod response;
pub mod postparams;
pub mod error;

use futures::future::Future;
use hyper;
use hyper::client;
use hyper::mime::Mime;
use serde_json;
use std;
use std::collections::HashMap;
use std::fmt::Debug;
use std::fmt::Error;
use std::io::Read;
use std::option::Option;
use std::process::exit;
use std::result::Result;
use tokio_core;





#[derive(Debug)]
pub struct Api {
    base_uri: &'static str,
    api_key: &'static str,
    login_token: String,
    client: hyper::client::Client<hyper::client::HttpConnector, hyper::Body>,
    user_agent: String,
}


#[allow(unused)]
impl Api {
    /// Create a new api client
    pub fn new(api_key: &'static str) -> Self {
        let crates_version = &std::env::var("CARGO_PKG_VERSION")
            .unwrap_or("unknown".to_string());
        let crates_name = std::env::var("CARGO_PKG_NAME").
            unwrap_or("unknown".to_string());

        let ua = format!("libproxer-rust({}/v{})", crates_name, crates_version);


        let mut core = tokio_core::reactor::Core::new().unwrap();

        Api {
            base_uri:    "http://proxer.me/api/v1/",
            api_key:     api_key,
            login_token: "None".to_string(),
            client:      hyper::client::Client::new(&core.handle()),
            user_agent:  ua,
        }
    }



    /// Get the full information for an anime or manga
    ///
    /// See [Proxer wiki](http://proxer.me/wiki/Proxer_API/v1/Info#Get_Full_Entry)

    pub fn info_get_full_info(self, id: u64) -> Result<serde_json::Value, error::Error> {
        let url = "info/fullentry";

        let mut post = postparams::Postparams::new();
        post.add("id", id);

        let response = self.http_req(url, &post);

        if response.is_err() {
            let http_error = error::http::Http(response.unwrap_err());
            let error = error::Error::Http(http_error);
            return Err(error);
        }

        Ok(serde_json::Value::Null)
    }




    //
    //
    // /// Get basic anime or manga information
    // ///
    // /// See [Proxer wiki](http://proxer.me/wiki/Proxer_API/v1/Info#Get_Entry)
    // pub fn info_get_entry(self, id: u32) -> Option<response::Response> {
    //     use api::entity::response::*;
    //     let url = "info/entry";
    //
    //     let mut post = postparams::Postparams::new();
    //     post.add("id", id);
    //
    //     let response = self.http_req(url, &post);
    //
    //     info::fullinfo::FullInfo::from_api(response)
    // }
    //
    //
    // /// Get the different names of an anime or manga
    // ///
    // /// See [Proxer wiki](http://proxer.me/wiki/Proxer_API/v1/Info#Get_Names)
    // pub fn info_get_names(self, id: u32) -> Option<response::Response> {
    //     use api::entity::response::*;
    //     let url = "info/names";
    //
    //     let mut post = postparams::Postparams::new();
    //     post.add("id", id);
    //
    //     let response = self.http_req(url, &post);
    //
    //     response::Response::new(response)
    // }








    fn http_req(self, url: &str, data: &postparams::Postparams) -> Result<client::Response, hyper::Error> {
        let uri = self.base_uri.to_string()+url;
        let hyper_url: hyper::Uri= uri.parse().unwrap();


        let mut req = hyper::Request::new(hyper::Method::Post, hyper_url);

        header! {(ProxerApiKeyHeader, "proxer-api-key") => [String]}

        req.headers_mut().set(ProxerApiKeyHeader(self.api_key.to_owned()));
        req.headers_mut().set(hyper::header::UserAgent::new(self.user_agent));
        req.headers_mut().set(hyper::header::ContentType::form_url_encoded());

        req.set_body(data.to_string());

        self.client.request(req).wait()
    }


    pub fn read_json(json: String) -> std::result::Result<serde_json::Value, serde_json::Error> {
        serde_json::from_str(&json)
    }
}
