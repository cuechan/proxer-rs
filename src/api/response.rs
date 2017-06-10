use std::option::Option;
use hyper;
use hyper::client::response;


pub struct Response {
    data: String,
    http_status: i32,
}



impl Response {
    pub fn new<T>(hyper_res: Result<response::Response, hyper::Error>) -> Result<response::Response, hyper::Error> {
        match hyper_res {
            Ok(e) => {
                Ok(e)
            }
            Err(e) => {
                Err(e)
            }
        }
    }
}
