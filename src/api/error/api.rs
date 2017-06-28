use hyper;
use hyper::client;
use std::option::Option;


pub struct Api {
    code: i64,
    message: String
}


impl Api {
    pub fn new(res: client::Response) -> Self {
        Api {
            code: 0,
            message: "very error".to_string()
        }
    }
}
