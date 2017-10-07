#![allow(dead_code)]
#![allow(unused)]



use error;
use reqwest;
use reqwest::IntoUrl;
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
use prelude::*;
use request::parameter;
use request::IsRequest;



#[derive(Debug, Clone)]
pub struct GetEntryList<'a> {
    endpoint: &'a str,
    parameter: parameter::list::GetList,
}


impl<'a> GetEntryList<'a> {
    pub fn new(data: parameter::list::GetList) -> Self {
        Self {
            endpoint: "list/entrylist".to_string(),
            parameter: data
        }
    }
}
