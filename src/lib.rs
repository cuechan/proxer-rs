#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(warnings)]


#[macro_use] extern crate reqwest;
#[macro_use] extern crate log;
extern crate hyper;
extern crate futures;
extern crate serde_json;
extern crate tokio_core;

pub mod api;
