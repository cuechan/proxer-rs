pub mod info;
pub mod user;
pub mod list;

use Endpoint;
use client::Client;
use parameter;


/// Shortcuts to the endpoints

impl Client {
	pub fn api(&self) -> Api
	{
		Api { client: self.clone() }
	}
}


pub struct Api {
	client: Client,
}
