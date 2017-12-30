#[derive(Serialize, Debug, Clone)]
pub struct GetList {
	pub kat: Option<String>,
	pub medium: Option<String>,
	pub is_h: Option<String>,
	pub start: Option<String>,
	pub sort: Option<String>,
	pub sort_type: Option<String>,
	pub p: Option<i64>,
	pub limit: Option<i64>,
}


impl Iterator for GetList {
	type Item = Self;

	fn next(&mut self) -> Option<Self::Item>
	{
		self.p = match self.p
		{
			Some(p) => Some(p + 1),
			None => Some(0),
		};

		Some(self.clone())
	}
}
