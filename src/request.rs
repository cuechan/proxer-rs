#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(warnings)]
#![allow(unused)]


use error;
use reqwest;
use reqwest::IntoUrl;
use response::Response;
use serde_derive;
use serde_json;
use serde;
use serde::Serializer;
use std;
use std::collections::HashMap;
use std::process::exit;
use std::result::Result;
use std::thread;
use std::time;
use types;




#[derive(Debug, Clone)]
pub struct Request<'a> {
    pub url: &'a str,
    parameter: HashMap<&'a str, String>,
}



impl<'a> Request<'a> {
    pub fn new(url: &'a str) -> Self {
        Self {
            parameter: HashMap::new(),
            url: url,
        }
    }


    pub fn get_paramter(self) -> HashMap<&'a str, String> {
        self.parameter.to_owned()
    }


    pub fn get_url(self) -> &'a str {
        self.url
    }


    pub fn add_parameter(&mut self, parameter: &'a str, value: String) {
        self.parameter.insert(parameter, value);
    }


    pub fn remove_parameter(&mut self, parameter: &str) {
        self.parameter.remove(parameter);
    }
}
