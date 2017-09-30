#![allow(dead_code)]


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




#[derive(Debug, Clone)]
pub struct GetList {
    pub kat: Option<String>,
    pub medium: Option<String>,
    pub is_h: Option<String>,
    pub start: Option<String>,
    pub sort: Option<String>,
    pub sort_type: Option<String>,
    pub p: Option<i64>,
    pub limit: Option<i64>,
}


impl Iterator for GetList {
    type Item = GetList;

    fn next(&mut self) -> Option<Self::Item> {
        match self.p {
            Some(p) => {
                self.p = Some(p+1);
                Some(self.clone())
            }
            None => {
                self.p = Some(0);
                Some(self.clone())
            }
        }
    }
}
