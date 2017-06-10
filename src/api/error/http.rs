use std::option::Option;


pub struct Response {
    data: String,
    http_status: i32,
}



impl Response {
    pub fn new<T>() -> Result<Self> {
        Response {
            data: "dummy".to_string(),
            http_status: 200
        }
    }
}
