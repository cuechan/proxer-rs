#![warn(missing_docs)]


#[macro_use]
extern crate log;
extern crate reqwest;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate chrono;
extern crate serde_json;

pub mod error;
pub mod response;
pub mod request;
pub mod api;
pub mod prelude;
pub mod client;

pub use client::Client;
use serde_json::Value;
use std::collections::HashMap;



/// Trait that represents an api endpoint
pub trait Request {
    type RequestType;
    type ResponseType: From<Value>;

    fn parse(&self, json: Value) -> Self::ResponseType {
        Self::ResponseType::from(json)
    }
    fn get_url(self) -> String;
    fn get_data(self) -> HashMap<String, String>;
}

// Trait that requeres a function that parses the request to
// to an freely chosable datatype
