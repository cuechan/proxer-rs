use std;
use hyper;
use hyper::method;
use std::collections::HashMap;
use std::fmt::Debug;
use std::io::Read;
use serde_json;
use serde_json::{Value, Error};



#[allow(unused)]
#[derive(Debug)]
pub struct Api {
    api_key: String,
}


#[allow(unused)]
impl Api {
    pub fn new(api_key: String) -> Api {
        Api {
            api_key: api_key,
        }
    }







    fn http_req(self) -> Self {
        let url = hyper::Url::parse("http://proxer.me/api/v1/info/fullentry").unwrap();
        header! {(ProxerApiKeyHeader, "proxer-api-key") => [String]}
        let mut request = hyper::client::request::Request::new(method::Method::Post, url).unwrap();



        let params: HashMap<String, String> = HashMap::new();





        let mut res = request.start().unwrap().send().unwrap();

        assert_eq!(res.status, hyper::Ok);
        let mut res_text = String::new();
        res.read_to_string(&mut res_text).unwrap();

        let v: Value = serde_json::from_str(&res_text).unwrap();


        if v["error"] == 1 {
            println!("Api error: {}", v["message"]);
        }
        else {
            println!("{:?}", v["data"]);
        }

        return self;
    }
}




fn hashmap_to_encoded_url(params: HashMap<String, String>) -> String{
    "foobar".to_string()
}
