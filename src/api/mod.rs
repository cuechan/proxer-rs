#![allow(dead_code)]
#![allow(warnings)]
#![allow(unused)]


pub mod info;
pub mod user;

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
use std::rc::Rc;


pub const DEFAULT_PAGER_PAGE: i64 = 0;
pub const DEFAULT_PAGER_LIMIT: i64 = 250;



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

    pub fn user(self) -> user::User<'a> {
        user::User {
            proxer: self.proxer
        }
    }
}
