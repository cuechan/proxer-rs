#![allow(dead_code)]
#![deny(unused_imports)]
#![allow(warnings)]
#![allow(unused)]


use error;
use error::api;
use request;
use request::info::*;
use reqwest;
use reqwest::IntoUrl;
use response;
use serde_derive;
use serde_json;
use serde_json::Value;
use std;
use std::collections::HashMap;
use std::process::exit;
use std::result::Result;
use std::thread;
use std::time;
use types;
use chrono;
use Api;

// low level stuff and api endpoint definitions








pub fn info_get_fullentry<'a>(data: GetFullEntry) -> request::Request<'a> {
    let mut request = request::Request::new("info/fullentry");

    request.set_parameter("id", data.to_string());
    println!("foobar");

    request
}
