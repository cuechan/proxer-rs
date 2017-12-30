extern crate proxer;
extern crate pretty_env_logger;

use proxer::Client;
use proxer::Endpoint;
use proxer::PageableEndpoint;
use proxer::api;
use std::ops::Add;


fn main()
{
	pretty_env_logger::init().unwrap();

	get_comments();
	get_fullentry();
}










fn get_fullentry()
{
	let prxr = Client::with_env_key("PROXER_API_KEY").unwrap();



	let req = api::info::GetFullEntry { id: 53 };


	let res = prxr.execute(req).unwrap();

	eprintln!("medium: {}", res.medium);
}













fn get_comments()
{
	let prxr = Client::with_env_key("PROXER_API_KEY").unwrap();

	let req = api::info::GetComments {
		id: 53,
		p: None,
		limit: Some(100),
		sort: None,
	};


	let pager = req.pager(prxr);

	let mut counter = 0;

	for comment in pager {
		counter = counter.add(1);
		let comment = comment.unwrap();

		println!("{:5}: {}", counter, comment.username);
	}
}
