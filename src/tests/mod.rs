#![allow(dead_code, unused_imports)]
use super::*;
use std::env;


const ENV_KEY: &str = "PROXER_API_KEY";


#[test]
fn api_response()
{
	// is the api structured as we want it to be?
	let client = Client::with_env_key(ENV_KEY)
		.expect("can't read api-key from environment");


	let req = client
		.api()
		.info()
		.get_fullentry(parameter::InfoGetFullEntry { id: 53 });


	let res = client.execute(req);

	eprintln!();


	match res
	{
		Err(e) => {
			match e
			{
				error::Error::Api(e) => {
					match e.error()
					{
						error::api::Errcode::NoApiPermissions => return,
						_ => panic!("this error is not expected: {}", e),
					}
				}
				error::Error::Json(e) => panic!("can't parse json: {}", e),
				error::Error::Unknown => panic!("unknown error"),
				_ => return,
			}
		}

		Ok(r) => {
			assert_eq!(r.medium, "animeseries");
		}
	}
}
