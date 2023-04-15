#![allow(non_snake_case)]

//		Packages

use axum::http::Uri;
use ring::hmac;
use std::collections::HashMap;
use serde::Deserialize;
use tera::Tera;
use url::form_urlencoded;



//		Structs

//		Config																	
#[derive(Deserialize)]
pub struct Config {
	pub port:   u16,
	pub logdir: String,
	pub users:  HashMap<String, String>,
}

//		AppState																
#[allow(dead_code)]
pub struct AppState {
	pub Config:   Config,
	pub Secret:   [u8; 64],
	pub Key:      hmac::Key,
	pub Template: Tera,
}



//		Functions

//		get_index																
pub fn extract_uri_query_parts(uri: Uri) -> HashMap<String, String> {
	uri
		.query()
		.map(|v| {
			form_urlencoded::parse(v.as_bytes())
				.into_owned()
				.collect()
		})
		.unwrap_or_else(HashMap::new)
}

//		get_index																
pub fn build_uri(path: String, params: HashMap<String, String>) -> Uri {
	Uri::builder()
		.path_and_query(format!("{}?{}",
			path,
			params
				.iter()
				.map(|(k, v)| format!("{}={}", k, v))
				.collect::<Vec<String>>()
				.join("&")
		))
		.build()
		.unwrap()
}


