// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod redis_service;

use redis_service::get_all_keys;

#[tauri::command]
fn fetch_redis_keys() -> Result<Vec<String>, String> {
	match get_all_keys() {
		Ok(keys) => Ok(keys),
		Err(err) => Err(format!("Failed to fetch keys: {}", err)),
	}
}

#[tauri::command]
fn fetch_redis_key_info(key: &str) -> Result<redis_service::RedisKeyInfo, String> {
	match redis_service::get_key_info(key) {
		Ok(key_info) => Ok(key_info),
		Err(err) => Err(format!("Failed to fetch key info: {}", err)),
	}
}

fn main() {
	tauri::Builder::default()
		.invoke_handler(tauri::generate_handler![fetch_redis_keys, fetch_redis_key_info])
		.run(tauri::generate_context!())
		.expect("error while running tauri application");
}
