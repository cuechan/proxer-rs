#![allow(dead_code)]
#![allow(warnings)]
#![allow(unused)]


pub mod info;
pub mod user;

use error;
use reqwest;
use reqwest::IntoUrl;
use serde_derive;
use serde_json;
use serde_json::Value;
use serde;
use serde::Serializer;
use std;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::iter::Chain;
use std::iter::Cloned;
use std::iter::Cycle;
use std::iter::Enumerate;
use std::iter::Filter;
use std::iter::FilterMap;
use std::iter::FlatMap;
use std::iter::FromIterator;
use std::iter::Fuse;
use std::iter::Inspect;
use std::iter::Map;
use std::iter::Peekable;
use std::iter::Product;
use std::iter::Rev;
use std::iter::Scan;
use std::iter::Skip;
use std::iter::SkipWhile;
use std::iter::Sum;
use std::iter::Take;
use std::iter::TakeWhile;
use std::iter::Zip;
use std::process::exit;
use std::thread;
use std::time;









pub struct Pager<'a> {
    request: Request<'a>,
    page: Option<u64>,
    limit: Option<u64>,
}









#[derive(Debug, Clone)]
pub struct Request<'a> {
    url: &'a str,
    parameter: HashMap<&'a str, String>,
}



impl<'a> Request<'a> {
    pub fn new(url: &'a str) -> Self {
        Self {
            url: url,
            parameter: HashMap::new()
        }
    }

    pub fn set_parameter<T: ToString>(&mut self, parameter: &'a str, value: T) {
        self.parameter.insert(parameter, value.to_string());
    }


    pub fn remove_parameter(&mut self, parameter: &str) {
        self.parameter.remove(parameter);
    }


    pub fn get_paramter(self) -> HashMap<&'a str, String> {
        self.parameter.to_owned()
    }


    pub fn get_url(self) -> &'a str {
        self.url
    }

}
