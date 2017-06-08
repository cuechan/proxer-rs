#[allow(unused)]
pub mod response;
pub mod entity;

use std;
use hyper;
use hyper::method;
use std::collections::HashMap;
use std::fmt::Debug;
use std::io::Read;
use serde_json;





#[derive(Debug)]
pub struct Api {
    api_key: String,
    login_token: String,
}


#[allow(unused)]
impl Api {
    pub fn new(api_key: String) -> Api {
        Api {
            api_key: api_key,
            login_token: "NULL".to_string()
        }
    }


    pub fn info_get_full_info(self) -> std::option::Option<response::Response> {
        let url = "info/fullentry";

        let res = read_json(self.http_req(url));



        Some(
            response::Response::new::<entity::response::info::fullinfo::FullInfo>()
        )
    }








    fn http_req(self, url: &'static str) -> String {
        let url = hyper::Url::parse("http://proxer.me/api/v1/info/fullentry").unwrap();
        header! {(ProxerApiKeyHeader, "proxer-api-key") => [String]}
        let mut request = hyper::client::request::Request::new(method::Method::Post, url).unwrap();



        let params: HashMap<String, String> = HashMap::new();





        let mut res = request.start().unwrap().send().unwrap();

        assert_eq!(res.status, hyper::Ok);
        let mut res_text = String::new();
        res.read_to_string(&mut res_text).unwrap();

        res_text
    }


    pub fn read_json(json: String) -> std::result::Result<serde_json::Value, serde_json::Error> {
        serde_json::from_str(&json)
    }
}




fn hashmap_to_encoded_url(params: HashMap<String, String>) -> String{
    "foobar".to_string()
}
