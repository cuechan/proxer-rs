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



pub struct User<'a> {
    pub client: Client<'a>
}

//
// impl<'a> User<'a> {
//     pub fn get_userinfo(self, uid: UserID) -> Result<user::Userinfo, error::Error> {
//         let mut request = request::Request::new("user/userinfo");
//
//         request.set_parameter("uid", uid);
//
//
//         let res = self.client.execute(request);
//
//
//
//
//
//         if res.is_err() {
//             return Err(res.err().unwrap());
//         }
//
//
//         // JSON PARsING
//
//         let json;
//
//         match res.unwrap().json::<ApiResponse>() {
//             Ok(r) => json = r,
//             Err(e) => return Err(error::Error::Json)
//         }
//
//
//         // API ERROR CHECKING
//
//         if json.error != 0 {
//             return Err(error::Error::Api(error::api::Api::from(json)))
//         }
//
//
//         let userinfo = user::Userinfo::from(json.data.unwrap());
//
//         Ok(userinfo)
//     }
//
//
//
//
//
//     pub fn get_list(self, vars: request::user::GetList) -> Result<Vec<user::GetList>, error::Error> {
//         let mut request = request::Request::new("user/list");
//
//         match (vars.uid, vars.username) {
//             (Some(i), None)    => request.set_parameter("uid", i),
//             (None,    Some(i)) => request.set_parameter("username", i),
//             _                  => panic!("either username nor uid are given"),
//         }
//
//         match vars.p {
//             Some(i) => request.set_parameter("p", i),
//             None => request.set_parameter("p", ::api::DEFAULT_PAGER_PAGE)
//         }
//
//
//         match vars.limit {
//             Some(i) => request.set_parameter("limit", i),
//             None => request.set_parameter("limit", ::api::DEFAULT_PAGER_LIMIT)
//         }
//
//         match vars.sort {
//             Some(i) => request.set_parameter("sort", i),
//             None => request.remove_parameter("sort")
//         }
//
//
//
//
//
//
//
//
//         let res = self.client.execute(request);
//
//
//         if res.is_err() {
//             return Err(res.err().unwrap());
//         }
//
//
//         // JSON PARsING
//
//         let api_res = match res.unwrap().json::<ApiResponse>() {
//             Ok(r) => r,
//             Err(e) => return Err(error::Error::Json)
//         };
//
//
//         // API ERROR CHECKING
//
//         if api_res.error != 0 {
//             return Err(error::Error::Api(error::api::Api::from(api_res)))
//         }
//
//
//         let data = match api_res.data.unwrap().as_array() {
//             None => return Err(error::Error::Unknown),
//             Some(x) => x.clone(),
//         };
//
//
//         let mut all_comments = Vec::new();
//         for com in data {
//             all_comments.push(user::GetList::from(com));
//         }
//
//         Ok(all_comments)
//     }
// }



pub struct Userinfo {
    data: HashMap<String, String>
}

impl Userinfo {
    fn new(vars: parameter::user::Userinfo) -> Self {
        let mut data = HashMap::new();

        match (vars.uid, vars.username) {
            (Some(i), None) => data.insert("uid".to_string(), i.to_string()),
            (None, Some(i)) => data.insert("username".to_string(), i.to_string()),
            _ => panic!("either username nor uid are given"),
        };

        Self {
            data: data
        }
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