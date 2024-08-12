use redis::Commands;

pub fn get_all_keys() -> Result<Vec<String>, redis::RedisError> {
	let client = redis::Client::open("redis://127.0.0.1:6379")?;
	let mut connection = client.get_connection()?;

	let keys: Vec<String> = connection.keys("*")?;
	Ok(keys)
}
