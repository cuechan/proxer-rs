#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(warnings)]


#[macro_use] extern crate log;
#[macro_use] extern crate reqwest;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate serde;
extern crate futures;
extern crate hyper;
extern crate serde_json;
extern crate tokio_core;

pub mod api;
