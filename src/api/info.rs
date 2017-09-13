#![allow(dead_code)]
#![deny(unused_imports)]
#![allow(warnings)]
#![allow(unused)]


use Proxer;
use ApiResponse;
use chrono;
use error;
use error::api;
use request;
use request::info::*;
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
use types;
use prelude::*;
use std::rc::Rc;
use std::ops::Deref;


pub struct Info<'a> {
    pub proxer: Proxer<'a>
}


impl<'a> Info<'a> {
    pub fn get_fullentry(self, eid: InfoID) -> Result<types::FullEntry, error::Error> {
        let mut request = request::Request::new("info/fullentry");

        let proxer = self.proxer;

        request.set_parameter("id", eid);


        let res = proxer.execute(request);





        if res.is_err() {
            return Err(res.err().unwrap());
        }


        // JSON PARsING

        let json;

        match res.unwrap().json::<ApiResponse>() {
            Ok(r) => json = r,
            Err(e) => return Err(error::Error::Json)
        }


        // API ERROR CHECKING

        if json.error != 0 {
            return Err(error::Error::Api(error::api::Api::from(json)))
        }


        let fullentry = types::FullEntry::from(json.data.unwrap());

        Ok(fullentry)
    }
}
