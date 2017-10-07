#![allow(dead_code)]
#![deny(unused_imports)]
#![allow(warnings)]
#![allow(unused)]
#![warn(missing_docs)]


use client::Client;
use chrono;
use error;
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
use prelude::*;
use std::rc::Rc;
use std::ops::Deref;
use response::user;
use request::parameter;
use Request;
use response;





pub struct Userinfo {
	data: HashMap<String, String>,
}

impl Userinfo {
	fn new(vars: parameter::user::Userinfo) -> Self
	{
		let mut data = HashMap::new();

		match (vars.uid, vars.username)
		{
			(Some(i), None) => data.insert("uid".to_string(), i.to_string()),
			(None, Some(i)) => data.insert("username".to_string(), i.to_string()),
			_ => panic!("either username nor uid are given"),
		};

		Self { data: data }
	}
}

// impl Request for Userinfo {
//     type RequestType = parameter::user::Userinfo;
//     type ResponseType = response::user::Userinfo;
//
//     fn get_url(self) -> String {
//         "user/userinfo".to_string()
//     }
//
//     fn get_data(self) -> HashMap<String, String> {
//         self.data
//     }
//
//     fn parse(json: Value) -> Option<Self::ResponseType> {
//
//     }
// }
