use client::Client;
use std::collections::HashMap;
use request::parameter;



pub struct User {
	pub client: Client,
}




pub struct Userinfo {
	data: HashMap<String, String>,
}

impl Userinfo {
	fn new(vars: parameter::user::Userinfo) -> Self
	{
		let mut data = HashMap::new();

		match (vars.uid, vars.username)
		{
			(Some(i), None) => data.insert("uid".to_string(), i.to_string()),
			(None, Some(i)) => data.insert("username".to_string(), i.to_string()),
			_ => panic!("either username nor uid are given"),
		};

		Self { data: data }
	}
}
