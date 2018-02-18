use chrono::DateTime;
use chrono::FixedOffset;
use chrono::NaiveDateTime;
use response::Timestamp;
use response::parse_timestamp;
use serde_json;

#[derive(Debug, Clone, Deserialize)]
struct TimeTest {
	#[serde(deserialize_with = "parse_timestamp")]
	pub timestamp: Timestamp,
}

#[test]
fn parse_php_stringly() {
	let json = r#" { "timestamp": "2016-06-20 01:34:29" } "#;

	let time: TimeTest = serde_json::from_str(json).unwrap();
	assert_eq!(time.timestamp, test_time());
}

#[test]
fn parse_unix() {
	let json = r#"{"timestamp": 1466386469}"#;

	let time: TimeTest = serde_json::from_str(json).unwrap();
	assert_eq!(time.timestamp, test_time());
}

#[test]
fn parse_unix_stringly() {
	let json = r#" { "timestamp": "1466386469" } "#;

	let time: TimeTest = serde_json::from_str(json).unwrap();
	assert_eq!(time.timestamp, test_time());
}

fn test_time() -> DateTime<FixedOffset> {
	let time = NaiveDateTime::from_timestamp(1466386469, 0);
	DateTime::from_utc(time, FixedOffset::east(3600))
}
