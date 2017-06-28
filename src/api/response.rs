use std;
use std::option::Option;
use hyper::client;
use serde_json;




pub struct Response {
    json: serde_json::Value,
}



impl Response {
    pub fn new(json: String) -> Self {
        Response {
            json: serde_json::from_str(&json).unwrap()
        }
    }
    pub fn new_hyper_res(json: String) -> Self {
        Response {
            json: serde_json::from_str(&json).unwrap()
        }
    }
}
