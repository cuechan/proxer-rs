extern crate proxer;
extern crate pretty_env_logger;

use proxer::Client;
use proxer::Endpoint;
use proxer::PageableEndpoint;
use proxer::Pager;
use proxer::parameter;
use std::error::Error;
use std::ops::Add;


fn main()
{
	pretty_env_logger::init().unwrap();

	get_comments();
}




fn get_fullentry()
{
	let prxr = Client::with_env_key("PROXER_API_KEY").unwrap();


	let req = prxr.api()
		.info()
		.get_fullentry(parameter::InfoGetFullEntry { id: 53 });


	let res = prxr.execute(req);

	eprintln!("{:#?}", res);
}













fn get_comments()
{
	let prxr = Client::with_env_key("PROXER_API_KEY").unwrap();

	let req = prxr.api()
		.info()
		.get_comments(
			parameter::InfoGetComments {
				id: 53,
				p: None,
				limit: Some(100),
				sort: None,
			}
		);


	let pager = req.pager(prxr);

	let mut counter = 0;

	for comment in pager {
		counter = counter.add(1);
		let comment = comment.unwrap();

		println!("{:5}: {}", counter, comment.username);
	}
}













fn get_comments() {
    let prxr = Client::with_env_key("PROXER_API_KEY").unwrap();

    let req = prxr.api().info().get_comments(parameter::InfoGetComments {
        id: 53,
        p: None,
        limit: None,
        sort: None
    });


    let pager = req.pager(prxr);

    let mut counter = 0;

    for comment in pager {
        counter = counter.add(1);
        let comment = comment.unwrap();

        println!("{}", counter);
        println!("{}", comment.username);
        println!("{}", comment.episode);
        println!("{}", comment.rating);

        println!();

    }
}
