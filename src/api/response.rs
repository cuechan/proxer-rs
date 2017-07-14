use reqwest;
use serde_json;
use std;
use std::io::Read;
use std::option::Option;
use std::result::Result;



#[derive(Debug)]
pub struct Response {
    error: i64,
    json: serde_json::Value,
    message: String,
}



impl Response {
    pub fn from_response(response: reqwest::Response) -> Option<Self> {
        let json: serde_json::Value = serde_json::from_reader(response).unwrap();

        let data = json.as_object().unwrap();


        // Parsing: check values existence and types

        let error = match data.get("error") {
            Some(e) => {
                match e.as_i64() {
                    Some(e) => e,
                    None => return None,
                }
            }
            None => return None,
        };

        let message = match data.get("message") {
            Some(e) => {
                match e.as_str() {
                    Some(e) => e,
                    None => return None,
                }
            }
            None => return None,
        };


        if error > 0 {
            return None;
        }


        Some(Self {
            error: error,
            json: json.to_owned(),
            message: message.to_string(),
        })
    }


    pub fn is_err(&self) -> bool {
        if self.error > 0 {
            true
        }
        else {
            false
        }
    }


    pub fn get_data(&self) -> Option<serde_json::Value> {
        if self.is_err() {
            None
        }
        else {
            Some(
                self.json.to_owned()
            )
        }
    }



}
