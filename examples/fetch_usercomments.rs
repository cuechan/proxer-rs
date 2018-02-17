extern crate proxer;
use proxer::api::user;
use proxer::Client;

fn main() {
	// a very simple straightforward request:
	let req = user::GetList::with_username("genesis");

	// create a client and a pager.
	// The pager automatically iterates over the response
	// and fetches new data if needed
	let pager = proxer::Client::with_env_key("PROXER_API_KEY")
		.unwrap()
		.pager(req);


	// Thanks to Rust's syntax sugar we can use an ordinary
	// `for each` loop to make the request
	for entry in pager {
		// the object we are getting here is a normal api response
		// before doing anything we need to check for errors and unpack the
		// real data
		match entry {
			Err(e) => println!("error: {}", e),
			Ok(r) => println!("{:#?}", r),
		}
	}

}
