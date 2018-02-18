use std;

#[derive(Debug, Clone, Copy)]
pub struct UserID {
	inner: i64,
}

// isizes

impl From<isize> for UserID {
	fn from(x: isize) -> Self {
		UserID { inner: x as i64 }
	}
}

impl From<i32> for UserID {
	fn from(x: i32) -> Self {
		UserID { inner: x as i64 }
	}
}

impl From<i64> for UserID {
	fn from(x: i64) -> Self {
		UserID { inner: x as i64 }
	}
}

// usizes

impl From<usize> for UserID {
	fn from(x: usize) -> Self {
		UserID { inner: x as i64 }
	}
}

impl From<u32> for UserID {
	fn from(x: u32) -> Self {
		UserID { inner: x as i64 }
	}
}

impl From<u64> for UserID {
	fn from(x: u64) -> Self {
		UserID { inner: x as i64 }
	}
}

// Strings

impl ToString for UserID {
	fn to_string(&self) -> String {
		self.inner.to_string()
	}
}

#[derive(Debug, Clone)]
pub struct InfoID {
	inner: i64,
}

impl From<isize> for InfoID {
	fn from(x: isize) -> Self {
		InfoID { inner: x as i64 }
	}
}

impl From<i32> for InfoID {
	fn from(x: i32) -> Self {
		InfoID { inner: x as i64 }
	}
}

impl From<i64> for InfoID {
	fn from(x: i64) -> Self {
		InfoID { inner: x as i64 }
	}
}

impl From<usize> for InfoID {
	fn from(x: usize) -> Self {
		InfoID { inner: x as i64 }
	}
}

impl From<u32> for InfoID {
	fn from(x: u32) -> Self {
		InfoID { inner: x as i64 }
	}
}

impl From<u64> for InfoID {
	fn from(x: u64) -> Self {
		InfoID { inner: x as i64 }
	}
}

impl ToString for InfoID {
	fn to_string(&self) -> String {
		self.inner.to_string()
	}
}

impl std::str::FromStr for InfoID {
	type Err = std::num::ParseIntError;

	fn from_str(string: &str) -> Result<InfoID, Self::Err> {
		match string.parse::<i64>() {
			Ok(i) => Ok(InfoID::from(i)),
			Err(e) => Err(e),
		}
	}
}
