#![allow(dead_code, unused_imports)]

use chrono::NaiveDateTime;
use response::Kat;
use response::stringly_array_spaces;
use response::stringly_int;
use response::stringly_timestamp_unix;
use response::stringly_timestamp_weird;
use serde_json::Value;
use std::env;
use super::*;


const ENV_KEY: &str = "PROXER_API_KEY";


#[test]
fn api_response()
{
	// is the api structured as we want it to be?
	info!("creating client");
	let mut client = match Client::with_env_key(ENV_KEY) {
		None => return,
		Some(client) => client
	};


	info!("build request");
	let req = api::info::GetFullEntry { id: 53 };


	info!("send request");
	let res = client.execute(req);
	info!("request sent");



	match res
	{
		Err(e) => {
			error!("error...");
			eprintln!("response {:#?}", e);

			match e
			{
				error::Error::Api(e) => {
					info!("api error");

					match e.error()
					{
						error::api::Errcode::NoApiPermissions => return,
						_ => panic!("unexpected api error: {}", e),
					}
				}
				error::Error::Json(e) => panic!("can't parse json: {}", e),
				error::Error::Unknown => panic!("unknown error"),
				_ => return,
			}
		}

		Ok(r) => {
			info!("Ok");
			assert_eq!(r.medium, "animeseries");
		}
	}
}





#[test]
fn stringly_timestamps() {
	use serde_json;


	let json = r#"
		{
			"timestamp": "2016-06-20 01:34:29",
			"unix": "1466386469"
		}
	"#;


	#[derive(Deserialize)]
	struct TimeTest {
		#[serde(deserialize_with = "stringly_timestamp_weird")]
		pub timestamp: NaiveDateTime,
		#[serde(deserialize_with = "stringly_timestamp_unix")]
		pub unix: NaiveDateTime
	};


	let time: TimeTest = serde_json::from_str(json).unwrap();


	assert_eq!(time.timestamp, time.unix);
}
