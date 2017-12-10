use super::*;
use std::env;
use error::Error;
use error::api::Errcode;


const ENV_KEY: &str = "PROXER_API_KEY";


#[test]
fn api_response() {
	// is the api structured as we want it to be?
	let client = mk_client();

	let res = client.api().info().get_fullentry(parameter::InfoGetFullEntry {
		id: 53
	}).send();

	println!();


	match res {
		Err(e) => {
			match e {
				Error::Api(e) => {
					match e.error() {
						Errcode::NoApiPermissions => return,
						_ => panic!("this error is not expected"),
					}
				},
				Error::Json => panic!("can't parse json"),
				Error::Unknown => panic!("unknown error"),
				_ => return,
			}
		},

		Ok(r) => {
			assert_eq!(r.medium, "animeseries");
		}
	}
}




fn mk_client() -> Client {
	let api_key = match env::var_os(ENV_KEY) {
		Some(r) => r,
		// using a dummy key
		None => "DUMMY".into()
	};

	Client::new(api_key.into_string().unwrap())
}
