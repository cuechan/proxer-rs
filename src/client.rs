#![warn(missing_docs)]


use error;
use reqwest;
use reqwest::Url;
use reqwest::header;
use serde_json::Value;
use std;
use api;
use Request;




const API_BASE_PATH: &str = "http://proxer.me/api/v1/";


/// Client that holds the api key and sends requests
#[derive(Debug, Clone)]
pub struct Client<'a> {
    api_key: &'a str,
    base_uri: &'a str,
    user_agent: String,
}




#[allow(unused)]
impl<'a> Client<'a> {
    /// Create a new api client
    pub fn new(api_key: &'a str) -> Self {
        let crates_version = &std::env::var("CARGO_PKG_VERSION").unwrap_or("unknown".to_string());
        let crates_name = std::env::var("CARGO_PKG_NAME").unwrap_or("unknown".to_string());


        let ua = format!("libproxer-rust({}/v{})", crates_name, crates_version);

        Self {
            api_key: api_key,
            base_uri: API_BASE_PATH,
            user_agent: ua,
        }
    }



    /// execute a request that satisfies [Request](../trait.Request.html)
    pub fn execute<T: Request + Clone>(self, endpoint: T) -> Result<Value, error::Error> {
        let url = Url::parse(&(self.base_uri.to_string() + &endpoint.clone().get_url())).unwrap();




        let mut headers = reqwest::header::Headers::new();

        headers.set(header::UserAgent::new(self.user_agent.to_string()));
        headers.set(reqwest::header::ContentType::form_url_encoded());


        let mut parameter = endpoint.get_data().clone();
        parameter.insert("api_key".to_string(), self.api_key.to_string());

        let client = reqwest::Client::new().unwrap();


        let response = client
            .post(url)
            .unwrap()
            .form(&parameter)
            .unwrap()
            .headers(headers)
            .send();


        // let api_response;

        match response {
            Err(e) => return Err(error::Error::Http),
            // I know, it looks horrible down here...
            Ok(mut res) => {
                match res.json::<ApiResponse>() {
                    Err(e) => return Err(error::Error::Json),
                    Ok(r) => Ok(r.data.unwrap()),
                }
            }
        }

        //
        // println!("{:#?}", api_response);
        //
        // return Ok(endpoint.parse(api_response.data.unwrap()));
        // Err(error::Error::Unknown)
    }

    /// shortcut for creating common requests
    pub fn api(&self) -> api::Api<'a> {
        api::Api { client: self.clone() }
    }
}


/// responses are parsed into ApiRequest
#[derive(Debug, Clone, Deserialize)]
pub struct ApiResponse {
    /// api error
    pub error: i64,
    /// api message
    pub message: String,
    /// actual data
    pub data: Option<Value>,
    /// optional error code
    /// in case of an error this contains the error code
    pub code: Option<i64>,
}
