use redis::{Commands, RedisError};
use serde::Serialize;

pub fn get_all_keys() -> Result<Vec<String>, redis::RedisError> {
	let client = redis::Client::open("redis://127.0.0.1:6379")?;
	let mut connection = client.get_connection()?;

	let keys: Vec<String> = connection.keys("*")?;
	Ok(keys)
}

#[derive(Serialize)]
pub struct RedisKeyInfo {
	pub key: String,
	pub ttl: Option<i64>,
	pub value: Option<String>,
}

pub fn get_key_info(key: &str) -> Result<RedisKeyInfo, RedisError> {
	let client = redis::Client::open("redis://127.0.0.1:6379")?;
	let mut connection = client.get_connection()?;

	let ttl: i64 = connection.ttl(key)?;
	let value: String = connection.get(key)?;

	Ok(RedisKeyInfo {
		key: key.to_string(),
		ttl: if ttl >= 0 { Some(ttl) } else { None },
		value: if value.is_empty() { None } else { Some(value) },
	})
}
