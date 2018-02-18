use Endpoint;
use error;
use PageableEndpoint;
use Pager;
use reqwest;
use reqwest::StatusCode;
use reqwest::header;
use serde_json;
use serde_urlencoded;
use std;
use std::env;
use std::fmt;
use std::io::Read;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::time;

const API_BASE_PATH: &str = "http://proxer.me/api/v1/";

#[derive(Debug, Clone)]
pub struct ClientConfig {
	pub cooldown: time::Duration,
}

impl Default for ClientConfig {
	/// Default ClientConfig:
	/// - No cooldown

	fn default() -> Self {
		Self {
			cooldown: time::Duration::new(0, 0),
		}
	}
}

/// Client that holds the api key and sends requests
#[derive(Debug, Clone)]
pub struct Client {
	api_key: String,
	base_uri: String,
	user_agent: String,
	config: ClientConfig,
	last_request: Arc<Mutex<Option<time::Instant>>>,
}

#[allow(unused)]
impl Client {
	/// Create a new api client with a given api key
	pub fn new(api_key: String) -> Self {
		let crates_name = env::var("CARGO_PKG_NAME").unwrap_or("unknown".to_string());

		let crates_version = &env::var("CARGO_PKG_VERSION").unwrap_or("unknown".to_string());
		let ua = format!("libproxer-rust({}/v{})", crates_name, crates_version);

		Self {
			api_key: api_key,
			base_uri: API_BASE_PATH.to_string(),
			user_agent: ua,
			config: ClientConfig::default(),
			last_request: Arc::new(Mutex::new(None)),
		}
	}

	/// load api key from an environment variable
	pub fn with_env_key(env_key: &str) -> Option<Client> {
		match env::var_os(env_key) {
			Some(r) => Some(Client::new(r.into_string().unwrap())),
			None => None,
		}
	}

	pub fn cooldown_time(&mut self, time: time::Duration) {
		self.config.cooldown = time;
	}

	/// execute a request that satisfies [`Endpoint`](../trait.Endpoint.html)
	pub fn execute<T>(&mut self, mut endpoint: T) -> Result<T::ResponseType, error::Error>
	where
		T: super::Endpoint + Clone + fmt::Debug,
	{
		let uristring = self.base_uri.to_string() + T::URL;

		header!{(ProxerApiKey, "Proxer-Api-Key") => [String]}

		let mut headers = reqwest::header::Headers::new();

		headers.set(header::UserAgent::new(self.user_agent.to_string()));
		headers.set(header::Host::new("proxer.me", None));
		headers.set(header::ContentType::form_url_encoded());
		headers.set(ProxerApiKey(self.api_key.clone()));

		let mut http_req = reqwest::Request::new(reqwest::Method::Post, uristring.parse().unwrap());

		// add our headers to the request
		http_req.headers_mut().extend(headers.iter());

		match serde_urlencoded::to_string(endpoint.clone()) {
			Ok(body) => {
				debug!("post data: {}", body);
				*http_req.body_mut() = Some(body.into());
			}
			Err(err) => panic!("can't serialize form parameters"),
		}

		debug!("creating reqwest client");
		let client = reqwest::Client::new();

		self.waiting_for_cooldown();
		debug!("sending request");

		let response = client.execute(http_req);
		debug!("response received");
		self.reset_cooldown();

		// This section needs some rewriting. maybe... later
		match response {
			Err(e) => return Err(error::Error::Unknown),
			Ok(mut res) => {
				debug!("http response code: {}", res.status());


				if res.status() != StatusCode::Ok {
					return Err(error::Error::Http)
				}


				let mut json_string = String::new();
				res.read_to_string(&mut json_string);

				debug!("http response length: {}kb", json_string.len() * 1000);

				match serde_json::from_str::<ApiResponse<T::ResponseType>>(&json_string) {
					Err(e) => return Err(error::Error::Json(e)),
					Ok(r) => {
						if r.error != 0 {
							Err(error::Error::Api(error::api::Api::new(
								r.code.unwrap(),
								r.message,
							)))
						}
						else {
							Ok(r.data.unwrap())
						}
					}
				}
			}
		}
	}

	fn remaining_cooldown_time(&self) -> time::Duration {
		match *self.last_request.lock().unwrap() {
			None => self.config.cooldown,
			Some(time) => time.elapsed().clone(),
		}
	}

	fn reset_cooldown(&mut self) {
		*self.last_request.lock().unwrap() = Some(time::Instant::now());
	}

	// vlocking till we can create a new request
	fn waiting_for_cooldown(&mut self) {
		let conf_cooldown = self.config.cooldown;
		let since_last = self.remaining_cooldown_time();

		debug!(
			"configurated cooldown {:2}.{}s",
			conf_cooldown.as_secs(),
			conf_cooldown.subsec_nanos()
		);
		debug!(
			"last request {:2}.{}s ago",
			since_last.as_secs(),
			since_last.subsec_nanos()
		);

		if since_last >= conf_cooldown {
			return;
		}

		let wait = self.config.cooldown - self.remaining_cooldown_time();
		debug!(
			"waiting for cooldown: {:2}.{}s",
			wait.as_secs(),
			wait.subsec_nanos()
		);

		thread::sleep(wait);
	}

	pub fn pager<T>(self, mut endpoint: T) -> Pager<T>
	where
		T: Endpoint + PageableEndpoint + Clone + fmt::Debug,
		<T as Endpoint>::ResponseType: IntoIterator + Clone + fmt::Debug,
		<<T as Endpoint>::ResponseType as std::iter::IntoIterator>::Item: Clone + fmt::Debug,
	{
		Pager::new(self, endpoint, None, None)
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
