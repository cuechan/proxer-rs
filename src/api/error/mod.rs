pub mod api;

use std::option::Option;


#[derive(Debug, Clone)]
pub enum Error {
    Api(api::Api),
    Json,
    Http,
    Unknown,
}


impl Error {
    pub fn is_api_error(self) -> bool {
        match self {
            Error::Api(_) => true,
            Error::Json => false,
            Error::Http => false,
            _ => false
        }
    }

    pub fn is_http_err(self) -> bool {
        match self {
            Error::Api(_) => false,
            Error::Json => false,
            Error::Http => true,
            _ => false
        }
    }

    pub fn is_json_error(self) -> bool {
        match self {
            Error::Api(_) => false,
            Error::Json => true,
            Error::Http => false,
            _ => false
        }
    }
}
