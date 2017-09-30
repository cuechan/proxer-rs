#![allow(missing_docs)]


use chrono;
use error;
use error::api;
use prelude::*;
use request;
use request::parameter;
use reqwest;
use reqwest::IntoUrl;
use response::info;
use serde_derive;
use serde_json;
use serde_json::Value;
use std;
use std::collections::HashMap;
use std::ops::Deref;
use std::process::exit;
use std::rc::Rc;
use std::thread;
use std::time;
use client::Client;
use response;
use Request;



pub struct Info<'a> {
    pub client: Client<'a>
}


impl<'a> Info<'a> {

}




#[derive(Debug, Clone)]
pub struct GetFullEntry {
    data: HashMap<String, String>
}



impl GetFullEntry {
    pub fn new(vars: parameter::info::GetFullEntry) -> Self {
        let mut data = HashMap::new();

        data.insert("id".to_string(), vars.0.to_string());

        Self {data: data}
    }
}


impl Request for GetFullEntry {
    type RequestType = parameter::info::GetFullEntry;
    type ResponseType = response::info::FullEntry;




    fn get_url(self) -> String {
        "info/fullentry".to_string()
    }

    fn get_data(self) -> HashMap<String, String> {
        self.data
    }
}




pub struct GetComments {
    data: HashMap<String, String>
}
