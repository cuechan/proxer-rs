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
use std::rc::Rc;


// low level stuff and api endpoint definitions


#[derive(Debug, Clone)]
pub struct Api<'a> {
    pub proxer: Proxer<'a>
}


impl<'a> Api<'a> {
    pub fn info(self) -> info::Info<'a> {
        info::Info {
            proxer: self.proxer
        }
    }
}
