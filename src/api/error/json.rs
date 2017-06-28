use std::option;
use serde_json;



pub struct Json (pub serde_json::error::Error);


impl Json {
    pub fn new(serde_error: serde_json::error::Error) -> Self {
        Json(serde_error)
    }
}
