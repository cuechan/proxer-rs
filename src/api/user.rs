use Endpoint;
use PageableEndpoint;
use Pager;
use client::Client;
use response;




#[derive(Serialize, Debug, Clone)]
pub struct Login {
	pub username: String,
	pub password: String,
	pub secretkey: Option<String>,
}



impl Endpoint for Login {
	type ResponseType = response::user::Login;
	#[doc(hidden)]
	const URL: &'static str = "user/login";
}

impl Login {
	/// shortcut
	pub fn login(user: String, pswd: String) -> Self {
		Self {
			username: user,
			password: pswd,
			secretkey: None,
		}
	}
}



#[derive(Serialize, Debug, Clone)]
pub struct Logout {}


impl Endpoint for Logout {
	type ResponseType = response::user::Logout;
	#[doc(hidden)]
	const URL: &'static str = "user/logout";
}







#[derive(Serialize, Debug, Clone)]
pub struct Userinfo {
	pub uid: Option<usize>,
	pub username: Option<String>,
}


impl Userinfo {
	/// shortcut with uid
	pub fn uid(uid: i64) -> Self {
		Self {
			uid: Some(uid as usize),
			username: None
		}
	}

	/// shortcut with username
	pub fn user(username: String) -> Self {
		Self {
			uid: None,
			username: Some(username)
		}
	}
}


impl Endpoint for Userinfo {
	type ResponseType = response::user::Userinfo;
	#[doc(hidden)]
	const URL: &'static str = "user/userinfo";
}



#[derive(Serialize, Debug, Clone)]
pub struct GetTopten {
	pub uid: Option<usize>,
	pub username: Option<String>,
	pub kat: Option<String>,
	pub is_h: Option<bool>,
}


impl Endpoint for GetTopten {
	type ResponseType = response::user::TopTen;
	#[doc(hidden)]
	const URL: &'static str = "user/topten";
}



impl GetTopten {
	pub fn user(user: String) -> Self {
		Self {
			uid: None,
			username: Some(user),
			kat: None,
			is_h: None
		}
	}
}






#[derive(Default, Serialize, Debug, Clone)]
pub struct GetList {
	pub uid: Option<usize>,
	pub username: Option<String>,
	pub kat: Option<String>,
	pub search: Option<String>,
	pub search_start: Option<String>,
	pub is_h: Option<bool>,
	pub filter: Option<u64>,
	pub sort: Option<String>,

	pub p: Option<usize>,
	pub limit: Option<usize>,
}


impl GetList {
	pub fn with_uid(uid: i64) -> Self {
		Self {
			uid: Some(uid as usize),
			..Default::default()
		}
	}


	pub fn with_username<T: ToString>(username: T) -> Self {
		Self {
			username: Some(username.to_string()),
			..Default::default()
		}
	}
}



impl Endpoint for GetList {
	type ResponseType = Vec<response::user::GetList>;
	#[doc(hidden)]
	const URL: &'static str = "user/list";
}


/// Pager for `GetList`
impl PageableEndpoint for GetList {
	fn pager(self, client: Client) -> Pager<Self> {
		Pager::new(client, self, Some(0), Some(250))
	}

	fn page_mut(&mut self) ->&mut Option<usize> {
		&mut self.p
	}

	fn limit_mut(&mut self) ->&mut Option<usize> {
		&mut self.limit
	}
}



#[derive(Serialize, Debug, Clone)]
pub struct GetLatestComments {
	pub uid: Option<usize>,
	pub username: Option<String>,
	pub kat: Option<String>,
	pub length: Option<u64>,

	pub p: Option<u64>,
	pub limit: Option<u64>,
}



#[derive(Serialize, Debug, Clone)]
pub struct GetHistory {
	pub uid: Option<usize>,
	pub username: Option<String>,
	pub is_h: Option<bool>,
	pub filter: Option<u64>,

	pub p: Option<u64>,
	pub limit: Option<u64>,
}
