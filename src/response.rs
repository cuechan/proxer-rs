#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(warnings)]
#![allow(unused)]


use reqwest;
use request;
use reqwest::IntoUrl;
use serde_derive;
use serde_json;
use std;
use std::collections::HashMap;
use std::process::exit;
use std::result::Result;
use std::thread;
use types;
use std::time;
use error;




// Struct for storing api response without parsing type specific data
#[derive(Debug)]
pub struct Response<'a> {
    pub error: Option<i64>,
    pub message: Option<String>,
    pub data: Option<serde_json::Value>,
    pub request: request::Request<'a>,
}


#[allow(unused)]
impl<'a> Response<'a> {
    pub fn new(request: request::Request<'a>) -> Self {
        Self {
            error: None,
            message: None,
            data: None,
            request: request
        }
    }
}




impl<'a> Into<types::FullInfo> for Response<'a> {
    fn into(self) -> types::FullInfo {
        unimplemented!()
    }
}
