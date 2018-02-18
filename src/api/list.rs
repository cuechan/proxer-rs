use Client;
use Endpoint;
use PageableEndpoint;
use Pager;
use response::list as response;

#[derive(Serialize, Debug, Clone)]
pub struct GetEntryList {
	pub kat: Option<String>,
	pub medium: Option<String>,
	pub is_h: Option<String>,
	pub start: Option<String>,
	pub sort: Option<String>,
	pub sort_type: Option<String>,
	pub p: Option<usize>,
	pub limit: Option<usize>,
}

impl Endpoint for GetEntryList {
	type ResponseType = Vec<response::EntryList>;
	#[doc(hidden)]
	const URL: &'static str = "list/entrylist";
}

impl PageableEndpoint for GetEntryList {
	/// Default pager:
	///
	/// ```ignore
	/// p     = 0
	/// limit = 3500
	/// ```

	fn pager(self, client: Client) -> Pager<Self> {
		debug!("new pager with data: {:?}", self);
		Pager::new(client, self, Some(0), Some(3500))
	}

	fn page_mut(&mut self) -> &mut Option<usize> {
		&mut self.p
	}

	fn limit_mut(&mut self) -> &mut Option<usize> {
		&mut self.limit
	}
}

impl GetEntryList {
	/// Creates an instance with everything `None`
	pub fn with_default() -> Self {
		Self {
			is_h: None,
			kat: None,
			medium: None,
			limit: None,
			p: None,
			sort_type: None,
			sort: None,
			start: None,
		}
	}
}
