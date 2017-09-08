#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(warnings)]
#![allow(unused)]

#[macro_use]
extern crate log;
#[macro_use]
extern crate reqwest;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate chrono;
extern crate futures;
extern crate serde_json;
extern crate tokio_core;

pub mod error;
pub mod types;
pub mod request;
pub mod api;
pub mod prelude;

use prelude::*;
use reqwest::{Url, Method};
use reqwest::IntoUrl;
use reqwest::Request;
use reqwest::header;
use serde_json::Value;
use std::collections::HashMap;
use std::process::exit;
use std::result::Result;
use std::thread;
use std::time;



enum Info {
    Entry,
    FullEntry
}


enum User {
    Info
}
