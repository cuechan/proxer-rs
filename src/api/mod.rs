pub mod info;
pub mod user;

use client::Client;


pub const DEFAULT_PAGER_PAGE: i64 = 0;
pub const DEFAULT_PAGER_LIMIT: i64 = 250;



#[derive(Debug, Clone)]
pub struct Api {
	pub client: Client,
}


impl Api {
	pub fn info(self) -> info::Info
	{
		info::Info { client: self.client }
	}

	pub fn user(self) -> user::User
	{
		user::User { client: self.client }
	}
}


impl Client {
	pub fn api(&self) -> Api
	{
		Api { client: self.clone() }
	}
}
