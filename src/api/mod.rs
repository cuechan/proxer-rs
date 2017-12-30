use Client;

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


#[allow(dead_code)]
pub struct Api {
	client: Client,
}



// Todo: move dat shit away
