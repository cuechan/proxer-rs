pub mod api;
pub mod http;
pub mod json;

use hyper;
use hyper::client;
use std::option::Option;


pub enum Error {
    Http(http::Http),
    Api(api::Api),
    Json(json::Json)
}


impl Error {
    pub fn is_api_error(self) -> bool {
        match self {
            Error::Api(_) => true,
            Error::Http(_) => false,
            Error::Json(_) => false
        }
    }

    pub fn is_http_err(self) -> bool {
        match self {
            Error::Api(_) => false,
            Error::Http(_) => true,
            Error::Json(_) => false
        }
    }

    pub fn is_json_error(self) -> bool {
        match self {
            Error::Api(_) => false,
            Error::Http(_) => false,
            Error::Json(_) => true
        }
    }
}