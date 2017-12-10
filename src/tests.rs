use super::*;
use std::env;


const ENV_KEY: &str = "PROXER_API_KEY";


#[test]
fn fetch_object() {
	// is the api structured as we want it to be?


	let api_key = match env::var_os(ENV_KEY) {
		Some(r) => r,
		None => panic!("no environment variable '{}' found", ENV_KEY)
	};

	let client = Client::new(api_key.into_string().unwrap());

	let res = client.api().info().get_fullentry(parameter::InfoGetFullEntry {
		id: 53
	}).send();


	println!("{:#?}", res);


	panic!();
}




fn mk_client() -> Client {
	let api_key = match env::var_os(ENV_KEY) {
		Some(r) => r,
		None => panic!("no environment variable '{}' found", ENV_KEY)
	};

	Client::new(api_key.into_string().unwrap())
}
