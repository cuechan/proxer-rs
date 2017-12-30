use Endpoint;
use PageableEndpoint;
use response;
use Client;
use Pager;

pub mod info;
pub mod user;
pub mod list;




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



// Todo: move dat shit away
