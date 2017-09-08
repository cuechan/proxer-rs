#![allow(dead_code)]
#![allow(warnings)]
#![allow(unused)]


pub mod info;

use Proxer;
use chrono;
use error;
use error::api;
use request;
use reqwest;
use reqwest::IntoUrl;
use serde_derive;
use serde_json;
use serde_json::Value;
use std;
use std::collections::HashMap;
use std::process::exit;
use std::thread;
use std::time;
use types;


// low level stuff and api endpoint definitions

pub struct Api (pub Proxer);


impl Api {
    pub fn info(self) -> info::Info {
        info::Info(self.0)
    }
}
