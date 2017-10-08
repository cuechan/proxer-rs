pub mod info;
pub mod user;
pub mod list;


use client::Client;
use request::parameter as p;

/// Shortcuts to the endpoints

impl Client {
	pub fn api(self) -> Api
	{
		Api { inner: self }
	}
}


pub struct Api {
	inner: Client,
}


impl Api {
	pub fn info(self) -> Info
	{
		Info { inner: self.inner.clone() }
	}

	pub fn user(self) -> User
	{
		User { inner: self.inner.clone() }
    }

	pub fn list(self) -> List
	{
		List { inner: self.inner.clone() }
	}
}


pub struct Info {
	inner: Client,
}

pub struct User {
	inner: Client,
}

pub struct List {
	inner: Client,
}


impl Info {
    pub fn get_fullentry(self, vars: p::info::GetFullEntry) -> info::GetFullEntry {
        info::GetFullEntry::new(&self.inner.clone(), vars)
    }

    pub fn get_comments(self, vars: p::info::GetComments) -> info::GetComments {
        info::GetComments::new(&self.inner.clone(), vars)
    }
}
