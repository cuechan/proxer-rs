#[allow(unused)]
pub mod response;
pub mod postparams;
pub mod error;

use hyper;
use reqwest;
use reqwest::IntoUrl;
use serde_json;
use std;
use std::process::exit;
use std::result::Result;
use tokio_core;
use std::collections::HashMap;





#[derive(Debug)]
pub struct Api {
    api_key: &'static str,
    base_uri: &'static str,
    client: reqwest::Client,
    login_token: String,
    user_agent: String,
}


#[allow(unused)]
impl Api {
    /// Create a new api client
    pub fn new(api_key: &'static str) -> Self {
        let crates_version = &std::env::var("CARGO_PKG_VERSION").unwrap_or("unknown".to_string());
        let crates_name = std::env::var("CARGO_PKG_NAME").unwrap_or("unknown".to_string());


        let ua = format!("libproxer-rust({}/v{})", crates_name, crates_version);


        Api {
            api_key:     api_key,
            base_uri:    "http://proxer.me/api/v1/",
            client:      reqwest::Client::new().expect("failed to create client"),
            login_token: "None".to_string(),
            user_agent:  ua
        }
    }



    /// Get the full information for an anime or manga
    ///
    /// See [Proxer wiki](http://proxer.me/wiki/Proxer_API/v1/Info#Get_Full_Entry)

    pub fn info_get_full_info(&mut self, id: u64) -> Option<response::Response> {
        let url = "info/fullentry";
        id.to_owned();


        let mut post = HashMap::<&str, String>::new();
        post.insert("id", id.to_string());


        let res = self.http_req(url, post);
        warn!("request done");

        let response = res.unwrap();

        warn!("{:?}", response.status());

        response::Response::from_response(response)
    }







    fn http_req(&mut self, url: &str, mut data: HashMap<&str, String>) -> reqwest::Result<reqwest::Response> {
        let uri = self.base_uri.to_string()+url;


        data.insert("api_key", self.api_key.to_owned());

        let mut headers: reqwest::header::Headers = reqwest::header::Headers::new();
        headers.set(reqwest::header::UserAgent(self.user_agent.to_owned()));
        headers.set(reqwest::header::ContentType::form_url_encoded());

        let mut req = self.client.request(reqwest::Method::Post, uri.into_url().unwrap());


        let foo = req.headers(headers).form(&data);
        foo.send()
    }

    pub fn read_json(json: String) -> std::result::Result<serde_json::Value, serde_json::Error> {
        serde_json::from_str(&json)
    }
}
