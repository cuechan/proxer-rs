use super::*;


#[test]
fn api_structure() {
	// is the api structured as we want it to be?

	let client = Client::new("my_api_key".to_string());



	let response = client.api().info().get_fullentry();


	println!("request:    {:#?}",  response);

	panic!();
}
