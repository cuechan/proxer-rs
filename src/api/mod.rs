#![allow(unused)]
pub mod error;
pub mod postparams;
pub mod types;

use hyper;
use reqwest;
use reqwest::IntoUrl;
use serde_json;
use std;
use std::collections::HashMap;
use std::process::exit;
use std::result::Result;
use std::thread;
use std::time;
use tokio_core;




#[derive(Debug)]
pub struct Api {
    api_key: &'static str,
    base_uri: &'static str,
    client: reqwest::Client,
    last_request: time::Instant,
    login_token: Option<bool>,
    user_agent: String,
    api_request_sleep: time::Duration,
}



// Struct for storing api response without parsing type specific data
#[derive(Debug, Deserialize)]
struct Response {
    pub error: i64,
    pub message: String,
    pub data: serde_json::Value,
}


#[allow(unused)]
impl Api {
    /// Create a new api client
    pub fn new(api_key: &'static str) -> Self {
        let crates_version = &std::env::var("CARGO_PKG_VERSION").unwrap_or("unknown".to_string());
        let crates_name = std::env::var("CARGO_PKG_NAME").unwrap_or("unknown".to_string());


        let ua = format!("libproxer-rust({}/v{})", crates_name, crates_version);


        Api {
            api_key:           api_key,
            base_uri:          "http://proxer.me/api/v1/",
            client:            reqwest::Client::new().expect("failed to create client"),
            last_request:      time::Instant::now(),
            login_token:       None,
            user_agent:        ua,
            api_request_sleep: time::Duration::from_secs(5),
        }
    }


    fn http_req(&mut self, url: &str, mut data: HashMap<&str, String>) -> reqwest::Result<reqwest::Response> {
        let uri = self.base_uri.to_string()+url;


        data.insert("api_key", self.api_key.to_owned());

        let mut headers: reqwest::header::Headers = reqwest::header::Headers::new();
        headers.set(reqwest::header::UserAgent(self.user_agent.to_owned()));
        headers.set(reqwest::header::ContentType::form_url_encoded());

        let mut req = self.client.request(reqwest::Method::Post, uri.into_url().unwrap());


        // check if we have to sleep

        thread::sleep(self.need_sleep());
        self.last_request = time::Instant::now();


        let foo = req.headers(headers).form(&data);
        foo.send()
    }


    fn need_sleep(&self) -> time::Duration {
        let delta = std::time::Instant::now() - self.last_request;

        if delta > self.api_request_sleep {
            time::Duration::from_secs(0)
        }
        else {
            self.api_request_sleep - delta
        }
    }






    // Here the api access methods begin
    

    /// Get the full information for an anime or manga
    ///
    /// See [Proxer wiki](http://proxer.me/wiki/Proxer_API/v1/Info#Get_Full_Entry)

    pub fn info_get_full_info(&mut self, id: i64) -> Result<types::FullInfo, error::Error> {
        let url = "info/fullentry";
        // id.to_owned();


        let mut post = HashMap::<&str, String>::new();
        post.insert("id", id.to_string());


        let http_res = self.http_req(url, post);

        let res: Response = match serde_json::from_reader(http_res.unwrap()) {
            Err(e) => return Err(error::Error::Json),
            Ok(r) => r,
        };


        types::FullInfo::from_api(res.data)
    }

}
