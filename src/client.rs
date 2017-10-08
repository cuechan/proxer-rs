use error;
use reqwest;
use reqwest::Url;
use reqwest::header;
use serde_json::Value;
use std;
use api;
use std::collections::HashMap;




const API_BASE_PATH: &str = "http://proxer.me/api/v1/";


/// Client that holds the api key and sends requests
#[derive(Debug, Clone)]
pub struct Client {
	api_key: String,
	base_uri: String,
	user_agent: String,
}




#[allow(unused)]
impl Client {
	/// Create a new api client
	pub fn new(api_key: String) -> Self
	{
		let crates_version = &std::env::var("CARGO_PKG_VERSION").unwrap_or("unknown".to_string());
		let crates_name = std::env::var("CARGO_PKG_NAME").unwrap_or("unknown".to_string());


		let ua = format!("libproxer-rust({}/v{})", crates_name, crates_version);

		Self {
			api_key: api_key,
			base_uri: API_BASE_PATH.to_string(),
			user_agent: ua,
		}
	}



	/// execute a request that satisfies [Request](../trait.Request.html)
	pub fn execute(self, url: String, vars: HashMap<String, String>) -> Result<Value, error::Error>
	{
		let url = Url::parse(&(self.base_uri.to_string() + &url)).unwrap();





		let mut headers = reqwest::header::Headers::new();

		headers.set(header::UserAgent::new(self.user_agent.to_string()));
		headers.set(reqwest::header::ContentType::form_url_encoded());


		let mut parameter = vars.clone();
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

		match response
		{
			Err(e) => return Err(error::Error::Http),
			Ok(mut res) => {
				match res.json::<ApiResponse>()
				{
					Err(e) => return Err(error::Error::Json),
					Ok(r) => Ok(r.data.unwrap()),
				}
			}
		}
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
