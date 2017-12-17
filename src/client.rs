use error;
use reqwest;
use reqwest::header;
use serde_json;
use serde_urlencoded;
use std;
use std::env;
use std::fmt;
use std::io::Read;





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
		let crates_version = &std::env::var("CARGO_PKG_VERSION")
			.unwrap_or("unknown".to_string());

		let crates_name = std::env::var("CARGO_PKG_NAME")
			.unwrap_or("unknown".to_string());


		let ua = format!("libproxer-rust({}/v{})", crates_name, crates_version);

		Self {
			api_key: api_key,
			base_uri: API_BASE_PATH.to_string(),
			user_agent: ua,
		}
	}


	pub fn with_env_key(env_key: &str) -> Option<Client>
	{
		match env::var_os(env_key)
		{
			Some(r) => Some(Client::new(r.into_string().unwrap())),
			None => None,
		}
	}



	/// execute a request that satisfies [Request](../trait.Request.html)
	pub fn execute<'a, T: super::Endpoint + Clone + fmt::Debug>(
		&self,
		mut endpoint: T,
	) -> Result<T::ResponseType, error::Error>
	{

		let uristring = self.base_uri.to_string() + T::URL;


		header!{(ProxerApiKey, "Proxer-Api-Key") => [String]}

		let mut headers = reqwest::header::Headers::new();

		headers.set(header::UserAgent::new(self.user_agent.to_string()));
		headers.set(header::Host::new("proxer.me", None));
		headers.set(header::ContentType::form_url_encoded());
		headers.set(ProxerApiKey(self.api_key.clone()));



		let mut http_req = reqwest::Request::new(
			reqwest::Method::Post,
			uristring.parse().unwrap(),
		);

		// add our headers to the request
		http_req.headers_mut().extend(headers.iter());

		match serde_urlencoded::to_string(endpoint.params_mut().clone())
		{
			Ok(body) => {
				debug!("post data: {}", body);
				*http_req.body_mut() = Some(body.into());
			}
			Err(err) => panic!("can't serialize form parameters"),
		}


		let client = reqwest::Client::new();

		let response = client.execute(http_req);





		// This section needs some reqriting. maybe... later
		match response
		{
			Err(e) => return Err(error::Error::Http),
			Ok(mut res) => {
				let mut json_string = String::new();
				res.read_to_string(&mut json_string);

				// println!("{}", json_string);
				//
				// let value: Value = serde_json::from_str(&json_string).unwrap();
				// println!("{:#?}", value);


				match serde_json::from_str::<ApiResponse<T::ResponseType>>(
					&json_string,
				)
				{
					Err(e) => return Err(error::Error::Json(e)),
					Ok(r) => {
						if r.error != 0 {
							Err(
								error::Error::Api(
									error::api::Api::new(r.code.unwrap(), r.message),
								)
							)
						}
						else {
							Ok(r.data.unwrap())
						}
					}
				}
			}
		}
	}
}


/// responses are parsed into ApiRequest
#[derive(Debug, Clone, Deserialize)]
pub struct ApiResponse<T> {
	/// api error
	pub error: i64,
	/// api message
	pub message: String,
	/// actual data
	pub data: Option<T>,
	/// optional error code
	/// in case of an error this contains the error code
	pub code: Option<i64>,
}
